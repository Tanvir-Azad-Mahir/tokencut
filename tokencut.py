#!/usr/bin/env python3
import sys
import re
import argparse
import pyperclip
import tiktoken

# ANSI Escape sequence regex parser
ANSI_REGEX = re.compile(r'\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])')

# Common third-party / standard library stack trace patterns to collapse
NOISE_TRACE_PATTERNS = [
    re.compile(r'^\s*File ".+?/site-packages/.+?", line \d+'),
    re.compile(r'^\s*File ".+?/lib/python\d\.\d+/.+?", line \d+'),
    re.compile(r'^\s*at node_modules/.+:\d+:\d+'),
    re.compile(r'^\s*at Module\._compile \(.+\)')
]

def clean_terminal_output(raw_text: str, prune_stack: bool = True) -> str:
    # 1. Strip ANSI escape sequences (colors, progress bars, terminal control)
    text = ANSI_REGEX.sub('', raw_text)

    # 2. Normalize whitespace and remove trailing blank lines
    text = re.sub(r'\r\n|\r', '\n', text)
    text = re.sub(r'\n\s*\n\s*\n+', '\n\n', text)

    lines = text.splitlines()
    processed_lines = []
    
    last_line = None
    dup_count = 0

    for line in lines:
        # Prune library noise from stack traces if enabled
        if prune_stack and any(pattern.match(line) for pattern in NOISE_TRACE_PATTERNS):
            if not processed_lines or processed_lines[-1] != "  [... external internal stack frames hidden ...]":
                processed_lines.append("  [... external internal stack frames hidden ...]")
            continue

        # Collapse identical duplicate lines (e.g., retries, polling logs)
        if line == last_line and line.strip() != "":
            dup_count += 1
        else:
            if dup_count > 0:
                processed_lines.append(f"  [... repeated {dup_count} times ...]")
                dup_count = 0
            processed_lines.append(line)
            last_line = line

    if dup_count > 0:
        processed_lines.append(f"  [... repeated {dup_count} times ...]")

    return "\n".join(processed_lines)


def main():
    parser = argparse.ArgumentParser(
        description="tokencut: Strip terminal log noise and compress LLM context tokens."
    )
    parser.add_argument(
        "-c", "--copy", action="store_true", help="Copy processed output to clipboard"
    )
    parser.add_argument(
        "--no-prune", action="store_true", help="Disable stack trace pruning"
    )
    args = parser.parse_args()

    # Read input from stdin
    if sys.stdin.isatty():
        print("Usage: cat log.txt | tokencut [options] or <command> | tokencut [options]")
        sys.exit(1)

    raw_input = sys.stdin.read()
    if not raw_input.strip():
        print("Error: Input stream is empty.", file=sys.stderr)
        sys.exit(1)

    # Process logs
    cleaned_output = clean_terminal_output(raw_input, prune_stack=not args.no_prune)

    # Calculate token metrics using OpenAI cl100k_base (GPT-4/GPT-3.5 tokenization)
    enc = tiktoken.get_encoding("cl100k_base")
    raw_tokens = len(enc.encode(raw_input))
    clean_tokens = len(enc.encode(cleaned_output))
    savings = 100 - (clean_tokens / raw_tokens * 100) if raw_tokens > 0 else 0

    # Format Markdown block for LLM prompt context
    formatted_result = f"```log\n{cleaned_output}\n```"

    # Print output to stdout
    print(formatted_result)
    print(
        f"\n✂️ tokencut: {raw_tokens:,} tokens → {clean_tokens:,} tokens ({savings:.1f}% saved)",
        file=sys.stderr,
    )

    # Copy to clipboard if requested
    if args.copy:
        pyperclip.copy(formatted_result)
        print("📋 Copied formatted log to clipboard!", file=sys.stderr)


if __name__ == "__main__":
    main()