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
    ];
    for pattern in &dangerous {
        if cmd == *pattern
            || cmd.starts_with(&format!("{} ", pattern.trim()))
            || cmd.trim() == pattern.trim()
        {
            return true;
        }
    }
    false
}

pub fn check(command: &str) -> RiskLevel {
    let cmd = command.trim();

    // Root deletion — always critical
    if is_root_deletion(cmd) {
        return RiskLevel::Critical("deletes your entire home or root filesystem".to_string());
    }

    let critical: &[(&str, &str)] = &[
        ("mkfs",            "formats a drive — destroys all data permanently"),
        ("dd if=",          "low-level disk write — can destroy entire drive"),
        ("> /dev/sda",      "overwrites disk device directly"),
        ("> /dev/nvme",     "overwrites disk device directly"),
        ("chmod -R 777 /",  "removes all security permissions from root"),
        (":(){ :|:& };:",   "fork bomb — crashes the entire system"),
        ("shutdown",        "shuts down the system"),
        ("poweroff",        "powers off the system"),
        ("halt",            "halts the system"),
        ("reboot",          "reboots the system"),
        ("kill -9 -1",      "kills all processes — crashes the session"),
        ("killall -9",      "force kills all matching processes"),
    ];

    for (pattern, reason) in critical {
        if cmd.contains(pattern) {
            return RiskLevel::Critical(reason.to_string());
        }
    }

    let high: &[(&str, &str)] = &[
        ("rm -rf",          "permanently deletes files and folders"),
        ("rm -f",           "force deletes without confirmation"),
        ("curl | bash",     "executes remote code directly"),
        ("curl | sh",       "executes remote code directly"),
        ("wget | bash",     "executes remote code directly"),
        ("wget | sh",       "executes remote code directly"),
        ("bash <(curl",     "executes remote code directly"),
        ("sh <(curl",       "executes remote code directly"),
        ("bash <(wget",     "executes remote code directly"),
        ("chmod -R",        "recursively changes file permissions"),
        ("chown -R",        "recursively changes file ownership"),
        ("sudo rm",         "removes files with elevated privileges"),
        ("sudo chmod",      "changes permissions with elevated privileges"),
        ("sudo chown",      "changes ownership with elevated privileges"),
        ("> /etc/",         "overwrites system config file"),
        ("pkill -9",        "forcefully kills processes"),
        ("kill -9",         "forcefully kills a process"),
    ];

    for (pattern, reason) in high {
        if cmd.contains(pattern) {
            return RiskLevel::High(reason.to_string());
        }
    }

    let medium: &[(&str, &str)] = &[
        ("sudo apt",        "modifies system packages"),
        ("sudo dnf",        "modifies system packages"),
        ("sudo pacman",     "modifies system packages"),
        ("sudo brew",       "modifies system packages"),
        ("pip install",     "installs Python packages"),
        ("npm install -g",  "installs global Node packages"),
        ("cargo install",   "installs Rust binary globally"),
        ("git push",        "pushes changes to remote"),
        ("git reset --hard","discards local changes permanently"),
        ("mv ",             "moves or renames files"),
        ("chmod ",          "changes file permissions"),
    ];

    for (pattern, reason) in medium {
        if cmd.contains(pattern) {
            return RiskLevel::Medium(reason.to_string());
        }
    }

    RiskLevel::Safe
}