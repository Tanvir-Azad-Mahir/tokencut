use regex::Regex;
use std::sync::OnceLock;

static ANSI_REGEX: OnceLock<Regex> = OnceLock::new();
static STACK_PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();

fn get_ansi_regex() -> &'static Regex {
    ANSI_REGEX.get_or_init(|| Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").unwrap())
}

fn get_stack_patterns() -> &'static Vec<Regex> {
    STACK_PATTERNS.get_or_init(|| {
        vec![
            Regex::new(r#"^\s*File ".+?/(site-packages|lib/python\d\.\d+)/.+?", line \d+"#)
                .unwrap(),
            Regex::new(r"^\s*at node_modules/.+:\d+:\d+").unwrap(),
            Regex::new(r"^\s*at Module\._compile \(.+\)").unwrap(),
        ]
    })
}

pub fn clean_terminal_output(raw_text: &str, prune_stack: bool) -> String {
    // 1. Strip ANSI escape codes
    let text = get_ansi_regex().replace_all(raw_text, "");

    // 2. Normalize newline markers
    let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
    let lines: Vec<&str> = normalized.lines().collect();

    let patterns = get_stack_patterns();
    let mut processed_lines: Vec<String> = Vec::new();
    let mut last_line: Option<&str> = None;
    let mut dup_count = 0;

    for line in lines {
        // Hide noisy third-party stack frames if enabled
        if prune_stack && patterns.iter().any(|pat| pat.is_match(line)) {
            let placeholder = "  [... external internal stack frames hidden ...]";
            if processed_lines.last().map(|s| s.as_str()) != Some(placeholder) {
                processed_lines.push(placeholder.to_string());
            }
            continue;
        }

        // Deduplicate repeating lines
        if last_line == Some(line) && !line.trim().is_empty() {
            dup_count += 1;
        } else {
            if dup_count > 0 {
                processed_lines.push(format!("  [... repeated {} times ...]", dup_count));
                dup_count = 0;
            }
            processed_lines.push(line.to_string());
            last_line = Some(line);
        }
    }

    if dup_count > 0 {
        processed_lines.push(format!("  [... repeated {} times ...]", dup_count));
    }

    processed_lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::clean_terminal_output;

    #[test]
    fn strips_ansi_color_codes() {
        assert_eq!(clean_terminal_output("\x1b[31mERROR\x1b[0m", true), "ERROR");
    }

    #[test]
    fn strips_terminal_control_sequences() {
        assert_eq!(clean_terminal_output("build\x1b[2K ok", true), "build ok");
    }

    #[test]
    fn leaves_plain_text_unchanged() {
        let input = "Starting build\nerror: cannot find symbol\nFinished";
        assert_eq!(clean_terminal_output(input, true), input);
    }

    #[test]
    fn normalizes_windows_and_classic_mac_newlines() {
        assert_eq!(
            clean_terminal_output("one\r\ntwo\rthree", true),
            "one\ntwo\nthree"
        );
    }

    #[test]
    fn collapses_consecutive_identical_lines() {
        let input = "retry\nretry\nretry\nfailed";
        assert_eq!(
            clean_terminal_output(input, true),
            "retry\n  [... repeated 2 times ...]\nfailed"
        );
    }

    #[test]
    fn flushes_repeats_at_end_of_input() {
        assert_eq!(
            clean_terminal_output("retry\nretry", true),
            "retry\n  [... repeated 1 times ...]"
        );
    }

    #[test]
    fn does_not_collapse_nonconsecutive_duplicates() {
        let input = "retry\nworking\nretry";
        assert_eq!(clean_terminal_output(input, true), input);
    }

    #[test]
    fn does_not_collapse_blank_lines() {
        assert_eq!(clean_terminal_output("a\n\n\nb", true), "a\n\n\nb");
    }

    #[test]
    fn pruning_hides_python_site_packages_frames_but_keeps_application_frame() {
        let input = concat!(
            "Traceback (most recent call last):\n",
            "  File \"app.py\", line 12, in main\n",
            "  File \"/project/.venv/lib/python3.11/site-packages/requests/api.py\", line 9, in get\n",
            "ValueError: bad input"
        );
        let expected = concat!(
            "Traceback (most recent call last):\n",
            "  File \"app.py\", line 12, in main\n",
            "  [... external internal stack frames hidden ...]\n",
            "ValueError: bad input"
        );
        assert_eq!(clean_terminal_output(input, true), expected);
    }

    #[test]
    fn pruning_hides_node_modules_frames() {
        let input = "Error\n    at node_modules/pkg/index.js:12:3\napp error";
        assert_eq!(
            clean_terminal_output(input, true),
            "Error\n  [... external internal stack frames hidden ...]\napp error"
        );
    }

    #[test]
    fn pruning_hides_node_module_compile_frames() {
        let input =
            "Error\n    at Module._compile (node:internal/modules/cjs/loader:1256:14)\nDone";
        assert_eq!(
            clean_terminal_output(input, true),
            "Error\n  [... external internal stack frames hidden ...]\nDone"
        );
    }

    #[test]
    fn neighboring_pruned_frames_share_one_placeholder() {
        let input = concat!(
            "Error\n",
            "    at node_modules/a/index.js:1:2\n",
            "    at node_modules/b/index.js:3:4\n",
            "Failed"
        );
        assert_eq!(
            clean_terminal_output(input, true),
            "Error\n  [... external internal stack frames hidden ...]\nFailed"
        );
    }

    #[test]
    fn no_prune_preserves_library_frames() {
        let input = "Error\n    at node_modules/pkg/index.js:12:3\nFailed";
        assert_eq!(clean_terminal_output(input, false), input);
    }

    #[test]
    fn preserves_error_information_and_unicode() {
        let input = "⚠️ Warning: échec\nERROR: connection refused";
        assert_eq!(clean_terminal_output(input, true), input);
    }

    #[test]
    fn empty_input_produces_empty_output() {
        assert_eq!(clean_terminal_output("", true), "");
    }
}
