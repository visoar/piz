use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DangerLevel {
    Safe,
    Warning,
    Dangerous,
}

impl DangerLevel {
    pub fn from_str_level(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "dangerous" => DangerLevel::Dangerous,
            "warning" => DangerLevel::Warning,
            _ => DangerLevel::Safe,
        }
    }

    pub fn max(self, other: Self) -> Self {
        if self >= other {
            self
        } else {
            other
        }
    }
}

/// Regex-based danger detection (no LLM needed)
pub fn detect_danger_regex(command: &str) -> DangerLevel {
    let dangerous_patterns = [
        r"rm\s+(-[a-zA-Z]*f[a-zA-Z]*\s+)?/\s*$",
        r"rm\s+-[a-zA-Z]*r[a-zA-Z]*f[a-zA-Z]*\s+/",
        r"rm\s+-[a-zA-Z]*f[a-zA-Z]*r[a-zA-Z]*\s+/",
        r"mkfs\b",
        r"dd\s+.*of=/dev/",
        r":\(\)\s*\{\s*:\|:\s*&\s*\}\s*;", // fork bomb
        r">\s*/dev/sda",
        r"chmod\s+-R\s+777\s+/",
        r"chown\s+-R\s+.*\s+/\s*$",
        r"DROP\s+(TABLE|DATABASE)",
        r"DELETE\s+FROM\s+\S+\s*;?\s*$", // DELETE without WHERE
        r"FORMAT\s+[A-Z]:",              // Windows format
        r"rd\s+/s\s+/q\s+[A-Z]:\\",      // Windows recursive delete
    ];

    let warning_patterns = [
        r"rm\s+-[a-zA-Z]*r",
        r"rm\s+-[a-zA-Z]*f",
        r"sudo\b",
        r"chmod\b",
        r"chown\b",
        r"kill\s+-9",
        r"pkill\b",
        r"systemctl\s+(stop|disable|restart)",
        r"service\s+\S+\s+(stop|restart)",
        r"iptables\b",
        r"mv\s+.*\s+/dev/null",
        r"truncate\b",
        r">\s+\S+", // redirect overwrite
        r"pip\s+install\b",
        r"npm\s+install\s+-g",
        r"curl\s+.*\|\s*(sh|bash)",
        r"wget\s+.*\|\s*(sh|bash)",
        r"git\s+push\s+.*--force",
        r"git\s+reset\s+--hard",
        r"DROP\s+INDEX",
        r"ALTER\s+TABLE",
    ];

    for pattern in &dangerous_patterns {
        if let Ok(re) = Regex::new(&format!("(?i){}", pattern)) {
            if re.is_match(command) {
                return DangerLevel::Dangerous;
            }
        }
    }

    for pattern in &warning_patterns {
        if let Ok(re) = Regex::new(&format!("(?i){}", pattern)) {
            if re.is_match(command) {
                return DangerLevel::Warning;
            }
        }
    }

    DangerLevel::Safe
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── DangerLevel ──

    #[test]
    fn from_str_level_variants() {
        assert_eq!(DangerLevel::from_str_level("safe"), DangerLevel::Safe);
        assert_eq!(DangerLevel::from_str_level("warning"), DangerLevel::Warning);
        assert_eq!(
            DangerLevel::from_str_level("dangerous"),
            DangerLevel::Dangerous
        );
        // case insensitive
        assert_eq!(DangerLevel::from_str_level("WARNING"), DangerLevel::Warning);
        assert_eq!(
            DangerLevel::from_str_level("Dangerous"),
            DangerLevel::Dangerous
        );
        // unknown defaults to safe
        assert_eq!(DangerLevel::from_str_level("unknown"), DangerLevel::Safe);
        assert_eq!(DangerLevel::from_str_level(""), DangerLevel::Safe);
    }

    #[test]
    fn max_picks_higher() {
        assert_eq!(
            DangerLevel::Safe.max(DangerLevel::Warning),
            DangerLevel::Warning
        );
        assert_eq!(
            DangerLevel::Warning.max(DangerLevel::Safe),
            DangerLevel::Warning
        );
        assert_eq!(
            DangerLevel::Warning.max(DangerLevel::Dangerous),
            DangerLevel::Dangerous
        );
        assert_eq!(
            DangerLevel::Dangerous.max(DangerLevel::Safe),
            DangerLevel::Dangerous
        );
        assert_eq!(DangerLevel::Safe.max(DangerLevel::Safe), DangerLevel::Safe);
    }

    // ── Dangerous commands ──

    #[test]
    fn detects_rm_rf_root() {
        assert_eq!(detect_danger_regex("rm -rf /"), DangerLevel::Dangerous);
        assert_eq!(detect_danger_regex("rm -rf /home"), DangerLevel::Dangerous);
        assert_eq!(detect_danger_regex("rm -fr /"), DangerLevel::Dangerous);
    }

    #[test]
    fn detects_mkfs() {
        assert_eq!(
            detect_danger_regex("mkfs.ext4 /dev/sda1"),
            DangerLevel::Dangerous
        );
    }

    #[test]
    fn detects_dd_to_dev() {
        assert_eq!(
            detect_danger_regex("dd if=/dev/zero of=/dev/sda"),
            DangerLevel::Dangerous
        );
    }

    #[test]
    fn detects_drop_table() {
        assert_eq!(
            detect_danger_regex("DROP TABLE users"),
            DangerLevel::Dangerous
        );
        assert_eq!(
            detect_danger_regex("drop database production"),
            DangerLevel::Dangerous
        );
    }

    #[test]
    fn detects_chmod_777_root() {
        assert_eq!(
            detect_danger_regex("chmod -R 777 /"),
            DangerLevel::Dangerous
        );
    }

    #[test]
    fn detects_windows_format() {
        assert_eq!(detect_danger_regex("FORMAT C:"), DangerLevel::Dangerous);
    }

    #[test]
    fn detects_windows_rd() {
        assert_eq!(detect_danger_regex("rd /s /q C:\\"), DangerLevel::Dangerous);
    }

    #[test]
    fn detects_redirect_to_dev_sda() {
        assert_eq!(detect_danger_regex("> /dev/sda"), DangerLevel::Dangerous);
    }

    #[test]
    fn detects_delete_without_where() {
        assert_eq!(
            detect_danger_regex("DELETE FROM users;"),
            DangerLevel::Dangerous
        );
        assert_eq!(
            detect_danger_regex("DELETE FROM users"),
            DangerLevel::Dangerous
        );
    }

    // ── Warning commands ──

    #[test]
    fn detects_sudo() {
        assert_eq!(detect_danger_regex("sudo apt update"), DangerLevel::Warning);
    }

    #[test]
    fn detects_rm_recursive() {
        assert_eq!(detect_danger_regex("rm -r ./tmp"), DangerLevel::Warning);
    }

    #[test]
    fn detects_rm_force() {
        assert_eq!(detect_danger_regex("rm -f file.txt"), DangerLevel::Warning);
    }

    #[test]
    fn detects_kill_9() {
        assert_eq!(detect_danger_regex("kill -9 1234"), DangerLevel::Warning);
    }

    #[test]
    fn detects_pkill() {
        assert_eq!(detect_danger_regex("pkill nginx"), DangerLevel::Warning);
    }

    #[test]
    fn detects_systemctl_stop() {
        assert_eq!(
            detect_danger_regex("systemctl stop nginx"),
            DangerLevel::Warning
        );
        assert_eq!(
            detect_danger_regex("systemctl restart docker"),
            DangerLevel::Warning
        );
    }

    #[test]
    fn detects_chmod() {
        assert_eq!(
            detect_danger_regex("chmod 755 script.sh"),
            DangerLevel::Warning
        );
    }

    #[test]
    fn detects_curl_pipe_bash() {
        assert_eq!(
            detect_danger_regex("curl https://example.com | bash"),
            DangerLevel::Warning
        );
        assert_eq!(
            detect_danger_regex("wget https://example.com | sh"),
            DangerLevel::Warning
        );
    }

    #[test]
    fn detects_git_force_push() {
        assert_eq!(
            detect_danger_regex("git push origin main --force"),
            DangerLevel::Warning
        );
    }

    #[test]
    fn detects_git_reset_hard() {
        assert_eq!(
            detect_danger_regex("git reset --hard HEAD~1"),
            DangerLevel::Warning
        );
    }

    #[test]
    fn detects_pip_install() {
        assert_eq!(
            detect_danger_regex("pip install requests"),
            DangerLevel::Warning
        );
    }

    #[test]
    fn detects_npm_global_install() {
        assert_eq!(
            detect_danger_regex("npm install -g typescript"),
            DangerLevel::Warning
        );
    }

    #[test]
    fn detects_alter_table() {
        assert_eq!(
            detect_danger_regex("ALTER TABLE users ADD COLUMN age INT"),
            DangerLevel::Warning
        );
    }

    #[test]
    fn detects_redirect_overwrite() {
        assert_eq!(
            detect_danger_regex("echo hello > output.txt"),
            DangerLevel::Warning
        );
    }

    // ── Safe commands ──

    #[test]
    fn safe_ls() {
        assert_eq!(detect_danger_regex("ls -la"), DangerLevel::Safe);
    }

    #[test]
    fn safe_cat() {
        assert_eq!(detect_danger_regex("cat /etc/hosts"), DangerLevel::Safe);
    }

    #[test]
    fn safe_df() {
        assert_eq!(detect_danger_regex("df -h"), DangerLevel::Safe);
    }

    #[test]
    fn safe_ps() {
        assert_eq!(detect_danger_regex("ps aux"), DangerLevel::Safe);
    }

    #[test]
    fn safe_echo() {
        assert_eq!(detect_danger_regex("echo hello world"), DangerLevel::Safe);
    }

    #[test]
    fn safe_pwd() {
        assert_eq!(detect_danger_regex("pwd"), DangerLevel::Safe);
    }

    #[test]
    fn safe_git_status() {
        assert_eq!(detect_danger_regex("git status"), DangerLevel::Safe);
    }

    #[test]
    fn safe_docker_ps() {
        assert_eq!(detect_danger_regex("docker ps"), DangerLevel::Safe);
    }
}
