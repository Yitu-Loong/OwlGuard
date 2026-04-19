use crate::detectors::state_machine::{
    ActionSuggestion, FraudState, FraudStateMachineDef, FraudType, StateTransition,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesVersion {
    pub version: String,
    pub update_notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudRuleFile {
    pub version: String,
    pub fraud_types: Vec<FraudRuleEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudRuleEntry {
    pub fraud_type_id: String,
    pub fraud_type_name: String,
    pub states: Vec<FraudStateEntry>,
    pub transitions: Vec<TransitionEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudStateEntry {
    pub id: String,
    pub name: String,
    pub risk_score: f64,
    pub action_suggestion: String,
    pub warning_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionEntry {
    pub from_state: String,
    pub to_state: String,
    pub trigger_keywords: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivePatternFile {
    pub version: String,
    pub patterns: Vec<SensitivePatternEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivePatternEntry {
    pub name: String,
    pub regex: String,
    pub context_keywords: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlacklistFile {
    pub version: String,
    pub items: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum RuleLoadError {
    #[error("规则文件不存在: {0}")]
    FileNotFound(String),
    #[error("规则文件读取失败: {0}")]
    ReadError(String),
    #[error("规则文件解析失败: {0}")]
    ParseError(String),
}

pub fn load_fraud_rules(path: &Path) -> Result<FraudRuleFile, RuleLoadError> {
    if !path.exists() {
        return Err(RuleLoadError::FileNotFound(path.display().to_string()));
    }
    let content = std::fs::read_to_string(path)
        .map_err(|e| RuleLoadError::ReadError(e.to_string()))?;
    serde_json::from_str(&content).map_err(|e| RuleLoadError::ParseError(e.to_string()))
}

pub fn load_sensitive_patterns(path: &Path) -> Result<SensitivePatternFile, RuleLoadError> {
    if !path.exists() {
        return Err(RuleLoadError::FileNotFound(path.display().to_string()));
    }
    let content = std::fs::read_to_string(path)
        .map_err(|e| RuleLoadError::ReadError(e.to_string()))?;
    serde_json::from_str(&content).map_err(|e| RuleLoadError::ParseError(e.to_string()))
}

pub fn load_blacklist(path: &Path) -> Result<HashSet<String>, RuleLoadError> {
    if !path.exists() {
        return Err(RuleLoadError::FileNotFound(path.display().to_string()));
    }
    let content = std::fs::read_to_string(path)
        .map_err(|e| RuleLoadError::ReadError(e.to_string()))?;
    let file: BlacklistFile =
        serde_json::from_str(&content).map_err(|e| RuleLoadError::ParseError(e.to_string()))?;
    Ok(file.items.into_iter().collect())
}

fn parse_action_suggestion(s: &str) -> ActionSuggestion {
    match s {
        "none" => ActionSuggestion::None,
        "monitor" => ActionSuggestion::Monitor,
        "warn" => ActionSuggestion::Warn,
        "block" => ActionSuggestion::Block,
        _ => ActionSuggestion::None,
    }
}

fn parse_fraud_type(id: &str) -> Option<FraudType> {
    match id {
        "F001" => Some(FraudType::Brushing),
        "F002" => Some(FraudType::Investment),
        "F003" => Some(FraudType::FakeShopping),
        "F004" => Some(FraudType::FakeCustomer),
        "F005" => Some(FraudType::LoanCredit),
        "F006" => Some(FraudType::FakeLeader),
        "F007" => Some(FraudType::FakePolice),
        "F008" => Some(FraudType::Romance),
        "F009" => Some(FraudType::GameTrade),
        "F010" => Some(FraudType::FlightChange),
        _ => None,
    }
}

pub fn convert_rule_to_definition(entry: &FraudRuleEntry) -> Option<FraudStateMachineDef> {
    let fraud_type = parse_fraud_type(&entry.fraud_type_id)?;
    let states: Vec<FraudState> = entry
        .states
        .iter()
        .map(|s| FraudState {
            id: s.id.clone(),
            name: s.name.clone(),
            risk_score: s.risk_score,
            action_suggestion: parse_action_suggestion(&s.action_suggestion),
            warning_text: s.warning_text.clone(),
        })
        .collect();
    let transitions: Vec<StateTransition> = entry
        .transitions
        .iter()
        .map(|t| StateTransition {
            from_state: t.from_state.clone(),
            to_state: t.to_state.clone(),
            trigger_keywords: t.trigger_keywords.clone(),
        })
        .collect();
    Some(FraudStateMachineDef {
        fraud_type,
        states,
        transitions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_missing_file() {
        let result = load_fraud_rules(Path::new("nonexistent.json"));
        assert!(matches!(result, Err(RuleLoadError::FileNotFound(_))));
    }

    #[test]
    fn test_parse_action_suggestion() {
        assert_eq!(parse_action_suggestion("none"), ActionSuggestion::None);
        assert_eq!(parse_action_suggestion("monitor"), ActionSuggestion::Monitor);
        assert_eq!(parse_action_suggestion("warn"), ActionSuggestion::Warn);
        assert_eq!(parse_action_suggestion("block"), ActionSuggestion::Block);
    }

    #[test]
    fn test_parse_fraud_type() {
        assert_eq!(parse_fraud_type("F001"), Some(FraudType::Brushing));
        assert_eq!(parse_fraud_type("F010"), Some(FraudType::FlightChange));
        assert_eq!(parse_fraud_type("F999"), None);
    }

    #[test]
    fn test_convert_rule_to_definition() {
        let entry = FraudRuleEntry {
            fraud_type_id: "F001".into(),
            fraud_type_name: "刷单返利".into(),
            states: vec![
                FraudStateEntry {
                    id: "S0".into(),
                    name: "初始".into(),
                    risk_score: 0.0,
                    action_suggestion: "none".into(),
                    warning_text: String::new(),
                },
            ],
            transitions: vec![],
        };
        let def = convert_rule_to_definition(&entry);
        assert!(def.is_some());
        assert_eq!(def.unwrap().fraud_type, FraudType::Brushing);
    }

    #[test]
    fn test_load_blacklist_missing() {
        let result = load_blacklist(Path::new("nonexistent.json"));
        assert!(matches!(result, Err(RuleLoadError::FileNotFound(_))));
    }
}
