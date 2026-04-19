use super::{
    ActionSuggestion, FraudState, FraudStateMachineDef, FraudType,
    StateTransition, GenericStateMachine,
};

pub fn create_fake_leader_machine() -> GenericStateMachine {
    let definition = FraudStateMachineDef {
        fraud_type: FraudType::FakeLeader,
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
                name: "身份声明".into(),
                risk_score: 0.3,
                action_suggestion: ActionSuggestion::Monitor,
                warning_text: "轻提示：请通过其他渠道核实对方身份".into(),
            },
            FraudState {
                id: "S2".into(),
                name: "情境铺垫".into(),
                risk_score: 0.5,
                action_suggestion: ActionSuggestion::Warn,
                warning_text: "候选词区警告：领导不会通过新号码要求转账".into(),
            },
            FraudState {
                id: "S3".into(),
                name: "制造紧急".into(),
                risk_score: 0.8,
                action_suggestion: ActionSuggestion::Block,
                warning_text: "高危弹窗：紧急借款是冒充领导诈骗的典型手法".into(),
            },
            FraudState {
                id: "S4".into(),
                name: "催促转账".into(),
                risk_score: 1.0,
                action_suggestion: ActionSuggestion::Block,
                warning_text: "全屏阻断：请当面或电话核实！拨打96110求助".into(),
            },
        ],
        transitions: vec![
            StateTransition {
                from_state: "S0".into(),
                to_state: "S1".into(),
                trigger_keywords: vec![
                    "我是XX".into(),
                    "换号了".into(),
                    "存一下".into(),
                ],
            },
            StateTransition {
                from_state: "S1".into(),
                to_state: "S2".into(),
                trigger_keywords: vec![
                    "不方便接电话".into(),
                    "在开会".into(),
                    "在外地".into(),
                ],
            },
            StateTransition {
                from_state: "S2".into(),
                to_state: "S3".into(),
                trigger_keywords: vec![
                    "急需用钱".into(),
                    "帮我转一笔钱".into(),
                    "事后报销".into(),
                ],
            },
            StateTransition {
                from_state: "S3".into(),
                to_state: "S4".into(),
                trigger_keywords: vec![
                    "账号发给你".into(),
                    "快点".into(),
                    "别耽误".into(),
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
        let mut sm = create_fake_leader_machine();
        assert!(sm.transition("我是王总，换号了存一下"));
        assert!(sm.transition("现在不方便接电话，在开会"));
        assert!(sm.transition("急需用钱，帮我转一笔钱，事后报销"));
        assert!(sm.transition("账号发给你了，快点别耽误"));
        assert_eq!(sm.risk_score(), 1.0);
    }

    #[test]
    fn test_partial() {
        let mut sm = create_fake_leader_machine();
        sm.transition("我是李总，换号了");
        sm.transition("在外地出差");
        assert_eq!(sm.risk_score(), 0.5);
    }

    #[test]
    fn test_no_match() {
        let mut sm = create_fake_leader_machine();
        assert!(!sm.transition("领导今天开会说了什么"));
        assert_eq!(sm.current_state().id, "S0");
    }
}
