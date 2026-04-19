use super::{
    ActionSuggestion, FraudState, FraudStateMachineDef, FraudType,
    StateTransition, GenericStateMachine,
};

pub fn create_romance_machine() -> GenericStateMachine {
    let definition = FraudStateMachineDef {
        fraud_type: FraudType::Romance,
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
                name: "搭讪接触".into(),
                risk_score: 0.1,
                action_suggestion: ActionSuggestion::Monitor,
                warning_text: "轻提示：注意网络交友安全".into(),
            },
            FraudState {
                id: "S2".into(),
                name: "情感升温".into(),
                risk_score: 0.3,
                action_suggestion: ActionSuggestion::Monitor,
                warning_text: "轻提示：网络交友需谨慎，注意对方真实身份".into(),
            },
            FraudState {
                id: "S3".into(),
                name: "身份包装".into(),
                risk_score: 0.4,
                action_suggestion: ActionSuggestion::Warn,
                warning_text: "候选词区警告：注意核实对方声称的身份".into(),
            },
            FraudState {
                id: "S4".into(),
                name: "透露商机".into(),
                risk_score: 0.6,
                action_suggestion: ActionSuggestion::Warn,
                warning_text: "浮层警告：恋人推荐投资是杀猪盘典型手法".into(),
            },
            FraudState {
                id: "S5".into(),
                name: "晒收益诱导".into(),
                risk_score: 0.7,
                action_suggestion: ActionSuggestion::Warn,
                warning_text: "浮层警告：收益截图可伪造，切勿跟投".into(),
            },
            FraudState {
                id: "S6".into(),
                name: "诱导投资".into(),
                risk_score: 0.9,
                action_suggestion: ActionSuggestion::Block,
                warning_text: "高危弹窗：检测到虚假投资平台诱导，立即停止充值".into(),
            },
            FraudState {
                id: "S7".into(),
                name: "无法提现".into(),
                risk_score: 1.0,
                action_suggestion: ActionSuggestion::Block,
                warning_text: "全屏阻断：杀猪盘最后阶段，不要再充值！拨打96110".into(),
            },
        ],
        transitions: vec![
            StateTransition {
                from_state: "S0".into(),
                to_state: "S1".into(),
                trigger_keywords: vec![
                    "认识一下".into(),
                    "缘分".into(),
                    "你单身吗".into(),
                ],
            },
            StateTransition {
                from_state: "S1".into(),
                to_state: "S2".into(),
                trigger_keywords: vec![
                    "亲爱的".into(),
                    "宝贝".into(),
                    "想你了".into(),
                ],
            },
            StateTransition {
                from_state: "S2".into(),
                to_state: "S3".into(),
                trigger_keywords: vec![
                    "军人".into(),
                    "医生".into(),
                    "海外华人".into(),
                    "成功人士".into(),
                ],
            },
            StateTransition {
                from_state: "S3".into(),
                to_state: "S4".into(),
                trigger_keywords: vec![
                    "有内幕".into(),
                    "发现平台漏洞".into(),
                    "带你赚钱".into(),
                ],
            },
            StateTransition {
                from_state: "S4".into(),
                to_state: "S5".into(),
                trigger_keywords: vec![
                    "今天又赚了".into(),
                    "截图给你看".into(),
                ],
            },
            StateTransition {
                from_state: "S5".into(),
                to_state: "S6".into(),
                trigger_keywords: vec![
                    "下载APP".into(),
                    "充值".into(),
                    "跟投".into(),
                ],
            },
            StateTransition {
                from_state: "S6".into(),
                to_state: "S7".into(),
                trigger_keywords: vec![
                    "账户冻结".into(),
                    "需要交税".into(),
                    "再充一笔".into(),
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
        let mut sm = create_romance_machine();
        assert!(sm.transition("你好，认识一下，你单身吗"));
        assert_eq!(sm.current_state().id, "S1");

        assert!(sm.transition("亲爱的，想你了"));
        assert_eq!(sm.current_state().id, "S2");

        assert!(sm.transition("我是军人，驻扎海外"));
        assert_eq!(sm.current_state().id, "S3");

        assert!(sm.transition("我发现平台漏洞，带你赚钱"));
        assert_eq!(sm.current_state().id, "S4");

        assert!(sm.transition("今天又赚了5万，截图给你看"));
        assert_eq!(sm.current_state().id, "S5");

        assert!(sm.transition("下载APP充值跟投"));
        assert_eq!(sm.current_state().id, "S6");

        assert!(sm.transition("账户冻结了，再充一笔才能提现"));
        assert_eq!(sm.current_state().id, "S7");
        assert_eq!(sm.risk_score(), 1.0);
    }

    #[test]
    fn test_partial_at_s4() {
        let mut sm = create_romance_machine();
        sm.transition("认识一下");
        sm.transition("亲爱的宝贝");
        sm.transition("我是海外华人成功人士");
        sm.transition("有内幕消息带你赚钱");
        assert_eq!(sm.risk_score(), 0.6);
        assert_eq!(sm.action_suggestion(), ActionSuggestion::Warn);
    }

    #[test]
    fn test_no_match() {
        let mut sm = create_romance_machine();
        assert!(!sm.transition("今天天气真好"));
        assert_eq!(sm.current_state().id, "S0");
    }
}
