use super::sensitive::{SensitiveInfo, SensitiveType};
use super::state_machine::{ActionSuggestion, AlertLevel, FraudType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub risk_score: f64,
    pub risk_level: AlertLevel,
    pub fraud_type: Option<FraudType>,
    pub action_suggestion: ActionSuggestion,
    pub warning_text: String,
    pub reasoning: String,
}

pub struct RiskScorer;

impl RiskScorer {
    pub fn assess(
        fraud_risk_score: f64,
        fraud_type: Option<FraudType>,
        fraud_action: ActionSuggestion,
        fraud_warning: String,
        sensitive_items: &[SensitiveInfo],
        fraud_reasoning: String,
    ) -> RiskAssessment {
        let has_verification_code = sensitive_items
            .iter()
            .any(|s| s.sensitive_type == SensitiveType::VerificationCode);
        let has_pay_password = sensitive_items
            .iter()
            .any(|s| s.sensitive_type == SensitiveType::PayPassword);
        let has_bank_card = sensitive_items
            .iter()
            .any(|s| s.sensitive_type == SensitiveType::BankCard);
        let has_id_card = sensitive_items
            .iter()
            .any(|s| s.sensitive_type == SensitiveType::IdCard);

        let (final_score, risk_level, action, warning) = if fraud_risk_score > 0.5
            && (has_verification_code || has_pay_password)
        {
            let score = (fraud_risk_score + 0.3).min(1.0);
            (
                score,
                AlertLevel::Critical,
                ActionSuggestion::Block,
                format!(
                    "⚠️ 极度危险：检测到{}话术且对方索要{}，这是诈骗最后阶段！{}",
                    fraud_type.map(|t| t.name()).unwrap_or("未知"),
                    if has_verification_code { "验证码" } else { "支付密码" },
                    "立即停止对话，拨打96110！"
                ),
            )
        } else if fraud_risk_score > 0.3 && has_bank_card {
            let score = (fraud_risk_score + 0.2).min(1.0);
            (
                score,
                AlertLevel::High,
                ActionSuggestion::Block,
                format!(
                    "🔴 高危：检测到{}话术且对方索要银行卡号，极可能是诈骗！{}",
                    fraud_type.map(|t| t.name()).unwrap_or("未知"),
                    "切勿继续提供信息，拨打96110求助"
                ),
            )
        } else if fraud_risk_score > 0.3 && has_id_card {
            let score = (fraud_risk_score + 0.15).min(1.0);
            (
                score,
                AlertLevel::High,
                ActionSuggestion::Block,
                format!(
                    "🔴 高危：检测到{}话术且对方索要身份证号，极可能是诈骗！",
                    fraud_type.map(|t| t.name()).unwrap_or("未知")
                ),
            )
        } else if fraud_risk_score > 0.6 {
            let level = if fraud_action == ActionSuggestion::Block {
                AlertLevel::High
            } else {
                AlertLevel::Medium
            };
            (
                fraud_risk_score,
                level,
                fraud_action,
                fraud_warning.clone(),
            )
        } else if fraud_risk_score > 0.1 {
            (
                fraud_risk_score,
                AlertLevel::Low,
                fraud_action,
                fraud_warning.clone(),
            )
        } else if !sensitive_items.is_empty() && fraud_risk_score <= 0.1 {
            (
                0.3,
                AlertLevel::Medium,
                ActionSuggestion::Warn,
                "检测到敏感信息但无诈骗话术背景，请注意保护个人信息".into(),
            )
        } else {
            (
                fraud_risk_score,
                AlertLevel::None,
                ActionSuggestion::None,
                String::new(),
            )
        };

        let reasoning = if fraud_reasoning.is_empty() && sensitive_items.is_empty() {
            "未检测到风险".into()
        } else {
            let mut parts = Vec::new();
            if !fraud_reasoning.is_empty() {
                parts.push(fraud_reasoning);
            }
            if !sensitive_items.is_empty() {
                let types: Vec<&str> = sensitive_items
                    .iter()
                    .map(|s| s.sensitive_type.as_str())
                    .collect();
                parts.push(format!("检测到敏感信息: {}", types.join(", ")));
            }
            parts.join("；")
        };

        RiskAssessment {
            risk_score: final_score,
            risk_level,
            fraud_type,
            action_suggestion: action,
            warning_text: warning,
            reasoning,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fraud_high_plus_verification_code_is_critical() {
        let result = RiskScorer::assess(
            0.6,
            Some(FraudType::Brushing),
            ActionSuggestion::Warn,
            "警告".into(),
            &[SensitiveInfo {
                sensitive_type: SensitiveType::VerificationCode,
                value: "******".into(),
                position: 0,
            }],
            "刷单返利话术状态机路径: S0 → S3".into(),
        );
        assert_eq!(result.risk_level, AlertLevel::Critical);
        assert_eq!(result.action_suggestion, ActionSuggestion::Block);
        assert!(result.risk_score > 0.8);
    }

    #[test]
    fn test_fraud_medium_plus_bank_card_is_high() {
        let result = RiskScorer::assess(
            0.4,
            Some(FraudType::LoanCredit),
            ActionSuggestion::Warn,
            "警告".into(),
            &[SensitiveInfo {
                sensitive_type: SensitiveType::BankCard,
                value: "6222****0123".into(),
                position: 0,
            }],
            "贷款征信话术状态机路径: S0 → S2".into(),
        );
        assert_eq!(result.risk_level, AlertLevel::High);
        assert_eq!(result.action_suggestion, ActionSuggestion::Block);
    }

    #[test]
    fn test_fraud_medium_plus_id_card_is_high() {
        let result = RiskScorer::assess(
            0.4,
            Some(FraudType::FakePolice),
            ActionSuggestion::Warn,
            "警告".into(),
            &[SensitiveInfo {
                sensitive_type: SensitiveType::IdCard,
                value: "110***********002X".into(),
                position: 0,
            }],
            "冒充公检法话术状态机路径: S0 → S2".into(),
        );
        assert_eq!(result.risk_level, AlertLevel::High);
    }

    #[test]
    fn test_sensitive_only_without_fraud_is_medium() {
        let result = RiskScorer::assess(
            0.0,
            None,
            ActionSuggestion::None,
            String::new(),
            &[SensitiveInfo {
                sensitive_type: SensitiveType::BankCard,
                value: "6222****0123".into(),
                position: 0,
            }],
            String::new(),
        );
        assert_eq!(result.risk_level, AlertLevel::Medium);
        assert_eq!(result.action_suggestion, ActionSuggestion::Warn);
    }

    #[test]
    fn test_no_risk() {
        let result = RiskScorer::assess(
            0.0,
            None,
            ActionSuggestion::None,
            String::new(),
            &[],
            String::new(),
        );
        assert_eq!(result.risk_level, AlertLevel::None);
        assert_eq!(result.action_suggestion, ActionSuggestion::None);
    }

    #[test]
    fn test_fraud_only_high_risk() {
        let result = RiskScorer::assess(
            0.7,
            Some(FraudType::Investment),
            ActionSuggestion::Warn,
            "浮层警告".into(),
            &[],
            "投资理财话术状态机路径: S0 → S3".into(),
        );
        assert_eq!(result.risk_level, AlertLevel::Medium);
        assert_eq!(result.action_suggestion, ActionSuggestion::Warn);
    }

    #[test]
    fn test_fraud_low_risk() {
        let result = RiskScorer::assess(
            0.2,
            Some(FraudType::Brushing),
            ActionSuggestion::Monitor,
            "轻提示".into(),
            &[],
            "刷单返利话术状态机路径: S0 → S1".into(),
        );
        assert_eq!(result.risk_level, AlertLevel::Low);
        assert_eq!(result.action_suggestion, ActionSuggestion::Monitor);
    }
}
