use super::{
    ActionSuggestion, FraudState, FraudStateMachineDef, FraudType,
    StateTransition, GenericStateMachine,
};

pub fn create_brushing_machine() -> GenericStateMachine {
    let definition = FraudStateMachineDef {
        fraud_type: FraudType::Brushing,
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
                name: "诱饵抛出".into(),
                risk_score: 0.2,
                action_suggestion: ActionSuggestion::Monitor,
                warning_text: "轻提示：警惕刷单诈骗".into(),
            },
            FraudState {
                id: "S2".into(),
                name: "信任建立".into(),
                risk_score: 0.4,
                action_suggestion: ActionSuggestion::Warn,
                warning_text: "候选词区变色警告：对方正在建立信任，刷单本身就是违法行为".into(),
            },
            FraudState {
                id: "S3".into(),
                name: "小额返利".into(),
                risk_score: 0.6,
                action_suggestion: ActionSuggestion::Warn,
                warning_text: "浮层警告：小额返利是诱饵，切勿继续投入资金".into(),
            },
            FraudState {
                id: "S4".into(),
                name: "诱导垫付".into(),
                risk_score: 0.8,
                action_suggestion: ActionSuggestion::Block,
                warning_text: "高危弹窗阻断：检测到垫付要求，这是刷单诈骗核心特征！立即停止对话".into(),
            },
            FraudState {
                id: "S5".into(),
                name: "连环套".into(),
                risk_score: 1.0,
                action_suggestion: ActionSuggestion::Block,
                warning_text: "全屏红屏+强制暂停：已进入连环诈骗阶段，切勿再转账！拨打96110求助".into(),
            },
        ],
        transitions: vec![
            StateTransition {
                from_state: "S0".into(),
                to_state: "S1".into(),
                trigger_keywords: vec![
                    "兼职".into(),
                    "日结".into(),
                    "宝妈可做".into(),
                    "手机操作".into(),
                    "刷单".into(),
                    "返利".into(),
                    "佣金".into(),
                    "任务单".into(),
                    "派单员".into(),
                    "抢单".into(),
                    "做任务".into(),
                ],
            },
            StateTransition {
                from_state: "S1".into(),
                to_state: "S2".into(),
                trigger_keywords: vec![
                    "正规平台".into(),
                    "有营业执照".into(),
                    "截图给你看".into(),
                    "不交押金".into(),
                    "不收押金".into(),
                ],
            },
            StateTransition {
                from_state: "S2".into(),
                to_state: "S3".into(),
                trigger_keywords: vec![
                    "先做一单试试".into(),
                    "马上返你".into(),
                    "已到账".into(),
                    "截图给你".into(),
                    "小额试单".into(),
                ],
            },
            StateTransition {
                from_state: "S3".into(),
                to_state: "S4".into(),
                trigger_keywords: vec![
                    "垫付".into(),
                    "佣金高".into(),
                    "做完一起返".into(),
                    "需要垫付".into(),
                    "这单佣金高".into(),
                ],
            },
            StateTransition {
                from_state: "S4".into(),
                to_state: "S5".into(),
                trigger_keywords: vec![
                    "系统卡单".into(),
                    "需要补单".into(),
                    "再充一笔才能提现".into(),
                    "账户冻结".into(),
                    "解冻费".into(),
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
    fn test_full_brushing_chain() {
        let mut sm = create_brushing_machine();
        assert_eq!(sm.current_state().id, "S0");

        assert!(sm.transition("你好，想找个兼职吗？手机操作，日结300-500"));
        assert_eq!(sm.current_state().id, "S1");
        assert_eq!(sm.risk_score(), 0.2);

        assert!(sm.transition("不用押金，我们是正规平台，有营业执照的"));
        assert_eq!(sm.current_state().id, "S2");
        assert_eq!(sm.risk_score(), 0.4);

        assert!(sm.transition("可以先做一单试试，做完马上返你"));
        assert_eq!(sm.current_state().id, "S3");
        assert_eq!(sm.risk_score(), 0.6);

        assert!(sm.transition("这单佣金高，需要垫付，做完一起返"));
        assert_eq!(sm.current_state().id, "S4");
        assert_eq!(sm.risk_score(), 0.8);
        assert_eq!(sm.action_suggestion(), ActionSuggestion::Block);

        assert!(sm.transition("系统卡单了，需要补单才能提现"));
        assert_eq!(sm.current_state().id, "S5");
        assert_eq!(sm.risk_score(), 1.0);
    }

    #[test]
    fn test_partial_progression() {
        let mut sm = create_brushing_machine();
        sm.transition("有刷单兼职，日结500");
        assert_eq!(sm.current_state().id, "S1");

        let transitioned = sm.transition("正常聊天内容");
        assert!(!transitioned);
        assert_eq!(sm.current_state().id, "S1");
    }

    #[test]
    fn test_high_risk_at_s4() {
        let mut sm = create_brushing_machine();
        sm.transition("兼职刷单日结");
        sm.transition("正规平台不交押金");
        sm.transition("先做一单试试马上返你");
        sm.transition("需要垫付，这单佣金高");

        assert!(sm.risk_score() >= 0.8);
        assert_eq!(sm.action_suggestion(), ActionSuggestion::Block);
        assert!(sm.warning_text().contains("垫付"));
    }

    #[test]
    fn test_matched_keywords_tracking() {
        let mut sm = create_brushing_machine();
        sm.transition("手机兼职日结300");
        assert!(sm.matched_keywords().contains(&"兼职".to_string()));
        assert!(sm.matched_keywords().contains(&"日结".to_string()));
    }

    #[test]
    fn test_reset() {
        let mut sm = create_brushing_machine();
        sm.transition("兼职刷单");
        sm.transition("正规平台");
        assert_ne!(sm.current_state().id, "S0");

        sm.reset();
        assert_eq!(sm.current_state().id, "S0");
        assert!(sm.matched_keywords().is_empty());
    }
}
