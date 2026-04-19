pub fn validate(card_number: &str) -> bool {
    let digits: String = card_number.chars().filter(|c| c.is_ascii_digit()).collect();

    let len = digits.len();
    if !(16..=19).contains(&len) {
        return false;
    }

    if digits.chars().all(|c| c == '0') {
        return false;
    }

    let sum: u32 = digits
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let d = c.to_digit(10).unwrap();
            if i % 2 == 1 {
                let doubled = d * 2;
                if doubled > 9 {
                    doubled - 9
                } else {
                    doubled
                }
            } else {
                d
            }
        })
        .sum();

    sum.is_multiple_of(10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_card_numbers() {
        assert!(validate("6222021234567894"));
        assert!(validate("6228480402564890018"));
        assert!(validate("6222021234567890128"));
    }

    #[test]
    fn test_invalid_card_numbers() {
        assert!(!validate("6222021234567895"));
        assert!(!validate("6228480402564890019"));
        assert!(!validate("6222021234567890129"));
        assert!(!validate("0000000000000000"));
    }

    #[test]
    fn test_length_validation() {
        assert!(!validate("123456789012345"));
        assert!(!validate("12345678901234567890"));
        assert!(!validate(""));
        assert!(!validate("1234567"));
    }

    #[test]
    fn test_non_digit_input() {
        assert!(validate("6222-0212-3456-7894"));
        assert!(validate("6222 0212 3456 7894"));
        assert!(!validate("abcdefghijklmnop"));
        assert!(!validate("6222-0212-3456-7895"));
    }
}
