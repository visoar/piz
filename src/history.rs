use anyhow::Result;

/// Read the last command from shell history as a fallback for `piz fix`
pub fn last_history_command() -> Result<String> {
    let home =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot determine home directory"))?;

    // Try zsh history first, then bash
    let candidates = [home.join(".zsh_history"), home.join(".bash_history")];

    for path in &candidates {
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            if let Some(last_line) = content.lines().rev().find(|l| !l.trim().is_empty()) {
                // zsh history format: ": timestamp:0;command"
                let cmd = if last_line.starts_with(':') {
                    last_line.split_once(';').map_or(last_line, |x| x.1)
                } else {
                    last_line
                };
                return Ok(cmd.trim().to_string());
            }
        }
    }

    // Windows: try PSReadLine history
    if cfg!(target_os = "windows") {
        if let Some(appdata) = dirs::data_local_dir() {
            let ps_history = appdata
                .join("Microsoft")
                .join("Windows")
                .join("PowerShell")
                .join("PSReadLine")
                .join("ConsoleHost_history.txt");
            if ps_history.exists() {
                let content = std::fs::read_to_string(ps_history)?;
                if let Some(last) = content.lines().rev().find(|l| !l.trim().is_empty()) {
                    return Ok(last.trim().to_string());
                }
            }
        }
    }

    anyhow::bail!("Could not find shell history")
}
