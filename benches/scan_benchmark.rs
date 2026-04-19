use criterion::{criterion_group, criterion_main, Criterion};
use owlguard_mcp::detectors::orchestrator::ScanOrchestrator;
use owlguard_mcp::mcp::tools::ChatMessage;

fn bench_scan_conversation(c: &mut Criterion) {
    let orchestrator = ScanOrchestrator::new();
    let messages = vec![
        ChatMessage { role: "other".into(), content: "手机兼职，日结300-500".into() },
        ChatMessage { role: "other".into(), content: "我们是正规平台，不交押金".into() },
        ChatMessage { role: "other".into(), content: "先做一单试试，马上返你".into() },
        ChatMessage { role: "other".into(), content: "这单佣金高，需要垫付".into() },
        ChatMessage { role: "other".into(), content: "系统卡单了，需要补单才能提现".into() },
    ];

    c.bench_function("scan_conversation_brushing", |b| {
        b.iter(|| orchestrator.scan_conversation(&messages))
    });
}

fn bench_scan_normal(c: &mut Criterion) {
    let orchestrator = ScanOrchestrator::new();
    let messages = vec![
        ChatMessage { role: "user".into(), content: "今天天气真好".into() },
        ChatMessage { role: "other".into(), content: "是啊，出去走走吧".into() },
    ];

    c.bench_function("scan_conversation_normal", |b| {
        b.iter(|| orchestrator.scan_conversation(&messages))
    });
}

fn bench_detect_sensitive(c: &mut Criterion) {
    let orchestrator = ScanOrchestrator::new();

    c.bench_function("detect_sensitive", |b| {
        b.iter(|| orchestrator.detect_sensitive("验证码是123456，银行卡号6222021234567894"))
    });
}

criterion_group!(benches, bench_scan_conversation, bench_scan_normal, bench_detect_sensitive);
criterion_main!(benches);
