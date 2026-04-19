pub fn clean(text: &str) -> String {
    let no_zero_width = remove_zero_width_chars(text);
    let half_width = full_width_to_half_width(&no_zero_width);
    normalize_whitespace(&half_width)
}

fn full_width_to_half_width(text: &str) -> String {
    text.chars()
        .map(|c| {
            if ('\u{FF10}'..='\u{FF19}').contains(&c)
                || ('\u{FF21}'..='\u{FF3A}').contains(&c)
                || ('\u{FF41}'..='\u{FF5A}').contains(&c)
            {
                char::from_u32(c as u32 - 0xFEE0).unwrap()
            } else if c == '\u{3000}' {
                ' '
            } else {
                c
            }
        })
        .collect()
}

fn normalize_whitespace(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut prev_was_space = false;

    for c in text.chars() {
        if c.is_whitespace() {
            if !prev_was_space {
                result.push(' ');
                prev_was_space = true;
            }
        } else {
            result.push(c);
            prev_was_space = false;
        }
    }

    let trimmed = result.trim();
    trimmed.to_string()
}

fn remove_zero_width_chars(text: &str) -> String {
    text.chars()
        .filter(|c| {
            !matches!(
                c,
                '\u{200B}'
                    | '\u{200C}'
                    | '\u{200D}'
                    | '\u{200E}'
                    | '\u{200F}'
                    | '\u{FEFF}'
                    | '\u{2060}'
                    | '\u{180E}'
                    | '\u{202A}'
                    | '\u{202B}'
                    | '\u{202C}'
                    | '\u{202D}'
                    | '\u{202E}'
                    | '\u{2061}'
                    | '\u{2062}'
                    | '\u{2063}'
                    | '\u{2064}'
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_width_digits() {
        assert_eq!(clean("１２３４５６７８９０"), "1234567890");
    }

    #[test]
    fn test_full_width_letters() {
        assert_eq!(clean("ＡＢＣＤＥＦ"), "ABCDEF");
        assert_eq!(clean("ａｂｃｄｅｆ"), "abcdef");
    }

    #[test]
    fn test_full_width_space() {
        assert_eq!(clean("hello　world"), "hello world");
    }

    #[test]
    fn test_whitespace_normalization() {
        assert_eq!(clean("hello   world"), "hello world");
        assert_eq!(clean("hello\tworld"), "hello world");
        assert_eq!(clean("hello\nworld"), "hello world");
        assert_eq!(clean("hello\r\nworld"), "hello world");
        assert_eq!(clean("  hello  world  "), "hello world");
    }

    #[test]
    fn test_zero_width_chars() {
        let input = "hello\u{200B}world";
        assert_eq!(clean(input), "helloworld");

        let input2 = "hello\u{200C}world";
        assert_eq!(clean(input2), "helloworld");

        let input3 = "hello\u{FEFF}world";
        assert_eq!(clean(input3), "helloworld");

        let input4 = "test\u{200D}data";
        assert_eq!(clean(input4), "testdata");
    }

    #[test]
    fn test_zero_width_with_spaces() {
        let input = "hello \u{200B}world";
        assert_eq!(clean(input), "hello world");
    }

    #[test]
    fn test_mixed_operations() {
        let input = "　ＡＢＣ　１２３\u{200B}　";
        assert_eq!(clean(input), "ABC 123");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(clean(""), "");
    }

    #[test]
    fn test_no_change_needed() {
        assert_eq!(clean("hello world 123"), "hello world 123");
    }

    #[test]
    fn test_chinese_text_unchanged() {
        assert_eq!(clean("你好世界"), "你好世界");
    }

    #[test]
    fn test_trailing_leading_whitespace() {
        assert_eq!(clean("  hello  "), "hello");
        assert_eq!(clean("\t\nhello\n\t"), "hello");
    }
}
