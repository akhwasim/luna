// Luna safety — command risk detection before execution

pub enum RiskLevel {
    Safe,
    Medium(String),
    High(String),
    Critical(String),
}

fn is_root_deletion(cmd: &str) -> bool {
    let dangerous = [
        "rm -rf /",
        "rm -rf /*",
        "rm -fr /",
        "rm -fr /*",
        "rm -rf ~/",
        "rm -rf ~",
        "rm -rf $HOME",
        "rm -rf \"$HOME\"",
        "rm -rf ../",
        "mv ~ ",
        "mv $HOME",
        "mv \"$HOME\"",
    ];

    for pattern in &dangerous {
        let trimmed = pattern.trim();

        // Exact match
        if cmd == trimmed {
            return true;
        }

        // Starts with pattern — catches "rm -rf / something"
        if cmd.starts_with(&format!("{} ", trimmed)) {
            return true;
        }

        // For subshell wrapping — catches "(rm -rf /)"
        if cmd.starts_with('(') && cmd.contains(trimmed) {
            return true;
        }

        // For double sudo — catches "sudo sudo rm -rf /"
        if cmd.starts_with("sudo") && cmd.contains(trimmed) {
            return true;
        }
    }
    false
}

fn is_remote_execution(cmd: &str) -> bool {
    let fetchers = ["curl ", "wget "];
    let has_fetcher = fetchers.iter().any(|f| cmd.contains(f));
    let has_executor = cmd.contains("| bash")
        || cmd.contains("|bash")
        || cmd.contains("| sh")
        || cmd.contains("|sh")
        || cmd.contains("| zsh")
        || cmd.contains("bash <(")
        || cmd.contains("sh <(");

    has_fetcher && (cmd.contains('|') && has_executor
        || cmd.contains("bash <(")
        || cmd.contains("sh <("))
}

fn is_service_modification(cmd: &str) -> bool {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.len() >= 3 && parts[0] == "service" {
        let action = parts[parts.len() - 1];
        return matches!(action, "stop" | "restart" | "disable" | "kill");
    }
    false
}

fn extract_shell_payload(cmd: &str) -> Option<String> {
    let shell_wrappers = [
        "sudo sh -c ",
        "sudo bash -c ",
        "sudo zsh -c ",
        "bash -c ",
        "sh -c ",
        "zsh -c ",
    ];

    for wrapper in &shell_wrappers {
        if let Some(pos) = cmd.find(wrapper) {
            let after = &cmd[pos + wrapper.len()..];
            let payload = after
                .trim()
                .trim_start_matches('"')
                .trim_end_matches('"')
                .trim_start_matches('\'')
                .trim_end_matches('\'')
                .to_string();

            if !payload.is_empty() {
                return Some(payload);
            }
        }
    }
    None
}

