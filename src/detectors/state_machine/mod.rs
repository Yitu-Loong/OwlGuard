pub mod brushing;
pub mod fake_customer;
pub mod fake_leader;
pub mod fake_police;
pub mod fake_shopping;
pub mod flight_change;
pub mod game_trade;
pub mod investment;
pub mod loan_credit;
pub mod romance;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FraudType {
    Brushing,
    Investment,
    FakeShopping,
    FakeCustomer,
    LoanCredit,
    FakeLeader,
    FakePolice,
    Romance,
    GameTrade,
    FlightChange,
}

impl FraudType {
    pub fn id(&self) -> &'static str {
        match self {
            FraudType::Brushing => "F001",
            FraudType::Investment => "F002",
            FraudType::FakeShopping => "F003",
            FraudType::FakeCustomer => "F004",
            FraudType::LoanCredit => "F005",
            FraudType::FakeLeader => "F006",
            FraudType::FakePolice => "F007",
            FraudType::Romance => "F008",
            FraudType::GameTrade => "F009",
            FraudType::FlightChange => "F010",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            FraudType::Brushing => "刷单返利",
            FraudType::Investment => "虚假投资理财",
            FraudType::FakeShopping => "虚假购物服务",
            FraudType::FakeCustomer => "冒充电商客服",
            FraudType::LoanCredit => "贷款征信",
            FraudType::FakeLeader => "冒充领导熟人",
            FraudType::FakePolice => "冒充公检法",
            FraudType::Romance => "婚恋交友(杀猪盘)",
            FraudType::GameTrade => "游戏虚假交易",
            FraudType::FlightChange => "机票退改签",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlertLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

impl AlertLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertLevel::None => "none",
            AlertLevel::Low => "low",
            AlertLevel::Medium => "medium",
            AlertLevel::High => "high",
            AlertLevel::Critical => "critical",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionSuggestion {
    None,
    Monitor,
    Warn,
    Block,
}

impl ActionSuggestion {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActionSuggestion::None => "none",
            ActionSuggestion::Monitor => "monitor",
            ActionSuggestion::Warn => "warn",
            ActionSuggestion::Block => "block",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudState {
    pub id: String,
    pub name: String,
    pub risk_score: f64,
    pub action_suggestion: ActionSuggestion,
    pub warning_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    pub from_state: String,
    pub to_state: String,
    pub trigger_keywords: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudStateMachineDef {
    pub fraud_type: FraudType,
    pub states: Vec<FraudState>,
    pub transitions: Vec<StateTransition>,
}

pub trait FraudStateMachine: Send + Sync {
    fn fraud_type(&self) -> FraudType;
    fn current_state(&self) -> &FraudState;
    fn current_state_index(&self) -> usize;
    fn risk_score(&self) -> f64;
    fn action_suggestion(&self) -> ActionSuggestion;
    fn warning_text(&self) -> String;
    fn matched_keywords(&self) -> &[String];
    fn transition(&mut self, text: &str) -> bool;
    fn reset(&mut self);
    fn reasoning(&self) -> String;
}

pub struct GenericStateMachine {
    definition: FraudStateMachineDef,
    current_index: usize,
    matched_keywords: Vec<String>,
    state_history: Vec<usize>,
}

impl GenericStateMachine {
    pub fn new(definition: FraudStateMachineDef) -> Self {
        Self {
            current_index: 0,
            matched_keywords: Vec::new(),
            state_history: vec![0],
            definition,
        }
    }

    fn find_transition(&self, from_state: &str) -> Vec<&StateTransition> {
        self.definition
            .transitions
            .iter()
            .filter(|t| t.from_state == from_state)
            .collect()
    }

    fn check_keywords_match(text: &str, keywords: &[String]) -> Vec<String> {
        let lower = text.to_lowercase();
        keywords
            .iter()
            .filter(|kw| lower.contains(&kw.to_lowercase()))
            .cloned()
            .collect()
    }
}

impl FraudStateMachine for GenericStateMachine {
    fn fraud_type(&self) -> FraudType {
        self.definition.fraud_type
    }

    fn current_state(&self) -> &FraudState {
        &self.definition.states[self.current_index]
    }

    fn current_state_index(&self) -> usize {
        self.current_index
    }

    fn risk_score(&self) -> f64 {
        self.definition.states[self.current_index].risk_score
    }

    fn action_suggestion(&self) -> ActionSuggestion {
        self.definition.states[self.current_index].action_suggestion
    }

    fn warning_text(&self) -> String {
        self.definition.states[self.current_index].warning_text.clone()
    }

    fn matched_keywords(&self) -> &[String] {
        &self.matched_keywords
    }

    fn transition(&mut self, text: &str) -> bool {
        let current_id = &self.definition.states[self.current_index].id;
        let possible_transitions = self.find_transition(current_id);

        for trans in possible_transitions {
            let matched = Self::check_keywords_match(text, &trans.trigger_keywords);
            if !matched.is_empty() {
                if let Some(next_index) = self
                    .definition
                    .states
                    .iter()
                    .position(|s| s.id == trans.to_state)
                {
                    if next_index > self.current_index {
                        self.matched_keywords.extend(matched);
                        self.current_index = next_index;
                        self.state_history.push(next_index);
                        return true;
                    }
                }
            }
        }
        false
    }

    fn reset(&mut self) {
        self.current_index = 0;
        self.matched_keywords.clear();
        self.state_history.clear();
        self.state_history.push(0);
    }

    fn reasoning(&self) -> String {
        let history: Vec<String> = self
            .state_history
            .iter()
            .map(|&i| {
                let state = &self.definition.states[i];
                format!("{}({})", state.id, state.name)
            })
            .collect();
        format!(
            "{}话术状态机路径: {}",
            self.definition.fraud_type.name(),
            history.join(" → ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fraud_type_id_and_name() {
        assert_eq!(FraudType::Brushing.id(), "F001");
        assert_eq!(FraudType::Brushing.name(), "刷单返利");
        assert_eq!(FraudType::FlightChange.id(), "F010");
        assert_eq!(FraudType::FlightChange.name(), "机票退改签");
    }

    #[test]
    fn test_alert_level_as_str() {
        assert_eq!(AlertLevel::None.as_str(), "none");
        assert_eq!(AlertLevel::Critical.as_str(), "critical");
    }

    #[test]
    fn test_action_suggestion_as_str() {
        assert_eq!(ActionSuggestion::Block.as_str(), "block");
    }

    #[test]
    fn test_generic_state_machine_basic() {
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
                    warning_text: "警惕刷单诈骗".into(),
                },
                FraudState {
                    id: "S2".into(),
                    name: "信任建立".into(),
                    risk_score: 0.4,
                    action_suggestion: ActionSuggestion::Warn,
                    warning_text: "候选词区变色警告".into(),
                },
            ],
            transitions: vec![
                StateTransition {
                    from_state: "S0".into(),
                    to_state: "S1".into(),
                    trigger_keywords: vec!["兼职".into(), "日结".into(), "宝妈可做".into()],
                },
                StateTransition {
                    from_state: "S1".into(),
                    to_state: "S2".into(),
                    trigger_keywords: vec!["正规平台".into(), "不交押金".into(), "有营业执照".into()],
                },
            ],
        };

        let mut sm = GenericStateMachine::new(definition);
        assert_eq!(sm.current_state().id, "S0");
        assert_eq!(sm.risk_score(), 0.0);

        let transitioned = sm.transition("手机兼职，日结300-500");
        assert!(transitioned);
        assert_eq!(sm.current_state().id, "S1");
        assert_eq!(sm.risk_score(), 0.2);

        let transitioned = sm.transition("我们是正规平台，有营业执照");
        assert!(transitioned);
        assert_eq!(sm.current_state().id, "S2");
        assert_eq!(sm.risk_score(), 0.4);
    }

    #[test]
    fn test_state_not_reversible() {
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
                    name: "诱饵".into(),
                    risk_score: 0.2,
                    action_suggestion: ActionSuggestion::Monitor,
                    warning_text: "警告".into(),
                },
            ],
            transitions: vec![
                StateTransition {
                    from_state: "S0".into(),
                    to_state: "S1".into(),
                    trigger_keywords: vec!["兼职".into()],
                },
                StateTransition {
                    from_state: "S1".into(),
                    to_state: "S0".into(),
                    trigger_keywords: vec!["正常对话".into()],
                },
            ],
        };

        let mut sm = GenericStateMachine::new(definition);
        sm.transition("有兼职吗");
        assert_eq!(sm.current_state().id, "S1");

        let transitioned = sm.transition("正常对话");
        assert!(!transitioned);
        assert_eq!(sm.current_state().id, "S1");
    }

