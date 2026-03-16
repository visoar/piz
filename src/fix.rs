use anyhow::Result;

use crate::context::SystemContext;
use crate::danger::{self, DangerLevel};
use crate::executor::{self, UserChoice};
use crate::history;
use crate::i18n;
use crate::llm::prompt::build_fix_prompt;
use crate::llm::LlmBackend;
use crate::ui;

pub async fn fix_last_command(
    backend: &dyn LlmBackend,
    ctx: &SystemContext,
    tr: &i18n::T,
    lang: &str,
) -> Result<()> {
    let (command, exit_code, stderr) = match executor::load_last_exec() {
        Ok(last) => {
            if last.exit_code == 0 {
                ui::print_info(tr.last_succeeded);
                return Ok(());
            }
            (last.command, last.exit_code, last.stderr)
        }
        Err(_) => {
            ui::print_info(tr.no_piz_record);
            let cmd = history::last_history_command()?;
            ui::print_info(&format!("{} {}", tr.last_from_history, cmd));
            (cmd, 1, String::new())
        }
    };

    println!();
    ui::print_info(&format!("{} {}", tr.failed_command, command));
    if !stderr.is_empty() {
        println!(
            "  {}",
            stderr.lines().take(5).collect::<Vec<_>>().join("\n  ")
        );
    }
    println!();

    let (system, user) = build_fix_prompt(ctx, &command, exit_code, &stderr, lang);

    let spinner = ui::create_spinner(tr.analyzing);
    let response = backend.chat(&system, &user).await?;
    spinner.finish_and_clear();

    let (diagnosis, fixed_cmd, llm_danger) = parse_fix_response(&response)?;

    ui::print_diagnosis(tr, &diagnosis);
    println!();

    let regex_danger = danger::detect_danger_regex(&fixed_cmd);
    let final_danger = regex_danger.max(llm_danger);

    let choice = executor::prompt_user(&fixed_cmd, final_danger, false, tr)?;
    match choice {
        UserChoice::Execute => {
            executor::execute_command(&fixed_cmd, tr)?;
        }
        UserChoice::Edit(edited) => {
            executor::execute_command(&edited, tr)?;
        }
        UserChoice::Cancel => {
            ui::print_info(tr.cancelled);
        }
    }

    Ok(())
}

pub fn parse_fix_response(response: &str) -> Result<(String, String, DangerLevel)> {
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(response) {
        return extract_fix_json(&v);
    }

    if let Some(json_str) = crate::extract_json_block(response) {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(json_str) {
            return extract_fix_json(&v);
        }
    }

    anyhow::bail!("Could not parse fix response from LLM:\n{}", response)
}

fn extract_fix_json(v: &serde_json::Value) -> Result<(String, String, DangerLevel)> {
    let diagnosis = v["diagnosis"]
        .as_str()
        .unwrap_or("Unknown issue")
        .to_string();
    let command = v["command"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("No 'command' field in fix response"))?
        .to_string();
    let danger = v["danger"]
        .as_str()
        .map(DangerLevel::from_str_level)
        .unwrap_or(DangerLevel::Safe);
    Ok((diagnosis, command, danger))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_fix_direct_json() {
        let input = r#"{"diagnosis": "Permission denied", "command": "sudo npm install", "danger": "warning"}"#;
        let (diag, cmd, danger) = parse_fix_response(input).unwrap();
        assert_eq!(diag, "Permission denied");
        assert_eq!(cmd, "sudo npm install");
        assert_eq!(danger, DangerLevel::Warning);
    }

    #[test]
    fn parse_fix_json_in_text() {
        let input = r#"The fix is: {"diagnosis": "wrong path", "command": "cd /correct/path && npm start", "danger": "safe"} Try this."#;
        let (diag, cmd, danger) = parse_fix_response(input).unwrap();
        assert_eq!(diag, "wrong path");
        assert!(cmd.contains("cd /correct/path"));
        assert_eq!(danger, DangerLevel::Safe);
    }

    #[test]
    fn parse_fix_missing_diagnosis_defaults() {
        let input = r#"{"command": "npm install"}"#;
        let (diag, cmd, _) = parse_fix_response(input).unwrap();
        assert_eq!(diag, "Unknown issue");
        assert_eq!(cmd, "npm install");
    }

    #[test]
    fn parse_fix_missing_danger_defaults_safe() {
        let input = r#"{"diagnosis": "typo", "command": "git push"}"#;
        let (_, _, danger) = parse_fix_response(input).unwrap();
        assert_eq!(danger, DangerLevel::Safe);
    }

    #[test]
    fn parse_fix_no_command_field_errors() {
        let input = r#"{"diagnosis": "something wrong"}"#;
        assert!(parse_fix_response(input).is_err());
    }

    #[test]
    fn parse_fix_invalid_text_errors() {
        let input = "I'm not sure what went wrong.";
        assert!(parse_fix_response(input).is_err());
    }

    #[test]
    fn parse_fix_dangerous_command() {
        let input =
            r#"{"diagnosis": "need root", "command": "rm -rf /tmp/*", "danger": "dangerous"}"#;
        let (_, _, danger) = parse_fix_response(input).unwrap();
        assert_eq!(danger, DangerLevel::Dangerous);
    }
}
