use super::{
    ActionSuggestion, FraudState, FraudStateMachineDef, FraudType,
    StateTransition, GenericStateMachine,
};

pub fn create_fake_customer_machine() -> GenericStateMachine {
    let definition = FraudStateMachineDef {
        fraud_type: FraudType::FakeCustomer,
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
                name: "身份冒充".into(),
                risk_score: 0.3,
                action_suggestion: ActionSuggestion::Monitor,
                warning_text: "轻提示：注意核实客服身份".into(),
            },
            FraudState {
                id: "S2".into(),
                name: "问题制造".into(),
                risk_score: 0.5,
                action_suggestion: ActionSuggestion::Warn,
                warning_text: "候选词区警告：正规客服不会私下联系要求操作".into(),
            },
            FraudState {
                id: "S3".into(),
                name: "诱导操作".into(),
                risk_score: 0.7,
                action_suggestion: ActionSuggestion::Warn,
                warning_text: "浮层警告：退款理赔应通过官方渠道，勿扫码操作".into(),
            },
            FraudState {
                id: "S4".into(),
                name: "屏幕共享".into(),
                risk_score: 0.9,
                action_suggestion: ActionSuggestion::Block,
                warning_text: "高危弹窗：客服不会要求屏幕共享！立即停止".into(),
            },
            FraudState {
                id: "S5".into(),
                name: "索要验证码".into(),
                risk_score: 1.0,
                action_suggestion: ActionSuggestion::Block,
                warning_text: "全屏阻断：验证码是最后防线，绝不可告知他人！拨打96110".into(),
            },
        ],
        transitions: vec![
            StateTransition {
                from_state: "S0".into(),
                to_state: "S1".into(),
                trigger_keywords: vec![
                    "淘宝客服".into(),
                    "京东客服".into(),
                    "拼多多客服".into(),
                    "快递客服".into(),
                ],
            },
            StateTransition {
                from_state: "S1".into(),
                to_state: "S2".into(),
                trigger_keywords: vec![
                    "商品质量问题".into(),
                    "快递丢失".into(),
                    "订单异常".into(),
                ],
            },
            StateTransition {
                from_state: "S2".into(),
                to_state: "S3".into(),
                trigger_keywords: vec![
                    "需要退款".into(),
                    "打开备用金".into(),
                    "扫码退款".into(),
                    "理赔".into(),
                ],
            },
            StateTransition {
                from_state: "S3".into(),
                to_state: "S4".into(),
                trigger_keywords: vec![
                    "下载会议".into(),
                    "屏幕共享".into(),
                    "远程协助".into(),
                    "腾讯会议".into(),
                ],
            },
            StateTransition {
                from_state: "S4".into(),
                to_state: "S5".into(),
                trigger_keywords: vec![
                    "需要验证码确认".into(),
                    "动态码发一下".into(),
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
        let mut sm = create_fake_customer_machine();
        assert!(sm.transition("您好，我是淘宝客服"));
        assert!(sm.transition("您的商品质量问题需要处理"));
        assert!(sm.transition("需要退款，请扫码退款办理理赔"));
        assert!(sm.transition("请下载会议软件，屏幕共享协助操作"));
        assert!(sm.transition("需要验证码确认身份"));
        assert_eq!(sm.risk_score(), 1.0);
        assert_eq!(sm.action_suggestion(), ActionSuggestion::Block);
    }

    #[test]
    fn test_stops_at_s3() {
        let mut sm = create_fake_customer_machine();
        sm.transition("我是京东客服");
        sm.transition("快递丢失了");
        sm.transition("需要退款理赔");
        assert_eq!(sm.risk_score(), 0.7);
    }

    #[test]
    fn test_no_false_positive() {
        let mut sm = create_fake_customer_machine();
        assert!(!sm.transition("今天快递到了"));
        assert_eq!(sm.current_state().id, "S0");
    }
}
