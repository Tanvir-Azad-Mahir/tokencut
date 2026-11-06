mod cleaner;
mod tokenizer;

use arboard::Clipboard;
use clap::Parser;
use std::io::{self, IsTerminal, Read};
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about = "Strip log noise & compress terminal context for LLMs", long_about = None)]
struct Args {
    /// Copy output directly to system clipboard
    #[arg(short, long)]
    copy: bool,

    /// Disable library stack trace pruning
    #[arg(long)]
    no_prune: bool,
}

fn main() {
    let args = Args::parse();

    // Ensure input is piped via stdin
    if io::stdin().is_terminal() {
        eprintln!("Usage: cat log.txt | tokencut [options]");
        eprintln!("   or: Get-Content log.txt | tokencut [options]");
        process::exit(1);
    }

    let mut raw_input = String::new();
    if io::stdin().read_to_string(&mut raw_input).is_err() || raw_input.trim().is_empty() {
        eprintln!("Error: Received empty input stream.");
        process::exit(1);
    }

    // Clean output
    let cleaned = cleaner::clean_terminal_output(&raw_input, !args.no_prune);

    // Calculate token counts
    let raw_tokens = tokenizer::count_tokens(&raw_input);
    let clean_tokens = tokenizer::count_tokens(&cleaned);
    let savings = if raw_tokens > 0 {
        100.0 - (clean_tokens as f64 / raw_tokens as f64 * 100.0)
    } else {
        0.0
    };

    let formatted_result = format!("```log\n{}\n```", cleaned);

    // Print result
    println!("{}", formatted_result);
    eprintln!(
        "\n✂️ tokencut: {} tokens → {} tokens ({:.1}% saved)",
        raw_tokens, clean_tokens, savings
    );

    // Handle clipboard flag
    if args.copy {
        if let Ok(mut clipboard) = Clipboard::new() {
            if clipboard.set_text(&formatted_result).is_ok() {
                eprintln!("📋 Copied formatted log to clipboard!");
            }
        }
    }
}
