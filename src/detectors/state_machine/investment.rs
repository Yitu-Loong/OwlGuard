use super::{
    ActionSuggestion, FraudState, FraudStateMachineDef, FraudType,
    StateTransition, GenericStateMachine,
};

pub fn create_investment_machine() -> GenericStateMachine {
    let definition = FraudStateMachineDef {
        fraud_type: FraudType::Investment,
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
                name: "引流接触".into(),
                risk_score: 0.2,
                action_suggestion: ActionSuggestion::Monitor,
                warning_text: "轻提示：警惕投资理财诈骗".into(),
            },
            FraudState {
                id: "S2".into(),
                name: "人设包装".into(),
                risk_score: 0.4,
                action_suggestion: ActionSuggestion::Warn,
                warning_text: "候选词区警告：所谓内幕消息和老师带单都是骗局".into(),
            },
            FraudState {
                id: "S3".into(),
                name: "晒单诱导".into(),
                risk_score: 0.6,
                action_suggestion: ActionSuggestion::Warn,
                warning_text: "浮层警告：收益截图可以伪造，切勿轻信".into(),
            },
            FraudState {
                id: "S4".into(),
                name: "诱导注册".into(),
                risk_score: 0.8,
                action_suggestion: ActionSuggestion::Block,
                warning_text: "高危弹窗：检测到诱导下载虚假投资APP，立即停止".into(),
            },
            FraudState {
                id: "S5".into(),
                name: "诱导充值".into(),
                risk_score: 1.0,
                action_suggestion: ActionSuggestion::Block,
                warning_text: "全屏阻断：虚假投资平台充值后无法提现！拨打96110求助".into(),
            },
        ],
        transitions: vec![
            StateTransition {
                from_state: "S0".into(),
                to_state: "S1".into(),
                trigger_keywords: vec![
                    "加群交流".into(),
                    "老师分析".into(),
                    "免费诊股".into(),
                    "导师".into(),
                    "带单".into(),
                ],
            },
            StateTransition {
                from_state: "S1".into(),
                to_state: "S2".into(),
                trigger_keywords: vec![
                    "有内幕消息".into(),
                    "跟着机构操作".into(),
                    "截图收益".into(),
                    "内幕".into(),
                    "稳赚".into(),
                ],
            },
            StateTransition {
                from_state: "S2".into(),
                to_state: "S3".into(),
                trigger_keywords: vec![
                    "今天又赚了".into(),
                    "学员收益截图".into(),
                    "错过等一年".into(),
                ],
            },
            StateTransition {
                from_state: "S3".into(),
                to_state: "S4".into(),
                trigger_keywords: vec![
                    "扫码下载APP".into(),
                    "专属通道".into(),
                    "名额有限".into(),
                    "下载APP".into(),
                ],
            },
            StateTransition {
                from_state: "S4".into(),
                to_state: "S5".into(),
                trigger_keywords: vec![
                    "入金".into(),
                    "充值".into(),
                    "跟上这单".into(),
                    "满仓干".into(),
                    "跟投".into(),
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
    fn test_full_investment_chain() {
        let mut sm = create_investment_machine();
        assert!(sm.transition("加群交流，老师分析股票"));
        assert_eq!(sm.current_state().id, "S1");

        assert!(sm.transition("有内幕消息，跟着机构操作"));
        assert_eq!(sm.current_state().id, "S2");

        assert!(sm.transition("今天又赚了5万，学员收益截图给你看"));
        assert_eq!(sm.current_state().id, "S3");

        assert!(sm.transition("扫码下载APP，名额有限"));
        assert_eq!(sm.current_state().id, "S4");

        assert!(sm.transition("赶紧入金充值，跟上这单"));
        assert_eq!(sm.current_state().id, "S5");
        assert_eq!(sm.risk_score(), 1.0);
        assert_eq!(sm.action_suggestion(), ActionSuggestion::Block);
    }

    #[test]
    fn test_partial_progression() {
        let mut sm = create_investment_machine();
        sm.transition("导师带单，免费诊股");
        assert_eq!(sm.current_state().id, "S1");

        assert!(!sm.transition("今天天气不错"));
        assert_eq!(sm.current_state().id, "S1");
    }

    #[test]
    fn test_risk_escalation() {
        let mut sm = create_investment_machine();
        sm.transition("加群交流");
        assert_eq!(sm.risk_score(), 0.2);

        sm.transition("有内幕消息");
        assert_eq!(sm.risk_score(), 0.4);

        sm.transition("今天又赚了");
        assert_eq!(sm.risk_score(), 0.6);
    }
}
