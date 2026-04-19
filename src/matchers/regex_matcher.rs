use regex::Regex;

#[derive(Debug, Clone)]
pub struct RegexMatch {
    pub pattern: String,
    pub matched_text: String,
    pub start: usize,
    pub end: usize,
}

pub struct RegexMatcher {
    regexes: Vec<(String, Regex)>,
}

impl RegexMatcher {
    pub fn new(patterns: &[String]) -> Result<Self, regex::Error> {
        let regexes = patterns
            .iter()
            .map(|p| {
                Regex::new(p).map(|r| (p.clone(), r))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { regexes })
    }

    pub fn find_matches(&self, text: &str) -> Vec<RegexMatch> {
        let mut matches = Vec::new();
        for (pattern, regex) in &self.regexes {
            for mat in regex.find_iter(text) {
                matches.push(RegexMatch {
                    pattern: pattern.clone(),
                    matched_text: mat.as_str().to_string(),
                    start: mat.start(),
                    end: mat.end(),
                });
            }
        }
        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bank_card_pattern() {
        let patterns: Vec<String> = vec![r"\d{16,19}".into()];
        let matcher = RegexMatcher::new(&patterns).unwrap();
        let matches = matcher.find_matches("卡号6222021234567890123请转账");

        assert!(!matches.is_empty());
        assert_eq!(matches[0].matched_text, "6222021234567890123");
    }

    #[test]
    fn test_id_card_pattern() {
        let patterns: Vec<String> = vec![r"[1-9]\d{5}(18|19|20)?\d{2}(0[1-9]|1[0-2])(0[1-9]|[12]\d|3[01])\d{3}[\dXx]".into()];
        let matcher = RegexMatcher::new(&patterns).unwrap();
        let matches = matcher.find_matches("身份证号11010519491231002X");

        assert!(!matches.is_empty());
    }

    #[test]
    fn test_verification_code_pattern() {
        let patterns: Vec<String> = vec![r"\d{6}".into()];
        let matcher = RegexMatcher::new(&patterns).unwrap();
        let matches = matcher.find_matches("验证码是123456");

        assert!(!matches.is_empty());
        assert_eq!(matches[0].matched_text, "123456");
    }

    #[test]
    fn test_no_match() {
        let patterns: Vec<String> = vec![r"\d{16,19}".into()];
        let matcher = RegexMatcher::new(&patterns).unwrap();
        let matches = matcher.find_matches("这段文本没有银行卡号");

        assert!(matches.is_empty());
    }

    #[test]
    fn test_invalid_pattern() {
        let patterns: Vec<String> = vec!["[invalid".into()];
        let result = RegexMatcher::new(&patterns);
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_patterns() {
        let patterns: Vec<String> = vec![r"\d{6}".into(), r"[a-zA-Z]+".into()];
        let matcher = RegexMatcher::new(&patterns).unwrap();
        let matches = matcher.find_matches("验证码abc123456xyz");

        assert!(matches.len() >= 2);
    }
}
