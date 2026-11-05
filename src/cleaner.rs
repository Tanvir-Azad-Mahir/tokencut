use regex::Regex;
use std::sync::OnceLock;

static ANSI_REGEX: OnceLock<Regex> = OnceLock::new();
static STACK_PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();

fn get_ansi_regex() -> &'static Regex {
    ANSI_REGEX.get_or_init(|| {
        Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").unwrap()
    })
}

fn get_stack_patterns() -> &'static Vec<Regex> {
    STACK_PATTERNS.get_or_init(|| {
        vec![
            Regex::new(r#"^\s*File ".+?/(site-packages|lib/python\d\.\d+)/.+?", line \d+"#).unwrap(),
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