use aho_corasick::AhoCorasick;

#[derive(Debug, Clone)]
pub struct KeywordMatch {
    pub keyword: String,
    pub start: usize,
    pub end: usize,
}

pub struct KeywordMatcher {
    ac: AhoCorasick,
    keywords: Vec<String>,
}

impl KeywordMatcher {
    pub fn new(keywords: &[String]) -> Self {
        let patterns: Vec<String> = keywords.iter().map(|k| k.to_lowercase()).collect();
        let ac = AhoCorasick::builder()
            .ascii_case_insensitive(true)
            .build(&patterns)
            .expect("aho-corasick模式构建失败");
        Self {
            ac,
            keywords: keywords.to_vec(),
        }
    }

    pub fn find_matches(&self, text: &str) -> Vec<KeywordMatch> {
        let mut matches = Vec::new();
        for mat in self.ac.find_iter(text) {
            let pattern_idx = mat.pattern().as_usize();
            if pattern_idx < self.keywords.len() {
                matches.push(KeywordMatch {
                    keyword: self.keywords[pattern_idx].clone(),
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
    fn test_basic_matching() {
        let keywords: Vec<String> = vec!["兼职".into(), "日结".into(), "宝妈可做".into()];
        let matcher = KeywordMatcher::new(&keywords);
        let matches = matcher.find_matches("手机兼职，日结300-500");

        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].keyword, "兼职");
        assert_eq!(matches[1].keyword, "日结");
    }

    #[test]
    fn test_case_insensitive() {
        let keywords: Vec<String> = vec!["APP".into(), "VIP".into()];
        let matcher = KeywordMatcher::new(&keywords);
        let matches = matcher.find_matches("扫码下载app，开通vip通道");

        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].keyword, "APP");
        assert_eq!(matches[1].keyword, "VIP");
    }

    #[test]
    fn test_no_match() {
        let keywords: Vec<String> = vec!["兼职".into(), "日结".into()];
        let matcher = KeywordMatcher::new(&keywords);
        let matches = matcher.find_matches("今天天气真好");

        assert!(matches.is_empty());
    }

    #[test]
    fn test_multiple_occurrences() {
        let keywords: Vec<String> = vec!["兼职".into()];
        let matcher = KeywordMatcher::new(&keywords);
        let matches = matcher.find_matches("兼职一：刷单兼职，兼职二：代练");

        assert_eq!(matches.len(), 3);
    }

    #[test]
    fn test_empty_keywords() {
        let keywords: Vec<String> = vec![];
        let matcher = KeywordMatcher::new(&keywords);
        let matches = matcher.find_matches("任何文本");

        assert!(matches.is_empty());
    }

    #[test]
    fn test_position_accuracy() {
        let keywords: Vec<String> = vec!["hello".into()];
        let matcher = KeywordMatcher::new(&keywords);
        let matches = matcher.find_matches("say hello world");

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].start, 4);
        assert_eq!(matches[0].end, 9);
        assert_eq!(matches[0].keyword, "hello");
    }
}
