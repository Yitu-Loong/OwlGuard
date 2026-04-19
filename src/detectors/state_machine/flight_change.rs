use super::{
    ActionSuggestion, FraudState, FraudStateMachineDef, FraudType,
    StateTransition, GenericStateMachine,
};

pub fn create_flight_change_machine() -> GenericStateMachine {
    let definition = FraudStateMachineDef {
        fraud_type: FraudType::FlightChange,
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
                name: "航班异常".into(),
                risk_score: 0.3,
                action_suggestion: ActionSuggestion::Monitor,
                warning_text: "轻提示：请通过航空公司官方渠道核实航班信息".into(),
            },
            FraudState {
                id: "S2".into(),
                name: "诱导操作".into(),
                risk_score: 0.7,
                action_suggestion: ActionSuggestion::Warn,
                warning_text: "浮层警告：退改签应通过官方APP，勿点击陌生链接".into(),
            },
            FraudState {
                id: "S3".into(),
                name: "索要信息".into(),
                risk_score: 1.0,
                action_suggestion: ActionSuggestion::Block,
                warning_text: "全屏阻断：航班诈骗最后阶段！拨打96110求助".into(),
            },
        ],
        transitions: vec![
            StateTransition {
                from_state: "S0".into(),
                to_state: "S1".into(),
                trigger_keywords: vec![
                    "航班取消".into(),
                    "航班延误".into(),
                ],
            },
            StateTransition {
                from_state: "S1".into(),
                to_state: "S2".into(),
                trigger_keywords: vec![
                    "办理理赔".into(),
                    "点击链接".into(),
                ],
            },
            StateTransition {
                from_state: "S2".into(),
                to_state: "S3".into(),
                trigger_keywords: vec![
                    "银行卡号".into(),
                    "验证码".into(),
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
        let mut sm = create_flight_change_machine();
        assert!(sm.transition("您的航班取消，请联系客服"));
        assert!(sm.transition("点击链接办理理赔"));
        assert!(sm.transition("请提供银行卡号和验证码"));
        assert_eq!(sm.risk_score(), 1.0);
    }

    #[test]
    fn test_partial() {
        let mut sm = create_flight_change_machine();
        sm.transition("航班延误了");
        assert_eq!(sm.risk_score(), 0.3);
    }

    #[test]
    fn test_no_match() {
        let mut sm = create_flight_change_machine();
        assert!(!sm.transition("我订了明天的机票"));
        assert_eq!(sm.current_state().id, "S0");
    }
}