    #[test]
    fn test_reset() {
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
                    name: "诱饵".into(),
                    risk_score: 0.2,
                    action_suggestion: ActionSuggestion::Monitor,
                    warning_text: "警告".into(),
                },
            ],
            transitions: vec![StateTransition {
                from_state: "S0".into(),
                to_state: "S1".into(),
                trigger_keywords: vec!["兼职".into()],
            }],
        };

        let mut sm = GenericStateMachine::new(definition);
        sm.transition("有兼职吗");
        assert_eq!(sm.current_state().id, "S1");

        sm.reset();
        assert_eq!(sm.current_state().id, "S0");
        assert!(sm.matched_keywords().is_empty());
    }

    #[test]
    fn test_reasoning() {
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
                    warning_text: "警告".into(),
                },
            ],
            transitions: vec![StateTransition {
                from_state: "S0".into(),
                to_state: "S1".into(),
                trigger_keywords: vec!["兼职".into()],
            }],
        };

        let mut sm = GenericStateMachine::new(definition);
        sm.transition("有兼职吗");
        let reasoning = sm.reasoning();
        assert!(reasoning.contains("刷单返利"));
        assert!(reasoning.contains("S0(初始)"));
        assert!(reasoning.contains("S1(诱饵抛出)"));
    }
}
