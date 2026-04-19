use super::{
    ActionSuggestion, FraudState, FraudStateMachineDef, FraudType,
    StateTransition, GenericStateMachine,
};

pub fn create_fake_police_machine() -> GenericStateMachine {
    let definition = FraudStateMachineDef {
        fraud_type: FraudType::FakePolice,
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
                name: "身份冒用".into(),
                risk_score: 0.4,
                action_suggestion: ActionSuggestion::Monitor,
                warning_text: "轻提示：公安机关不会通过电话办案".into(),
            },
            FraudState {
                id: "S2".into(),
                name: "罪名指控".into(),
                risk_score: 0.6,
                action_suggestion: ActionSuggestion::Warn,
                warning_text: "候选词区警告：公检法不会电话通知涉嫌犯罪".into(),
            },
            FraudState {
                id: "S3".into(),
                name: "恐吓操控".into(),
                risk_score: 0.8,
                action_suggestion: ActionSuggestion::Block,
                warning_text: "高危弹窗：要求保密和隔离是操控手段".into(),
            },
            FraudState {
                id: "S4".into(),
                name: "诱导转账".into(),
                risk_score: 1.0,
                action_suggestion: ActionSuggestion::Block,
                warning_text: "全屏阻断+一键拨打96110：公检法不存在安全账户！".into(),
            },
        ],
        transitions: vec![
            StateTransition {
                from_state: "S0".into(),
                to_state: "S1".into(),
                trigger_keywords: vec![
                    "公安局".into(),
                    "检察院".into(),
                    "法院".into(),
                    "我是XX警官".into(),
                ],
            },
            StateTransition {
                from_state: "S1".into(),
                to_state: "S2".into(),
                trigger_keywords: vec![
                    "涉嫌洗钱".into(),
                    "通缉令".into(),
                    "身份被盗用".into(),
                ],
            },
            StateTransition {
                from_state: "S2".into(),
                to_state: "S3".into(),
                trigger_keywords: vec![
                    "保密".into(),
                    "不要告诉家人".into(),
                    "影响子女".into(),
                ],
            },
            StateTransition {
                from_state: "S3".into(),
                to_state: "S4".into(),
                trigger_keywords: vec![
                    "安全账户".into(),
                    "资金核查".into(),
                    "保证金".into(),
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
        let mut sm = create_fake_police_machine();
        assert!(sm.transition("我是公安局XX警官"));
        assert!(sm.transition("你涉嫌洗钱，已有通缉令"));
        assert!(sm.transition("保密，不要告诉家人，影响子女前途"));
        assert!(sm.transition("把钱转入安全账户进行资金核查"));
        assert_eq!(sm.risk_score(), 1.0);
        assert!(sm.warning_text().contains("96110"));
    }

    #[test]
    fn test_partial() {
        let mut sm = create_fake_police_machine();
        sm.transition("检察院来电");
        sm.transition("涉嫌洗钱");
        assert_eq!(sm.risk_score(), 0.6);
    }

    #[test]
    fn test_no_match() {
        let mut sm = create_fake_police_machine();
        assert!(!sm.transition("今天去派出所办身份证"));
        assert_eq!(sm.current_state().id, "S0");
    }
}
