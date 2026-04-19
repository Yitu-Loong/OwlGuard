use owlguard_mcp::mcp::tools::{
    handle_scan_conversation, handle_detect_sensitive, handle_check_url, handle_check_app,
    handle_get_rules_version, handle_batch_scan, BatchScanRequest, ChatMessage,
    CheckAppRequest, CheckUrlRequest, DetectSensitiveRequest, ScanConversationRequest,
    ScanResult, SensitiveResult,
};

fn make_conversation(messages: Vec<(&str, &str)>) -> Vec<ChatMessage> {
    messages
        .into_iter()
        .map(|(role, content)| ChatMessage {
            role: role.to_string(),
            content: content.to_string(),
        })
        .collect()
}

#[test]
fn test_scan_normal_conversation() {
    let req = ScanConversationRequest {
        conversation: make_conversation(vec![
            ("user", "今天天气真好"),
            ("other", "是啊，出去走走吧"),
        ]),
        context_hint: None,
    };
    let result_str = handle_scan_conversation(req);
    let result: ScanResult = serde_json::from_str(&result_str).unwrap();
    assert_eq!(result.risk_level, "none");
    assert_eq!(result.risk_score, 0.0);
}

#[test]
fn test_scan_brushing_fraud_full_chain() {
    let req = ScanConversationRequest {
        conversation: make_conversation(vec![
            ("other", "手机兼职，日结300-500"),
            ("other", "我们是正规平台，不交押金"),
            ("other", "先做一单试试，马上返你"),
            ("other", "这单佣金高，需要垫付"),
            ("other", "系统卡单了，需要补单才能提现"),
        ]),
        context_hint: None,
    };
    let result_str = handle_scan_conversation(req);
    let result: ScanResult = serde_json::from_str(&result_str).unwrap();
    assert!(result.risk_score >= 1.0);
    assert_eq!(result.fraud_type, Some("刷单返利".to_string()));
    assert_eq!(result.action_suggestion, "block");
}

#[test]
fn test_scan_investment_fraud() {
    let req = ScanConversationRequest {
        conversation: make_conversation(vec![
            ("other", "加群交流，老师分析股票"),
            ("other", "有内幕消息，跟着机构操作"),
            ("other", "今天又赚了5万"),
            ("other", "扫码下载APP"),
            ("other", "赶紧入金充值"),
        ]),
        context_hint: None,
    };
    let result_str = handle_scan_conversation(req);
    let result: ScanResult = serde_json::from_str(&result_str).unwrap();
    assert!(result.risk_score >= 0.8);
    assert_eq!(result.fraud_type, Some("虚假投资理财".to_string()));
}

#[test]
fn test_scan_fake_police_fraud() {
    let req = ScanConversationRequest {
        conversation: make_conversation(vec![
            ("other", "我是公安局XX警官"),
            ("other", "你涉嫌洗钱，已有通缉令"),
            ("other", "保密，不要告诉家人"),
            ("other", "把钱转入安全账户进行资金核查"),
        ]),
        context_hint: None,
    };
    let result_str = handle_scan_conversation(req);
    let result: ScanResult = serde_json::from_str(&result_str).unwrap();
    assert!(result.risk_score >= 0.8);
    assert_eq!(result.fraud_type, Some("冒充公检法".to_string()));
}

#[test]
fn test_scan_romance_fraud() {
    let req = ScanConversationRequest {
        conversation: make_conversation(vec![
            ("other", "你好，认识一下"),
            ("other", "亲爱的，想你了"),
            ("other", "我是军人，驻扎海外"),
            ("other", "我发现平台漏洞，带你赚钱"),
            ("other", "今天又赚了5万，截图给你看"),
            ("other", "下载APP充值跟投"),
            ("other", "账户冻结了，再充一笔"),
        ]),
        context_hint: None,
    };
    let result_str = handle_scan_conversation(req);
    let result: ScanResult = serde_json::from_str(&result_str).unwrap();
    assert!(result.risk_score >= 1.0);
    assert_eq!(result.fraud_type, Some("婚恋交友(杀猪盘)".to_string()));
}

#[test]
fn test_detect_sensitive_info() {
    let req = DetectSensitiveRequest {
        text: "验证码是123456，请输入".to_string(),
    };
    let result_str = handle_detect_sensitive(req);
    let result: SensitiveResult = serde_json::from_str(&result_str).unwrap();
    assert!(result.found);
}

#[test]
fn test_check_malicious_url() {
    let req = CheckUrlRequest {
        url: "https://fake-invest.com/download".to_string(),
    };
    let result_str = handle_check_url(req);
    let result: serde_json::Value = serde_json::from_str(&result_str).unwrap();
    assert!(result["is_malicious"].as_bool().unwrap());
}

#[test]
fn test_check_safe_app() {
    let req = CheckAppRequest {
        app_name: "微信".to_string(),
    };
    let result_str = handle_check_app(req);
    let result: serde_json::Value = serde_json::from_str(&result_str).unwrap();
    assert!(!result["is_dangerous"].as_bool().unwrap());
}

#[test]
fn test_get_rules_version() {
    let result_str = handle_get_rules_version();
    let result: serde_json::Value = serde_json::from_str(&result_str).unwrap();
    assert_eq!(result["version"].as_str().unwrap(), "0.1.0");
}

#[test]
fn test_batch_scan() {
    let req = BatchScanRequest {
        conversations: vec![
            make_conversation(vec![("user", "今天天气真好")]),
            make_conversation(vec![("other", "手机兼职日结300"), ("other", "需要垫付")]),
        ],
    };
    let result_str = handle_batch_scan(req);
    let results: Vec<ScanResult> = serde_json::from_str(&result_str).unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].risk_level, "none");
    assert!(results[1].risk_score > 0.0);
}

#[test]
fn test_scan_with_context_hint() {
    let req = ScanConversationRequest {
        conversation: make_conversation(vec![
            ("other", "请提供银行卡号6222021234567894"),
        ]),
        context_hint: Some("转账场景".to_string()),
    };
    let result_str = handle_scan_conversation(req);
    let result: ScanResult = serde_json::from_str(&result_str).unwrap();
    assert!(result.risk_score > 0.0);
}
