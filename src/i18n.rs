/// Supported display languages
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Lang {
    Zh,
    En,
    Ja,
}

impl Lang {
    pub fn from_code(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "en" => Lang::En,
            "ja" => Lang::Ja,
            _ => Lang::Zh,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Lang::Zh => "zh",
            Lang::En => "en",
            Lang::Ja => "ja",
        }
    }
}

/// All translatable UI strings
pub struct T {
    // ── ui ──
    pub cached: &'static str,
    pub command_explanation: &'static str,
    pub diagnosis: &'static str,
    pub cancelled: &'static str,
    pub thinking: &'static str,
    pub analyzing: &'static str,

    // ── danger warnings ──
    pub danger_warning: &'static str,
    pub modify_warning: &'static str,

    // ── executor prompts ──
    pub confirm_dangerous: &'static str,
    pub yes_execute: &'static str,
    pub no_cancel: &'static str,
    pub edit_command: &'static str,
    pub execute: &'static str,
    pub cancel: &'static str,
    pub edit: &'static str,
    pub edit_prompt: &'static str,
    pub exit_code: &'static str,

    // ── fix ──
    pub no_piz_record: &'static str,
    pub last_from_history: &'static str,
    pub last_succeeded: &'static str,
    pub failed_command: &'static str,

    // ── config init ──
    pub wizard_title: &'static str,
    pub select_backend: &'static str,
    pub auto_confirm_prompt: &'static str,
    pub extra_backends: &'static str,
    pub config_saved: &'static str,
    pub config_validated: &'static str,
    pub config_edit_hint: &'static str,
    pub config_rerun_hint: &'static str,
    pub config_overwrite: &'static str,
    pub api_key_prompt: &'static str,
    pub model_prompt: &'static str,
    pub select_provider: &'static str,
    pub base_url_prompt: &'static str,
    pub custom_url_prompt: &'static str,
    pub add_openai: &'static str,
    pub add_claude: &'static str,
    pub add_gemini: &'static str,
    pub add_ollama: &'static str,
    pub ollama_host: &'static str,
}

pub fn t(lang: Lang) -> &'static T {
    match lang {
        Lang::Zh => &ZH,
        Lang::En => &EN,
        Lang::Ja => &JA,
    }
}

static ZH: T = T {
    cached: "(缓存命中)",
    command_explanation: "命令解释：",
    diagnosis: "诊断：",
    cancelled: "已取消。",
    thinking: "思考中...",
    analyzing: "分析中...",

    danger_warning: "危险命令！可能导致数据丢失或系统损坏。",
    modify_warning: "该命令会修改文件或系统设置。",

    confirm_dangerous: "⚠ 确定要执行这条危险命令吗？",
    yes_execute: "是，执行（我了解风险）",
    no_cancel: "否，取消",
    edit_command: "编辑命令",
    execute: "[Y] 执行",
    cancel: "[n] 取消",
    edit: "[e] 编辑",
    edit_prompt: "编辑命令",
    exit_code: "退出码",

    no_piz_record: "未找到 piz 执行记录，正在读取 shell 历史...",
    last_from_history: "历史命令：",
    last_succeeded: "上次命令执行成功，无需修复。",
    failed_command: "失败命令：",

    wizard_title: "⚙ piz 配置向导",
    select_backend: "选择默认 LLM 后端",

    auto_confirm_prompt: "安全命令是否自动执行（不弹出确认）？",
    extra_backends: "是否配置其他后端？",
    config_saved: "✔ 配置已保存：",
    config_validated: "✔ 配置验证通过。",
    config_edit_hint: "后续可手动编辑：",
    config_rerun_hint: "或重新运行：piz config --init",
    config_overwrite: "配置已存在，是否覆盖？",
    api_key_prompt: "API 密钥",
    model_prompt: "模型名称",
    select_provider: "选择 API 供应商",
    base_url_prompt: "API 地址",
    custom_url_prompt: "是否使用自定义 API 地址（代理）？",
    add_openai: "添加 OpenAI 兼容后端？",
    add_claude: "添加 Claude 后端？",
    add_gemini: "添加 Gemini 后端？",
    add_ollama: "添加 Ollama 后端？",
    ollama_host: "Ollama 地址",
};

