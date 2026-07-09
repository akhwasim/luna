// Multi-line input detection for Luna.
//
// Bash treats some lines as "not finished yet" — heredocs, unbalanced
// quotes, lines ending with backslash continuations. We replicate that
// here so users can paste or type multi-line shell blocks without Luna
// splitting them into separate commands.
//
// The detection rules mirror what bash does:
//   - Line ends with `\` → incomplete (line continuation)
//   - Unbalanced single quotes → incomplete
//   - Unbalanced double quotes → incomplete
//   - Unbalanced backticks → incomplete
//   - Heredoc opener (`<< WORD` or `<<- WORD`) without the matching WORD on
//     its own line → incomplete
//
// Once the user types enough to close everything, the line is Complete
// and Luna runs it as one command.

use reedline::{ValidationResult, Validator};

pub struct LunaValidator;

impl Validator for LunaValidator {
    fn validate(&self, line: &str) -> ValidationResult {
        if is_incomplete(line) {
            ValidationResult::Incomplete
        } else {
            ValidationResult::Complete
        }
    }
}

/// Check whether a line of input is "incomplete" — bash would wait for
/// more input rather than execute it. Pure function, no side effects.
fn is_incomplete(line: &str) -> bool {
    // Backslash continuation at end of line
    if line.ends_with('\\') && !line.ends_with("\\\\") {
        return true;
    }

    // Count quotes, ignoring those escaped by backslashes
    let (single, double, backtick) = count_quotes(line);

    if single % 2 != 0 || double % 2 != 0 || backtick % 2 != 0 {
        return true;
    }

    // Heredoc detection: if a << opener is present and the closing
    // marker hasn't appeared on its own line, the input is incomplete.
    if has_unclosed_heredoc(line) {
        return true;
    }

    false
}

/// Count single quotes, double quotes, and backticks in a line, treating
/// backslash-escaped quotes as literal characters, not quote delimiters.
fn count_quotes(line: &str) -> (usize, usize, usize) {
    let mut single = 0;
    let mut double = 0;
    let mut backtick = 0;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            // Skip the next character — it's escaped
            chars.next();
            continue;
        }
        match c {
            '\'' => single += 1,
            '"' => double += 1,
            '`' => backtick += 1,
            _ => {}
        }
    }

    (single, double, backtick)
}

/// Detect an unclosed heredoc. Looks for `<<EOF` or `<<-EOF` style
/// openers and checks whether the closing marker has appeared on its
/// own line.
fn has_unclosed_heredoc(line: &str) -> bool {
    // Find the heredoc opener. We look for `<<` followed by a word.
    // Common patterns: `<<EOF`, `<<-EOF`, `<< 'EOF'`, `<< "EOF"`.
    let mut start = 0;
    while let Some(pos) = line[start..].find("<<") {
        let abs_pos = start + pos;
        let after = &line[abs_pos + 2..];

        // Skip optional `-` for `<<-`
        let after = after.trim_start_matches('-');

        // Skip whitespace
        let after = after.trim_start();

        // Extract the marker — word until whitespace or end of line
        let marker: String = after
            .chars()
            .take_while(|c| !c.is_whitespace())
            .collect();

        // Strip quotes from the marker if present
        let marker = marker.trim_matches(|c| c == '\'' || c == '"');

        if marker.is_empty() {
            start = abs_pos + 2;
            continue;
        }

        // Check whether any line in the input ends with the marker on
        // its own (i.e., the closing line of the heredoc).
        let closing_present = line
            .lines()
            .skip(1) // skip the line with the opener
            .any(|l| l.trim() == marker);

        if !closing_present {
            return true;
        }

        start = abs_pos + 2 + marker.len();
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_complete() {
        assert!(!is_incomplete("ls -la"));
        assert!(!is_incomplete("cargo build"));
    }

    #[test]
    fn backslash_continuation() {
        assert!(is_incomplete("cargo build \\"));
        assert!(!is_incomplete("cargo build"));
    }

    #[test]
    fn unbalanced_quotes() {
        assert!(is_incomplete("echo \"hello"));
        assert!(is_incomplete("echo 'hello"));
        assert!(is_incomplete("echo `uname"));
        assert!(!is_incomplete("echo \"hello\""));
        assert!(!is_incomplete("echo 'hello'"));
        assert!(!is_incomplete("echo `uname -s`"));
    }

    #[test]
    fn escaped_quotes_dont_count() {
        assert!(!is_incomplete("echo \"hello \\\"world\\\"\""));
    }

    #[test]
    fn heredoc_detection() {
        assert!(is_incomplete("cat <<EOF\nhello"));
        assert!(!is_incomplete("cat <<EOF\nhello\nEOF"));
    }

    #[test]
    fn ssh_agent_use_case() {
        // The exact case from the bug report
        let multi = "eval \"$(ssh-agent -s)\"\nssh-add ~/.ssh/id_ed25519";
        assert!(!is_incomplete(multi));
    }
}