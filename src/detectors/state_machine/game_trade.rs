use super::{
    ActionSuggestion, FraudState, FraudStateMachineDef, FraudType,
    StateTransition, GenericStateMachine,
};

pub fn create_game_trade_machine() -> GenericStateMachine {
    let definition = FraudStateMachineDef {
        fraud_type: FraudType::GameTrade,
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
                name: "低价吸引".into(),
                risk_score: 0.3,
                action_suggestion: ActionSuggestion::Monitor,
                warning_text: "轻提示：警惕游戏交易诈骗".into(),
            },
            FraudState {
                id: "S2".into(),
                name: "引导私下".into(),
                risk_score: 0.5,
                action_suggestion: ActionSuggestion::Warn,
                warning_text: "候选词区警告：脱离平台交易无保障".into(),
            },
            FraudState {
                id: "S3".into(),
                name: "要求付款".into(),
                risk_score: 0.8,
                action_suggestion: ActionSuggestion::Block,
                warning_text: "高危弹窗：先付款后交货是典型骗局".into(),
            },
            FraudState {
                id: "S4".into(),
                name: "连环骗".into(),
                risk_score: 1.0,
                action_suggestion: ActionSuggestion::Block,
                warning_text: "全屏阻断：连环收费无法追回！拨打96110".into(),
            },
        ],
        transitions: vec![
            StateTransition {
                from_state: "S0".into(),
                to_state: "S1".into(),
                trigger_keywords: vec![
                    "低价卖号".into(),
                    "代练".into(),
                    "稀有装备".into(),
                ],
            },
            StateTransition {
                from_state: "S1".into(),
                to_state: "S2".into(),
                trigger_keywords: vec![
                    "加QQ".into(),
                    "走第三方".into(),
                    "平台手续费贵".into(),
                ],
            },
            StateTransition {
                from_state: "S2".into(),
                to_state: "S3".into(),
                trigger_keywords: vec![
                    "先转账".into(),
                    "定金".into(),
                    "保证金".into(),
                ],
            },
            StateTransition {
                from_state: "S3".into(),
                to_state: "S4".into(),
                trigger_keywords: vec![
                    "账户冻结".into(),
                    "需要再转一笔解冻".into(),
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
        let mut sm = create_game_trade_machine();
        assert!(sm.transition("低价卖号，代练升级"));
        assert!(sm.transition("加QQ聊，走第三方"));
        assert!(sm.transition("先转定金和保证金"));
        assert!(sm.transition("账户冻结，需要再转一笔解冻"));
        assert_eq!(sm.risk_score(), 1.0);
    }

    #[test]
    fn test_partial() {
        let mut sm = create_game_trade_machine();
        sm.transition("低价卖号");
        sm.transition("加QQ聊");
        assert_eq!(sm.risk_score(), 0.5);
    }

    #[test]
    fn test_no_match() {
        let mut sm = create_game_trade_machine();
        assert!(!sm.transition("今天游戏更新了"));
        assert_eq!(sm.current_state().id, "S0");
    }
}
