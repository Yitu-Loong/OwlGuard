pub mod keyword;
pub mod regex_matcher;

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub pattern: String,
    pub matched_text: String,
    pub start: usize,
    pub end: usize,
}

pub trait Matcher {
    fn find_matches(&self, text: &str) -> Vec<MatchResult>;
}

impl Matcher for keyword::KeywordMatcher {
    fn find_matches(&self, text: &str) -> Vec<MatchResult> {
        keyword::KeywordMatcher::find_matches(self, text)
            .into_iter()
            .map(|m| MatchResult {
                pattern: m.keyword.clone(),
                matched_text: m.keyword,
                start: m.start,
                end: m.end,
            })
            .collect()
    }
}

impl Matcher for regex_matcher::RegexMatcher {
    fn find_matches(&self, text: &str) -> Vec<MatchResult> {
        regex_matcher::RegexMatcher::find_matches(self, text)
            .into_iter()
            .map(|m| MatchResult {
                pattern: m.pattern,
                matched_text: m.matched_text,
                start: m.start,
                end: m.end,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_matcher_via_trait() {
        let keywords: Vec<String> = vec!["兼职".into(), "日结".into()];
        let matcher = keyword::KeywordMatcher::new(&keywords);
        let results: Vec<MatchResult> = Matcher::find_matches(&matcher, "手机兼职日结300");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_regex_matcher_via_trait() {
        let patterns: Vec<String> = vec![r"\d{6}".into()];
        let matcher = regex_matcher::RegexMatcher::new(&patterns).unwrap();
        let results: Vec<MatchResult> = Matcher::find_matches(&matcher, "验证码123456");
        assert!(!results.is_empty());
        assert_eq!(results[0].matched_text, "123456");
    }
}
