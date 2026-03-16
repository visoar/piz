use anyhow::Result;

use crate::context::SystemContext;
use crate::i18n;
use crate::llm::prompt::build_explain_prompt;
use crate::llm::LlmBackend;
use crate::ui;

pub async fn explain_command(
    backend: &dyn LlmBackend,
    ctx: &SystemContext,
    command: &str,
    tr: &i18n::T,
    lang: &str,
) -> Result<()> {
    let (system, user) = build_explain_prompt(ctx, command, lang);

    let spinner = ui::create_spinner(tr.analyzing);
    let response = backend.chat(&system, &user).await?;
    spinner.finish_and_clear();

    ui::print_explanation(tr, &response);
    Ok(())
}
