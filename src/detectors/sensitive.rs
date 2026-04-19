use serde::{Deserialize, Serialize};
use crate::utils::{luhn, idcard, text_clean};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SensitiveType {
    BankCard,
    IdCard,
    VerificationCode,
    PayPassword,
}

impl SensitiveType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SensitiveType::BankCard => "bank_card",
            SensitiveType::IdCard => "id_card",
            SensitiveType::VerificationCode => "verification_code",
            SensitiveType::PayPassword => "pay_password",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitiveInfo {
    pub sensitive_type: SensitiveType,
    pub value: String,
    pub position: usize,
}

pub struct SensitiveDetector {
    bank_card_regex: regex::Regex,
    id_card_regex: regex::Regex,
    code_regex: regex::Regex,
    code_context_keywords: Vec<String>,
    pay_context_keywords: Vec<String>,
}

impl Default for SensitiveDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl SensitiveDetector {
    pub fn new() -> Self {
        let bank_card_regex = regex::Regex::new(r"\d{16,19}").expect("银行卡正则编译失败");
        let id_card_regex = regex::Regex::new(r"[1-9]\d{5}(?:18|19|20)\d{2}(?:0[1-9]|1[0-2])(?:0[1-9]|[12]\d|3[01])\d{3}[\dXx]")
            .expect("身份证正则编译失败");
        let code_regex = regex::Regex::new(r"\d{6}").expect("验证码正则编译失败");

        let code_context_keywords = vec![
            "验证码".into(),
            "动态码".into(),
            "校验码".into(),
            "确认码".into(),
            "安全码".into(),
        ];
        let pay_context_keywords = vec![
            "转账".into(),
            "付款".into(),
            "支付".into(),
            "汇款".into(),
            "充值".into(),
        ];

        Self {
            bank_card_regex,
            id_card_regex,
            code_regex,
            code_context_keywords,
            pay_context_keywords,
        }
    }

    pub fn detect(&self, text: &str) -> Vec<SensitiveInfo> {
        let cleaned = text_clean::clean(text);
        let mut results = Vec::new();

        self.detect_bank_cards(&cleaned, &mut results);
        self.detect_id_cards(&cleaned, &mut results);
        self.detect_verification_codes(&cleaned, &mut results);
        self.detect_pay_passwords(&cleaned, &mut results);

        results
    }

    fn detect_bank_cards(&self, text: &str, results: &mut Vec<SensitiveInfo>) {
        for mat in self.bank_card_regex.find_iter(text) {
            let candidate = mat.as_str();
            if luhn::validate(candidate) {
                results.push(SensitiveInfo {
                    sensitive_type: SensitiveType::BankCard,
                    value: Self::mask_value(candidate, 4, 4),
                    position: mat.start(),
                });
            }
        }
    }

    fn detect_id_cards(&self, text: &str, results: &mut Vec<SensitiveInfo>) {
        for mat in self.id_card_regex.find_iter(text) {
            let candidate = mat.as_str();
            if idcard::validate(candidate) {
                results.push(SensitiveInfo {
                    sensitive_type: SensitiveType::IdCard,
                    value: Self::mask_value(candidate, 3, 4),
                    position: mat.start(),
                });
            }
        }
    }

    fn detect_verification_codes(&self, text: &str, results: &mut Vec<SensitiveInfo>) {
        for mat in self.code_regex.find_iter(text) {
            let position = mat.start();
            if self.has_context_keywords(text, position, &self.code_context_keywords) {
                results.push(SensitiveInfo {
                    sensitive_type: SensitiveType::VerificationCode,
                    value: "******".into(),
                    position,
                });
            }
        }
    }

    fn detect_pay_passwords(&self, text: &str, results: &mut Vec<SensitiveInfo>) {
        for mat in self.code_regex.find_iter(text) {
            let position = mat.start();
            if self.has_context_keywords(text, position, &self.pay_context_keywords) {
                let already_detected = results.iter().any(|r| {
                    r.position == position && r.sensitive_type == SensitiveType::VerificationCode
                });
                if !already_detected {
                    results.push(SensitiveInfo {
                        sensitive_type: SensitiveType::PayPassword,
                        value: "******".into(),
                        position,
                    });
                }
            }
        }
    }

    fn has_context_keywords(&self, text: &str, position: usize, keywords: &[String]) -> bool {
        let context_start = position.saturating_sub(30);
        let context_end = (position + 30).min(text.len());
        let context = &text[context_start..context_end];
        let lower = context.to_lowercase();
        keywords.iter().any(|kw| lower.contains(&kw.to_lowercase()))
    }

    fn mask_value(value: &str, keep_start: usize, keep_end: usize) -> String {
        let chars: Vec<char> = value.chars().collect();
        let len = chars.len();
        if len <= keep_start + keep_end {
            return value.to_string();
        }
        let mut result = String::new();
        for (i, c) in chars.iter().enumerate() {
            if i < keep_start || i >= len - keep_end {
                result.push(*c);
            } else {
                result.push('*');
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_bank_card() {
        let detector = SensitiveDetector::new();
        let results = detector.detect("卡号6222021234567894请转账");
        let bank_cards: Vec<_> = results.iter().filter(|r| r.sensitive_type == SensitiveType::BankCard).collect();
        assert!(!bank_cards.is_empty());
    }

    #[test]
    fn test_detect_id_card() {
        let detector = SensitiveDetector::new();
        let results = detector.detect("身份证号11010519491231002X");
        let id_cards: Vec<_> = results.iter().filter(|r| r.sensitive_type == SensitiveType::IdCard).collect();
        assert!(!id_cards.is_empty());
    }

    #[test]
    fn test_detect_verification_code() {
        let detector = SensitiveDetector::new();
        let results = detector.detect("验证码是123456，请输入");
        let codes: Vec<_> = results.iter().filter(|r| r.sensitive_type == SensitiveType::VerificationCode).collect();
        assert!(!codes.is_empty());
    }

    #[test]
    fn test_no_verification_code_without_context() {
        let detector = SensitiveDetector::new();
        let results = detector.detect("数量是123456个");
        let codes: Vec<_> = results.iter().filter(|r| r.sensitive_type == SensitiveType::VerificationCode).collect();
        assert!(codes.is_empty());
    }

    #[test]
    fn test_detect_pay_password() {
        let detector = SensitiveDetector::new();
        let results = detector.detect("转账密码123456");
        let passwords: Vec<_> = results.iter().filter(|r| r.sensitive_type == SensitiveType::PayPassword).collect();
        assert!(!passwords.is_empty());
    }

    #[test]
    fn test_mask_value() {
        assert_eq!(SensitiveDetector::mask_value("1234567890", 3, 2), "123*****90");
        assert_eq!(SensitiveDetector::mask_value("12345", 2, 2), "12*45");
    }

    #[test]
    fn test_no_false_positive_on_short_numbers() {
        let detector = SensitiveDetector::new();
        let results = detector.detect("价格是1234元");
        let bank_cards: Vec<_> = results.iter().filter(|r| r.sensitive_type == SensitiveType::BankCard).collect();
        assert!(bank_cards.is_empty());
    }
}
