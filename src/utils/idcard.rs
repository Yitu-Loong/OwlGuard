const WEIGHT_FACTORS: [u32; 17] = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
const CHECK_CODES: [char; 11] = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];

pub fn validate(id_number: &str) -> bool {
    if id_number.len() != 18 {
        return false;
    }

    let chars: Vec<char> = id_number.chars().collect();

    for c in chars.iter().take(17) {
        if !c.is_ascii_digit() {
            return false;
        }
    }

    let last = chars[17];
    if !last.is_ascii_digit() && last != 'X' && last != 'x' {
        return false;
    }

    let digits: Vec<u32> = chars[..17]
        .iter()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let weighted_sum: u32 = digits
        .iter()
        .zip(WEIGHT_FACTORS.iter())
        .map(|(d, w)| d * w)
        .sum();

    let check_index = (weighted_sum % 11) as usize;
    let expected = CHECK_CODES[check_index];

    if last == 'x' {
        return expected == 'X';
    }

    last == expected
}

fn is_valid_date(year: u32, month: u32, day: u32) -> bool {
    if !(1900..=2100).contains(&year) {
        return false;
    }
    if !(1..=12).contains(&month) {
        return false;
    }

    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400) {
                29
            } else {
                28
            }
        }
        _ => return false,
    };

    (1..=max_day).contains(&day)
}

pub fn validate_with_date(id_number: &str) -> bool {
    if !validate(id_number) {
        return false;
    }

    let chars: Vec<char> = id_number.chars().collect();

    let year: u32 = chars[6..10]
        .iter()
        .collect::<String>()
        .parse()
        .unwrap_or(0);
    let month: u32 = chars[10..12]
        .iter()
        .collect::<String>()
        .parse()
        .unwrap_or(0);
    let day: u32 = chars[12..14]
        .iter()
        .collect::<String>()
        .parse()
        .unwrap_or(0);

    is_valid_date(year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_id_numbers() {
        assert!(validate("11010519491231002X"));
        assert!(validate("110105199507166012"));
        assert!(validate("440308199901010514"));
        assert!(validate("320311199001010018"));
    }

    #[test]
    fn test_invalid_check_digit() {
        assert!(!validate("110105194912310021"));
        assert!(!validate("110105199507166013"));
        assert!(!validate("440308199901010511"));
    }

    #[test]
    fn test_format_errors() {
        assert!(!validate("1101051949123100"));
        assert!(!validate("11010519491231002X1"));
        assert!(!validate(""));
        assert!(!validate("11010519491231002Y"));
    }

    #[test]
    fn test_non_digit_in_body() {
        assert!(!validate("11010X19491231002X"));
        assert!(!validate("A1010519491231002X"));
    }

    #[test]
    fn test_lowercase_x() {
        assert!(validate("11010519491231002x"));
        assert!(!validate("44030819990101051x"));
    }

    #[test]
    fn test_date_validation() {
        assert!(validate_with_date("11010519491231002X"));
        assert!(validate_with_date("110105199507166012"));
        assert!(!validate_with_date("110105194902290010"));
        assert!(!validate_with_date("110105194913310013"));
    }

    #[test]
    fn test_is_valid_date() {
        assert!(is_valid_date(2000, 2, 29));
        assert!(!is_valid_date(1900, 2, 29));
        assert!(is_valid_date(2024, 2, 29));
        assert!(!is_valid_date(2023, 2, 29));
        assert!(is_valid_date(2023, 1, 31));
        assert!(!is_valid_date(2023, 4, 31));
        assert!(!is_valid_date(2023, 13, 1));
        assert!(!is_valid_date(2023, 0, 1));
    }
}
