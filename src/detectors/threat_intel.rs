use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatResult {
    pub is_malicious: bool,
    pub threat_type: Option<String>,
    pub source: String,
}

pub struct ThreatIntelChecker {
    url_blacklist: HashSet<String>,
    app_blacklist: HashSet<String>,
}

impl Default for ThreatIntelChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl ThreatIntelChecker {
    pub fn new() -> Self {
        Self {
            url_blacklist: Self::default_url_blacklist(),
            app_blacklist: Self::default_app_blacklist(),
        }
    }

    pub fn with_blacklists(url_blacklist: HashSet<String>, app_blacklist: HashSet<String>) -> Self {
        Self {
            url_blacklist,
            app_blacklist,
        }
    }

    pub fn check_url(&self, url: &str) -> ThreatResult {
        let lower = url.to_lowercase();
        let domain = Self::extract_domain(&lower);
        let is_malicious = self.url_blacklist.contains(&lower)
            || domain.as_ref().map(|d| self.url_blacklist.contains(d)).unwrap_or(false);

        ThreatResult {
            is_malicious,
            threat_type: if is_malicious { Some("phishing".into()) } else { None },
            source: "local_blacklist".into(),
        }
    }

    pub fn check_app(&self, app_name: &str) -> ThreatResult {
        let lower = app_name.to_lowercase();
        let is_dangerous = self.app_blacklist.contains(&lower);

        ThreatResult {
            is_malicious: is_dangerous,
            threat_type: if is_dangerous { Some("fraud_app".into()) } else { None },
            source: "local_blacklist".into(),
        }
    }

    fn extract_domain(url: &str) -> Option<String> {
        let without_scheme = url
            .strip_prefix("https://")
            .or_else(|| url.strip_prefix("http://"))
            .unwrap_or(url);
        let domain = without_scheme.split('/').next().unwrap_or(without_scheme);
        let domain = domain.split(':').next().unwrap_or(domain);
        if domain.is_empty() {
            None
        } else {
            Some(domain.to_lowercase())
        }
    }

    fn default_url_blacklist() -> HashSet<String> {
        let mut set = HashSet::new();
        set.insert("fake-invest.com".into());
        set.insert("malicious-app.download".into());
        set.insert("phishing-bank.cn".into());
        set
    }

    fn default_app_blacklist() -> HashSet<String> {
        let mut set = HashSet::new();
        set.insert("fake invest pro".into());
        set.insert("刷单神器".into());
        set.insert("内幕消息".into());
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_malicious_url() {
        let checker = ThreatIntelChecker::new();
        let result = checker.check_url("https://fake-invest.com/download");
        assert!(result.is_malicious);
        assert_eq!(result.threat_type, Some("phishing".into()));
    }

    #[test]
    fn test_check_safe_url() {
        let checker = ThreatIntelChecker::new();
        let result = checker.check_url("https://www.baidu.com");
        assert!(!result.is_malicious);
    }

    #[test]
    fn test_check_dangerous_app() {
        let checker = ThreatIntelChecker::new();
        let result = checker.check_app("fake invest pro");
        assert!(result.is_malicious);
    }

    #[test]
    fn test_check_safe_app() {
        let checker = ThreatIntelChecker::new();
        let result = checker.check_app("微信");
        assert!(!result.is_malicious);
    }

    #[test]
    fn test_extract_domain() {
        assert_eq!(ThreatIntelChecker::extract_domain("https://example.com/path"), Some("example.com".into()));
        assert_eq!(ThreatIntelChecker::extract_domain("http://test.cn:8080/api"), Some("test.cn".into()));
    }
}