static EN: T = T {
    cached: "(cached)",
    command_explanation: "Command explanation:",
    diagnosis: "Diagnosis:",
    cancelled: "Cancelled.",
    thinking: "Thinking...",
    analyzing: "Analyzing...",

    danger_warning: "DANGEROUS COMMAND! This could cause data loss or system damage.",
    modify_warning: "This command modifies files or system settings.",

    confirm_dangerous: "⚠ Are you SURE you want to execute this?",
    yes_execute: "Yes, execute (I understand the risks)",
    no_cancel: "No, cancel",
    edit_command: "Edit command",
    execute: "[Y] Execute",
    cancel: "[n] Cancel",
    edit: "[e] Edit",
    edit_prompt: "Edit command",
    exit_code: "Exit code",

    no_piz_record: "No piz execution record found, reading shell history...",
    last_from_history: "Last command from history:",
    last_succeeded: "Last command succeeded. Nothing to fix.",
    failed_command: "Failed command:",

    wizard_title: "⚙ piz configuration wizard",
    select_backend: "Select default LLM backend",

    auto_confirm_prompt: "Auto-execute safe commands without confirmation?",
    extra_backends: "Configure additional backends now?",
    config_saved: "✔ Config saved:",
    config_validated: "✔ Config validated successfully.",
    config_edit_hint: "You can edit it later:",
    config_rerun_hint: "Or re-run: piz config --init",
    config_overwrite: "Config already exists. Overwrite?",
    api_key_prompt: "API key",
    model_prompt: "Model name",
    select_provider: "Select API provider",
    base_url_prompt: "API base URL",
    custom_url_prompt: "Use custom API URL (proxy)?",
    add_openai: "Add OpenAI-compatible backend?",
    add_claude: "Add Claude backend?",
    add_gemini: "Add Gemini backend?",
    add_ollama: "Add Ollama backend?",
    ollama_host: "Ollama host",
};

static JA: T = T {
    cached: "(キャッシュ)",
    command_explanation: "コマンド解説：",
    diagnosis: "診断：",
    cancelled: "キャンセルしました。",
    thinking: "考え中...",
    analyzing: "分析中...",

    danger_warning: "危険なコマンドです！データ損失やシステム障害を引き起こす可能性があります。",
    modify_warning: "このコマンドはファイルやシステム設定を変更します。",

    confirm_dangerous: "⚠ 本当にこの危険なコマンドを実行しますか？",
    yes_execute: "はい、実行する（リスクを理解しています）",
    no_cancel: "いいえ、キャンセル",
    edit_command: "コマンドを編集",
    execute: "[Y] 実行",
    cancel: "[n] キャンセル",
    edit: "[e] 編集",
    edit_prompt: "コマンドを編集",
    exit_code: "終了コード",

    no_piz_record: "piz の実行記録が見つかりません。シェル履歴を読み込んでいます...",
    last_from_history: "履歴のコマンド：",
    last_succeeded: "前回のコマンドは成功しました。修正の必要はありません。",
    failed_command: "失敗したコマンド：",

    wizard_title: "⚙ piz 設定ウィザード",
    select_backend: "デフォルトの LLM バックエンドを選択",

    auto_confirm_prompt: "安全なコマンドを確認なしで自動実行しますか？",
    extra_backends: "他のバックエンドも設定しますか？",
    config_saved: "✔ 設定を保存しました：",
    config_validated: "✔ 設定の検証に成功しました。",
    config_edit_hint: "後で編集できます：",
    config_rerun_hint: "または再実行：piz config --init",
    config_overwrite: "設定は既に存在します。上書きしますか？",
    api_key_prompt: "APIキー",
    model_prompt: "モデル名",
    select_provider: "APIプロバイダーを選択",
    base_url_prompt: "API ベース URL",
    custom_url_prompt: "カスタム API URL（プロキシ）を使用しますか？",
    add_openai: "OpenAI 互換バックエンドを追加しますか？",
    add_claude: "Claude バックエンドを追加しますか？",
    add_gemini: "Gemini バックエンドを追加しますか？",
    add_ollama: "Ollama バックエンドを追加しますか？",
    ollama_host: "Ollama ホスト",
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lang_from_code() {
        assert_eq!(Lang::from_code("zh"), Lang::Zh);
        assert_eq!(Lang::from_code("en"), Lang::En);
        assert_eq!(Lang::from_code("ja"), Lang::Ja);
        assert_eq!(Lang::from_code("EN"), Lang::En);
        assert_eq!(Lang::from_code("unknown"), Lang::Zh); // default
        assert_eq!(Lang::from_code(""), Lang::Zh);
    }

    #[test]
    fn lang_code_roundtrip() {
        assert_eq!(Lang::from_code(Lang::Zh.code()), Lang::Zh);
        assert_eq!(Lang::from_code(Lang::En.code()), Lang::En);
        assert_eq!(Lang::from_code(Lang::Ja.code()), Lang::Ja);
    }

    #[test]
    fn all_langs_have_translations() {
        for lang in [Lang::Zh, Lang::En, Lang::Ja] {
            let tr = t(lang);
            assert!(!tr.cached.is_empty());
            assert!(!tr.thinking.is_empty());
            assert!(!tr.danger_warning.is_empty());
            assert!(!tr.execute.is_empty());
            assert!(!tr.wizard_title.is_empty());
        }
    }
}
