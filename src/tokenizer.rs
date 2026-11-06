use tiktoken_rs::cl100k_base;

pub fn count_tokens(text: &str) -> usize {
    let bpe = cl100k_base().unwrap();
    bpe.encode_with_special_tokens(text).len()
}

#[cfg(test)]
mod tests {
    use super::count_tokens;

    #[test]
    fn empty_input_has_zero_tokens() {
        assert_eq!(count_tokens(""), 0);
    }

    #[test]
    fn nonempty_input_has_tokens() {
        assert!(count_tokens("hello world") > 0);
    }

    #[test]
    fn repeated_text_uses_more_tokens_than_single_word() {
        assert!(count_tokens("hello hello hello hello") > count_tokens("hello"));
    }

    #[test]
    fn handles_unicode_text() {
        assert!(count_tokens("বাংলা ভাষা ⚙️") > 0);
    }
}
