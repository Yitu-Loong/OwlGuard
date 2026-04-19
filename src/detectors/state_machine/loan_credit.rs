use super::{
    ActionSuggestion, FraudState, FraudStateMachineDef, FraudType,
    StateTransition, GenericStateMachine,
};

pub fn create_loan_credit_machine() -> GenericStateMachine {
    let definition = FraudStateMachineDef {
        fraud_type: FraudType::LoanCredit,
        states: vec![
            FraudState {
                id: "S0".into(),
                name: "初始".into(),
                risk_score: 0.0,
                action_suggestion: ActionSuggestion::None,
                warning_text: String::new(),
            },
            FraudState {
                id: "S1".into(),
                name: "吸引眼球".into(),
                risk_score: 0.3,
                action_suggestion: ActionSuggestion::Monitor,
                warning_text: "轻提示：警惕贷款诈骗".into(),
            },
            FraudState {
                id: "S2".into(),
                name: "索要资料".into(),
                risk_score: 0.5,
                action_suggestion: ActionSuggestion::Warn,
                warning_text: "候选词区警告：正规贷款不需要提前提供银行卡号".into(),
            },
            FraudState {
                id: "S3".into(),
                name: "制造障碍".into(),
                risk_score: 0.8,
                action_suggestion: ActionSuggestion::Block,
                warning_text: "高危弹窗：账户冻结需解冻费是典型骗局".into(),
            },
            FraudState {
                id: "S4".into(),
                name: "连环收费".into(),
                risk_score: 1.0,
                action_suggestion: ActionSuggestion::Block,
                warning_text: "全屏阻断：贷款前收费都是诈骗！拨打96110".into(),
            },
        ],
        transitions: vec![
            StateTransition {
                from_state: "S0".into(),
                to_state: "S1".into(),
                trigger_keywords: vec![
                    "无抵押".into(),
                    "秒到账".into(),
                    "黑户也能贷".into(),
                    "低息贷款".into(),
                ],
            },
            StateTransition {
                from_state: "S1".into(),
                to_state: "S2".into(),
                trigger_keywords: vec![
                    "身份证正反面".into(),
                    "银行卡号".into(),
                    "手持身份证".into(),
                ],
            },
            StateTransition {
                from_state: "S2".into(),
                to_state: "S3".into(),
                trigger_keywords: vec![
                    "银行卡号填错".into(),
                    "账户冻结".into(),
                    "需要解冻费".into(),
                ],
            },
            StateTransition {
                from_state: "S3".into(),
                to_state: "S4".into(),
                trigger_keywords: vec![
                    "保证金".into(),
                    "验证金".into(),
                    "手续费".into(),
                ],
            },
        ],
    };
    GenericStateMachine::new(definition)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detectors::state_machine::FraudStateMachine;

    #[test]
    fn test_full_chain() {
        let mut sm = create_loan_credit_machine();
        assert!(sm.transition("无抵押秒到账，黑户也能贷"));
        assert!(sm.transition("请提供身份证正反面和银行卡号"));
        assert!(sm.transition("银行卡号填错，账户冻结，需要解冻费"));
        assert!(sm.transition("还需要交保证金和手续费"));
        assert_eq!(sm.risk_score(), 1.0);
    }

    #[test]
    fn test_partial() {
        let mut sm = create_loan_credit_machine();
        sm.transition("低息贷款，无抵押");
        sm.transition("请提供银行卡号");
        assert_eq!(sm.risk_score(), 0.5);
    }

    #[test]
    fn test_no_match() {
        let mut sm = create_loan_credit_machine();
        assert!(!sm.transition("银行利率下调了"));
        assert_eq!(sm.current_state().id, "S0");
    }
}
