use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::detectors::orchestrator::ScanOrchestrator;
use std::sync::LazyLock;

static ORCHESTRATOR: LazyLock<ScanOrchestrator> = LazyLock::new(ScanOrchestrator::new);

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ChatMessage {
    #[schemars(description = "消息角色：user（用户）或 other（对方）")]
    pub role: String,
    #[schemars(description = "消息文本内容")]
    pub content: String,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ScanConversationRequest {
    #[schemars(description = "对话消息数组，每条消息包含 role 和 content")]
    pub conversation: Vec<ChatMessage>,
    #[schemars(description = "可选，上下文提示（如\"转账场景\"、\"客服场景\"等）")]
    pub context_hint: Option<String>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct DetectSensitiveRequest {
    #[schemars(description = "要检测的文本字符串")]
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct CheckUrlRequest {
    #[schemars(description = "要检测的URL地址")]
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct CheckAppRequest {
    #[schemars(description = "要检测的APP名称")]
    pub app_name: String,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct BatchScanRequest {
    #[schemars(description = "多组对话数组，每组为一个独立对话")]
    pub conversations: Vec<Vec<ChatMessage>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ScanResult {
    pub risk_score: f64,
    pub risk_level: String,
    pub fraud_type: Option<String>,
    pub fraud_type_id: Option<String>,
    pub current_state: Option<String>,
    pub matched_keywords: Vec<String>,
    pub action_suggestion: String,
    pub warning_text: String,
    pub reasoning: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SensitiveResult {
    pub found: bool,
    pub sensitive_items: Vec<SensitiveItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SensitiveItem {
    pub sensitive_type: String,
    pub value: String,
    pub position: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UrlCheckResult {
    pub is_malicious: bool,
    pub threat_type: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AppCheckResult {
    pub is_dangerous: bool,
    pub risk_reason: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RulesVersionResult {
    pub version: String,
    pub update_notes: String,
}

pub fn handle_scan_conversation(req: ScanConversationRequest) -> String {
    let result = ORCHESTRATOR.scan_conversation(&req.conversation);
    serde_json::to_string(&result).unwrap_or_else(|e| format!("{{\"error\": \"{e}\"}}"))
}

pub fn handle_detect_sensitive(req: DetectSensitiveRequest) -> String {
    let result = ORCHESTRATOR.detect_sensitive(&req.text);
    serde_json::to_string(&result).unwrap_or_else(|e| format!("{{\"error\": \"{e}\"}}"))
}

pub fn handle_check_url(req: CheckUrlRequest) -> String {
    let result = ORCHESTRATOR.check_url(&req.url);
    serde_json::to_string(&result).unwrap_or_else(|e| format!("{{\"error\": \"{e}\"}}"))
}

pub fn handle_check_app(req: CheckAppRequest) -> String {
    let result = ORCHESTRATOR.check_app(&req.app_name);
    serde_json::to_string(&result).unwrap_or_else(|e| format!("{{\"error\": \"{e}\"}}"))
}

pub fn handle_get_rules_version() -> String {
    let result = RulesVersionResult {
        version: "0.1.0".to_string(),
        update_notes: "MVP版本：10类诈骗话术状态机 + 敏感信息检测".to_string(),
    };
    serde_json::to_string(&result).unwrap_or_else(|e| format!("{{\"error\": \"{e}\"}}"))
}

pub fn handle_batch_scan(req: BatchScanRequest) -> String {
    let results: Vec<ScanResult> = req
        .conversations
        .iter()
        .map(|conv| ORCHESTRATOR.scan_conversation(conv))
        .collect();
    serde_json::to_string(&results).unwrap_or_else(|e| format!("{{\"error\": \"{e}\"}}"))
}
