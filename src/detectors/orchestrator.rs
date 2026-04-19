use super::risk_scorer::RiskScorer;
use super::sensitive::SensitiveDetector;
use super::state_machine::{
    brushing, fake_customer, fake_leader, fake_police, fake_shopping, flight_change, game_trade,
    investment, loan_credit, romance, ActionSuggestion, FraudStateMachine, FraudType,
};
use super::threat_intel::ThreatIntelChecker;
use crate::mcp::tools::{ScanResult, SensitiveResult, SensitiveItem};

pub struct ScanOrchestrator {
    sensitive_detector: SensitiveDetector,
    threat_checker: ThreatIntelChecker,
}

impl Default for ScanOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

struct FraudScanResult {
    score: f64,
    fraud_type: FraudType,
    state_name: String,
    action: ActionSuggestion,
    warning: String,
    reasoning: String,
    keywords: Vec<String>,
}

impl ScanOrchestrator {
    pub fn new() -> Self {
        Self {
            sensitive_detector: SensitiveDetector::new(),
            threat_checker: ThreatIntelChecker::new(),
        }
    }

    pub fn scan_conversation(&self, messages: &[crate::mcp::tools::ChatMessage]) -> ScanResult {
        let all_text: String = messages.iter().map(|m| m.content.as_str()).collect::<Vec<_>>().join(" ");

        let sensitive_items = self.sensitive_detector.detect(&all_text);

        let mut best_result: Option<FraudScanResult> = None;

        let mut machines: Vec<Box<dyn FraudStateMachine>> = vec![
            Box::new(brushing::create_brushing_machine()),
            Box::new(investment::create_investment_machine()),
            Box::new(fake_shopping::create_fake_shopping_machine()),
            Box::new(fake_customer::create_fake_customer_machine()),
            Box::new(loan_credit::create_loan_credit_machine()),
            Box::new(fake_leader::create_fake_leader_machine()),
            Box::new(fake_police::create_fake_police_machine()),
            Box::new(romance::create_romance_machine()),
            Box::new(game_trade::create_game_trade_machine()),
            Box::new(flight_change::create_flight_change_machine()),
        ];

        for sm in &mut machines {
            for msg in messages {
                sm.transition(&msg.content);
            }
            let score = sm.risk_score();
            if score > best_result.as_ref().map_or(0.0, |r| r.score) {
                best_result = Some(FraudScanResult {
                    score,
                    fraud_type: sm.fraud_type(),
                    state_name: sm.current_state().name.clone(),
                    action: sm.action_suggestion(),
                    warning: sm.warning_text(),
                    reasoning: sm.reasoning(),
                    keywords: sm.matched_keywords().to_vec(),
                });
            }
        }

        let (fraud_score, fraud_type, current_state, fraud_action, fraud_warning, fraud_reasoning, matched_keywords) =
            match best_result {
                Some(r) => (r.score, r.fraud_type, r.state_name, r.action, r.warning, r.reasoning, r.keywords),
                None => (0.0, FraudType::Brushing, "初始".into(), ActionSuggestion::None, String::new(), String::new(), vec![]),
            };

        let assessment = RiskScorer::assess(
            fraud_score,
            if fraud_score > 0.0 { Some(fraud_type) } else { None },
            fraud_action,
            fraud_warning,
            &sensitive_items,
            fraud_reasoning,
        );

        ScanResult {
            risk_score: assessment.risk_score,
            risk_level: assessment.risk_level.as_str().to_string(),
            fraud_type: assessment.fraud_type.map(|t| t.name().to_string()),
            fraud_type_id: assessment.fraud_type.map(|t| t.id().to_string()),
            current_state: if fraud_score > 0.0 { Some(current_state) } else { None },
            matched_keywords,
            action_suggestion: assessment.action_suggestion.as_str().to_string(),
            warning_text: assessment.warning_text,
            reasoning: Some(assessment.reasoning),
        }
    }

    pub fn detect_sensitive(&self, text: &str) -> SensitiveResult {
        let items = self.sensitive_detector.detect(text);
        SensitiveResult {
            found: !items.is_empty(),
            sensitive_items: items
                .into_iter()
                .map(|s| SensitiveItem {
                    sensitive_type: s.sensitive_type.as_str().to_string(),
                    value: s.value,
                    position: s.position,
                })
                .collect(),
        }
    }

    pub fn check_url(&self, url: &str) -> crate::mcp::tools::UrlCheckResult {
        let result = self.threat_checker.check_url(url);
        crate::mcp::tools::UrlCheckResult {
            is_malicious: result.is_malicious,
            threat_type: result.threat_type,
            source: result.source,
        }
    }

    pub fn check_app(&self, app_name: &str) -> crate::mcp::tools::AppCheckResult {
        let result = self.threat_checker.check_app(app_name);
        crate::mcp::tools::AppCheckResult {
            is_dangerous: result.is_malicious,
            risk_reason: result.threat_type,
            source: result.source,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcp::tools::ChatMessage;

    #[test]
    fn test_scan_normal_conversation() {
        let orchestrator = ScanOrchestrator::new();
        let messages = vec![
            ChatMessage { role: "user".into(), content: "今天天气真好".into() },
        ];
        let result = orchestrator.scan_conversation(&messages);
        assert_eq!(result.risk_level, "none");
        assert_eq!(result.risk_score, 0.0);
    }

    #[test]
    fn test_scan_brushing_fraud() {
        let orchestrator = ScanOrchestrator::new();
        let messages = vec![
            ChatMessage { role: "other".into(), content: "手机兼职，日结300-500".into() },
            ChatMessage { role: "other".into(), content: "我们是正规平台，不交押金".into() },
            ChatMessage { role: "other".into(), content: "先做一单试试，马上返你".into() },
            ChatMessage { role: "other".into(), content: "这单佣金高，需要垫付".into() },
        ];
        let result = orchestrator.scan_conversation(&messages);
        assert!(result.risk_score >= 0.8);
        assert_eq!(result.fraud_type, Some("刷单返利".to_string()));
        assert_eq!(result.action_suggestion, "block");
    }

    #[test]
    fn test_scan_with_sensitive_info() {
        let orchestrator = ScanOrchestrator::new();
        let messages = vec![
            ChatMessage { role: "other".into(), content: "请提供银行卡号6222021234567894".into() },
        ];
        let result = orchestrator.scan_conversation(&messages);
        assert!(result.risk_score > 0.0);
    }

    #[test]
    fn test_detect_sensitive_bank_card() {
        let orchestrator = ScanOrchestrator::new();
        let result = orchestrator.detect_sensitive("卡号6222021234567894");
        assert!(result.found);
    }

    #[test]
    fn test_check_malicious_url() {
        let orchestrator = ScanOrchestrator::new();
        let result = orchestrator.check_url("https://fake-invest.com");
        assert!(result.is_malicious);
    }

    #[test]
    fn test_check_safe_app() {
        let orchestrator = ScanOrchestrator::new();
        let result = orchestrator.check_app("微信");
        assert!(!result.is_dangerous);
    }
}
