use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct SystemContext {
    pub os: String,
    pub shell: String,
    pub cwd: String,
}

pub fn collect_context() -> SystemContext {
    let os = if cfg!(target_os = "windows") {
        detect_windows_version()
    } else if cfg!(target_os = "macos") {
        "macOS".into()
    } else {
        "Linux".into()
    };

    let shell = detect_shell();
    let cwd = std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| ".".into());

    SystemContext { os, shell, cwd }
}

fn detect_windows_version() -> String {
    // Check if running in PowerShell or cmd
    "Windows".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_fields_not_empty() {
        let ctx = collect_context();
        assert!(!ctx.os.is_empty(), "OS should not be empty");
        assert!(!ctx.shell.is_empty(), "shell should not be empty");
        assert!(!ctx.cwd.is_empty(), "cwd should not be empty");
    }

    #[test]
    fn context_os_is_known() {
        let ctx = collect_context();
        let valid = ["Windows", "Linux", "macOS"];
        assert!(
            valid.iter().any(|v| ctx.os.contains(v)),
            "OS '{}' should contain one of {:?}",
            ctx.os,
            valid
        );
    }
}

fn detect_shell() -> String {
    if cfg!(target_os = "windows") {
        // Check parent process or common env vars
        if std::env::var("PSModulePath").is_ok() {
            return "PowerShell".into();
        }
        if std::env::var("SHELL")
            .ok()
            .map(|s| s.contains("bash"))
            .unwrap_or(false)
        {
            return "bash".into();
        }
        return "cmd".into();
    }

    std::env::var("SHELL")
        .unwrap_or_else(|_| "bash".into())
        .rsplit('/')
        .next()
        .unwrap_or("bash")
        .to_string()
}