pub fn check(command: &str) -> RiskLevel {
    let cmd = command.trim();

    // Step 1 — root deletion
    if is_root_deletion(cmd) {
        return RiskLevel::Critical(
            "deletes your entire home or root filesystem".to_string(),
        );
    }

    // Step 2 — remote code execution
    if is_remote_execution(cmd) {
        return RiskLevel::High("executes remote code directly".to_string());
    }

    // Step 3 — service modification
    if is_service_modification(cmd) {
        return RiskLevel::High("modifies a system service".to_string());
    }

    // Step 4 — recursive shell payload analysis
    // Opens the box and checks what's inside wrappers like bash -c "..."
    if let Some(payload) = extract_shell_payload(cmd) {
        match check(&payload) {
            RiskLevel::Critical(reason) => {
                return RiskLevel::Critical(
                    format!("shell wrapper hides: {}", reason)
                );
            }
            RiskLevel::High(reason) => {
                return RiskLevel::High(
                    format!("shell wrapper executes: {}", reason)
                );
            }
            RiskLevel::Medium(reason) => {
                return RiskLevel::Medium(
                    format!("shell wrapper executes: {}", reason)
                );
            }
            RiskLevel::Safe => {
                if cmd.starts_with("sudo") {
                    return RiskLevel::Medium(
                        "running safe command with elevated privileges".to_string()
                    );
                }
            }
        }
    }

    // Step 5 — critical patterns
    let critical: &[(&str, &str)] = &[
        ("mkfs",              "formats a drive — destroys all data permanently"),
        ("dd if=",            "low-level disk write — can destroy entire drive"),
        ("dd if=/dev/zero",   "overwrites device with zeros — destroys data"),
        ("dd if=/dev/random", "overwrites device with random data"),
        ("> /dev/sda",        "overwrites disk device directly"),
        ("> /dev/nvme",       "overwrites disk device directly"),
        ("> /etc/passwd",     "overwrites system user database"),
        ("> /etc/shadow",     "overwrites system password database"),
        ("> /etc/hosts",      "overwrites system hosts file"),
        ("chmod 000 /etc",    "removes all permissions from system config"),
        ("chmod -R 777 /",    "removes all security permissions from root"),
        (":(){ :|:& };:",     "fork bomb — crashes the entire system"),
        ("shutdown",          "shuts down the system"),
        ("poweroff",          "powers off the system"),
        ("halt",              "halts the system"),
        ("reboot",            "reboots the system"),
        ("kill -9 -1",        "kills all processes — crashes the session"),
        ("kill -9 1",         "kills init/systemd — destroys the session"),
        ("killall -9",        "force kills all matching processes"),
        ("fdisk /dev/",       "modifies disk partitions — can destroy data"),
        ("parted /dev/",      "modifies disk partitions — can destroy data"),
        ("wipefs",            "erases filesystem signatures from device"),
        ("shred /dev/",       "permanently destroys entire disk device"),
        ("wipe /dev/",        "permanently destroys entire disk device"),
        ("rm -rf *",          "deletes all files in current directory"),
        ("rm -rf .",          "deletes current directory and all contents"),
        ("rm -r /*",          "deletes entire filesystem"),
        ("rm -r *",           "deletes all files in current directory"),
        ("; rm -rf",          "dangerous command hidden after separator"),
        ("|| rm -rf",         "dangerous command hidden after OR operator"),
        ("; find . -delete",  "dangerous command hidden after separator"),
        ("; mkfs",            "dangerous command hidden after separator"),
        ("; dd if=",          "dangerous command hidden after separator"),
    ];

    for (pattern, reason) in critical {
        if cmd.contains(pattern) {
            return RiskLevel::Critical(reason.to_string());
        }
    }

    // Step 6 — high risk patterns
    let high: &[(&str, &str)] = &[
        ("rm -rf",            "permanently deletes files and folders"),
        ("rm -f",             "force deletes without confirmation"),
        ("chmod -R",          "recursively changes file permissions"),
        ("chown -R",          "recursively changes file ownership"),
        ("sudo rm",           "removes files with elevated privileges"),
        ("sudo chmod",        "changes permissions with elevated privileges"),
        ("sudo chown",        "changes ownership with elevated privileges"),
        ("sudo su",           "switches to root user"),
        ("sudo -i",           "opens root shell"),
        ("> /etc/",           "overwrites system config file"),
        ("pkill -9",          "forcefully kills processes"),
        ("kill -9",           "forcefully kills a process"),
        ("find / -delete",    "deletes files recursively from root"),
        ("find . -delete",    "deletes all files in current directory"),
        ("find ~ -delete",    "deletes all files in home directory"),
        ("find -delete",      "deletes files matching criteria"),
        ("systemctl stop",    "stops a system service"),
        ("systemctl disable", "disables a system service"),
        ("systemctl mask",    "masks a system service permanently"),
        ("shred",             "permanently overwrites and deletes files"),
        ("wipe",              "permanently wipes files or devices"),
        ("iptables -F",       "flushes all firewall rules"),
        ("ufw disable",       "disables the firewall"),
        ("$( rm",    "command substitution hiding rm command"),
        ("$(rm",     "command substitution hiding rm command"),
        ("$( dd",    "command substitution hiding dd command"),
        ("$( mkfs",  "command substitution hiding mkfs command"),
    ];

    for (pattern, reason) in high {
        if cmd.contains(pattern) {
            return RiskLevel::High(reason.to_string());
        }
    }

    // Step 7 — medium risk patterns
    let medium: &[(&str, &str)] = &[
        ("sudo apt",          "modifies system packages"),
        ("sudo dnf",          "modifies system packages"),
        ("sudo pacman",       "modifies system packages"),
        ("sudo brew",         "modifies system packages"),
        ("pip install",       "installs Python packages"),
        ("npm install -g",    "installs global Node packages"),
        ("cargo install",     "installs Rust binary globally"),
        ("git push",          "pushes changes to remote"),
        ("git reset --hard",  "discards local changes permanently"),
        ("mv ",               "moves or renames files"),
        ("chmod ",            "changes file permissions"),
    ];

    for (pattern, reason) in medium {
        if cmd.contains(pattern) {
            return RiskLevel::Medium(reason.to_string());
        }
    }

    RiskLevel::Safe
}