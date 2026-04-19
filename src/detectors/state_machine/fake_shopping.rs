use super::{
    ActionSuggestion, FraudState, FraudStateMachineDef, FraudType,
    StateTransition, GenericStateMachine,
};

pub fn create_fake_shopping_machine() -> GenericStateMachine {
    let definition = FraudStateMachineDef {
        fraud_type: FraudType::FakeShopping,
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
                risk_score: 0.2,
                action_suggestion: ActionSuggestion::Monitor,
                warning_text: "轻提示：警惕虚假购物诈骗".into(),
            },
            FraudState {
                id: "S2".into(),
                name: "引导脱离平台".into(),
                risk_score: 0.5,
                action_suggestion: ActionSuggestion::Warn,
                warning_text: "浮层警告：要求私下交易是典型诈骗手法".into(),
            },
            FraudState {
                id: "S3".into(),
                name: "催促付款".into(),
                risk_score: 0.8,
                action_suggestion: ActionSuggestion::Block,
                warning_text: "高危弹窗：私下交易无保障，切勿先付定金".into(),
            },
            FraudState {
                id: "S4".into(),
                name: "失联".into(),
                risk_score: 1.0,
                action_suggestion: ActionSuggestion::Block,
                warning_text: "全屏阻断：诈骗已发生，请立即报警拨打96110".into(),
            },
        ],
        transitions: vec![
            StateTransition {
                from_state: "S0".into(),
                to_state: "S1".into(),
                trigger_keywords: vec![
                    "限时抢购".into(),
                    "亏本清仓".into(),
                    "厂家直销".into(),
                    "一折起".into(),
                    "低价".into(),
                    "秒杀".into(),
                ],
            },
            StateTransition {
                from_state: "S1".into(),
                to_state: "S2".into(),
                trigger_keywords: vec![
                    "加微信聊".into(),
                    "私下交易".into(),
                    "平台手续费太高".into(),
                    "走第三方".into(),
                ],
            },
            StateTransition {
                from_state: "S2".into(),
                to_state: "S3".into(),
                trigger_keywords: vec![
                    "定金留货".into(),
                    "马上被人抢了".into(),
                    "先转定金".into(),
                    "先付款".into(),
                ],
            },
            StateTransition {
                from_state: "S3".into(),
                to_state: "S4".into(),
                trigger_keywords: vec![
                    "系统故障".into(),
                    "物流延迟".into(),
                    "拉黑".into(),
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
        let mut sm = create_fake_shopping_machine();
        assert!(sm.transition("限时抢购，亏本清仓"));
        assert_eq!(sm.current_state().id, "S1");

        assert!(sm.transition("加微信聊吧，私下交易更优惠"));
        assert_eq!(sm.current_state().id, "S2");

        assert!(sm.transition("先转定金留货，马上被人抢了"));
        assert_eq!(sm.current_state().id, "S3");

        assert!(sm.transition("系统故障，物流延迟"));
        assert_eq!(sm.current_state().id, "S4");
        assert_eq!(sm.risk_score(), 1.0);
    }

    #[test]
    fn test_no_match_normal_chat() {
        let mut sm = create_fake_shopping_machine();
        assert!(!sm.transition("今天吃什么"));
        assert_eq!(sm.current_state().id, "S0");
    }

    #[test]
    fn test_risk_at_s3() {
        let mut sm = create_fake_shopping_machine();
        sm.transition("亏本清仓一折起");
        sm.transition("私下交易");
        sm.transition("先转定金留货");
        assert_eq!(sm.risk_score(), 0.8);
        assert_eq!(sm.action_suggestion(), ActionSuggestion::Block);
    }
}
