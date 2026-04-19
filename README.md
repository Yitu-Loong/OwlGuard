# 枭卫（OwlGuard）MCP

> 开源反诈智能体 MCP 服务 — 通过「话术状态机」技术实时检测 10 类电信诈骗模式

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![MCP](https://img.shields.io/badge/MCP-Protocol-green.svg)](https://modelcontextprotocol.io/)

## 简介

枭卫（OwlGuard）是一个基于 [MCP（Model Context Protocol）](https://modelcontextprotocol.io/) 协议的反诈检测服务。它采用**话术状态机**技术，通过追踪对话中诈骗话术的递进演变过程，实现对电信诈骗的精准识别与早期预警。

### 核心特性

- **10 类诈骗检测**：覆盖当前最高发的电信诈骗类型
- **话术状态机引擎**：追踪诈骗话术递进路径，而非简单关键词匹配
- **敏感信息检测**：自动识别银行卡号、身份证号、验证码等敏感数据
- **威胁情报查询**：恶意 URL 和高危 APP 检测
- **MCP 标准协议**：与任何支持 MCP 的 AI 平台无缝集成
- **纯 Rust 实现**：高性能、低延迟、内存安全

## 支持的诈骗类型

| 编号 | 诈骗类型 | 状态机层级 |
|------|---------|-----------|
| F001 | 刷单返利 | S0→S5（6级） |
| F002 | 虚假投资理财 | S0→S5（6级） |
| F003 | 虚假购物服务 | S0→S4（5级） |
| F004 | 冒充电商客服 | S0→S5（6级） |
| F005 | 贷款征信 | S0→S4（5级） |
| F006 | 冒充领导熟人 | S0→S4（5级） |
| F007 | 冒充公检法 | S0→S4（5级） |
| F008 | 婚恋交友（杀猪盘） | S0→S7（8级） |
| F009 | 游戏虚假交易 | S0→S4（5级） |
| F010 | 机票退改签 | S0→S3（4级） |

## MCP 工具列表

| 工具名 | 说明 |
|--------|------|
| `scan_conversation` | 扫描对话，检测10类诈骗话术，返回风险评分、诈骗类型、状态路径和建议动作 |
| `detect_sensitive` | 检测文本中的敏感信息（银行卡号、身份证号、验证码、支付密码） |
| `check_url` | 检测URL是否为恶意/钓鱼链接 |
| `check_app` | 检测APP名称是否为高危诱导下载APP |
| `get_rules_version` | 获取当前规则库版本信息 |
| `batch_scan` | 批量扫描多条对话（用于后台审核） |

## 快速开始

### 环境要求

- Rust 1.70+（推荐最新稳定版）
- 无需数据库、无需外部服务依赖

### 编译安装

```bash
# 克隆仓库
git clone https://github.com/Yitu-Loong/owlguard-mcp.git
cd owlguard-mcp

# 编译 release 版本
cargo build --release

# 二进制文件位于 target/release/owlguard-mcp
```

### 运行测试

```bash
# 运行全部测试（105 单元测试 + 11 集成测试）
cargo test

# 运行 clippy 检查
cargo clippy -- -D warnings

# 运行性能基准测试
cargo bench
```

---

## 平台集成指南

枭卫 MCP 服务遵循 MCP 标准协议，支持多种部署方式，方便各类 AI 平台集成调用。

### 部署方式一：Stdio 传输（推荐 — 桌面端集成）

**适用场景**：Claude Desktop、Trae IDE、Cursor、Windsurf 等桌面 AI 客户端

这是最简单的集成方式，MCP 服务通过标准输入/输出与 AI 客户端通信，无需网络端口。

#### Claude Desktop 配置

编辑 `~/AppData/Roaming/Claude/claude_desktop_config.json`（Windows）或 `~/Library/Application Support/Claude/claude_desktop_config.json`（macOS）：

```json
{
  "mcpServers": {
    "owlguard": {
      "command": "owlguard-mcp",
      "args": []
    }
  }
}
```

如果二进制不在 PATH 中，使用完整路径：

```json
{
  "mcpServers": {
    "owlguard": {
      "command": "/完整路径/owlguard-mcp",
      "args": []
    }
  }
}
```

#### Trae IDE 配置

在 Trae IDE 的 MCP 设置中添加：

```json
{
  "mcpServers": {
    "owlguard": {
      "command": "owlguard-mcp",
      "args": []
    }
  }
}
```

#### Cursor 配置

在 `.cursor/mcp.json` 中添加：

```json
{
  "mcpServers": {
    "owlguard": {
      "command": "owlguard-mcp",
      "args": []
    }
  }
}
```

### 部署方式二：SSE 传输（服务端部署）

**适用场景**：Web 平台、SaaS 服务、需要远程调用的场景

通过 HTTP SSE（Server-Sent Events）传输，支持远程网络调用。需要配合 MCP 代理网关使用。

#### 使用 Supergateway 代理

[Supergateway](https://github.com/supercorp-ai/supergateway) 可以将 stdio MCP 服务转换为 SSE/WebSocket 服务：

```bash
# 安装 supergateway
npm install -g supergateway

# 启动 SSE 服务（将 stdio 转为 SSE）
npx supergateway --stdio "owlguard-mcp" --port 8808
```

客户端连接配置：

```json
{
  "mcpServers": {
    "owlguard": {
      "url": "http://your-server:8808/sse"
    }
  }
}
```

#### 使用 mcp-proxy 代理

[mcp-proxy](https://github.com/sparfenyuk/mcp-proxy) 另一个流行的 stdio-to-SSE 代理：

```bash
# 安装 mcp-proxy
pip install mcp-proxy

# 启动 SSE 服务
mcp-proxy --port 8808 owlguard-mcp
```

### 部署方式三：Docker 容器化部署

**适用场景**：云原生环境、Kubernetes 集群、需要弹性伸缩的生产环境

#### 构建镜像

```dockerfile
FROM rust:1.82-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/owlguard-mcp /usr/local/bin/
COPY --from=builder /app/rules/ /etc/owlguard/rules/

ENTRYPOINT ["owlguard-mcp"]
```

```bash
# 构建镜像
docker build -t owlguard-mcp:latest .

# 以 stdio 模式运行（配合代理网关）
docker run -i owlguard-mcp:latest

# 配合 supergateway 暴露 SSE 端点
docker run -d -p 8808:8808 \
  node:20-slim \
  sh -c "npm install -g supergateway && supergateway --stdio 'docker run -i owlguard-mcp:latest' --port 8808 --host 0.0.0.0"
```

#### Docker Compose（完整方案）

```yaml
version: '3.8'
services:
  owlguard-mcp:
    build: .
    restart: unless-stopped

  owlguard-gateway:
    image: node:20-slim
    ports:
      - "8808:8808"
    command: >
      sh -c "npm install -g supergateway &&
             supergateway --stdio 'docker run -i owlguard-mcp owlguard-mcp' --port 8808 --host 0.0.0.0"
    depends_on:
      - owlguard-mcp
```

### 部署方式四：NPM 包封装

**适用场景**：Node.js 生态、前端开发者、需要 npm install 一键安装

创建 `package.json` 并使用 `bin` 字段封装原生二进制：

```json
{
  "name": "owlguard-mcp",
  "version": "0.1.0",
  "description": "枭卫（OwlGuard）MCP — 开源反诈智能体MCP服务",
  "bin": {
    "owlguard-mcp": "./bin/owlguard-mcp"
  }
}
```

然后在支持 MCP 的客户端中配置：

```json
{
  "mcpServers": {
    "owlguard": {
      "command": "npx",
      "args": ["-y", "owlguard-mcp"]
    }
  }
}
```

### 部署方式对比

| 部署方式 | 适用场景 | 网络要求 | 复杂度 | 延迟 |
|---------|---------|---------|-------|------|
| Stdio 直连 | 桌面 AI 客户端 | 本地 | ★☆☆ | 最低 |
| SSE 代理 | Web/SaaS 平台 | 内网/公网 | ★★☆ | 低 |
| Docker 容器 | 云原生/K8s | 内网/公网 | ★★☆ | 低 |
| NPM 封装 | Node.js 生态 | 本地 | ★☆☆ | 最低 |

---

## API 示例

### 扫描对话

```json
{
  "conversation": [
    { "role": "other", "content": "手机兼职，日结300-500" },
    { "role": "other", "content": "我们是正规平台，不交押金" },
    { "role": "other", "content": "先做一单试试，马上返你" },
    { "role": "other", "content": "这单佣金高，需要垫付" }
  ],
  "context_hint": "转账场景"
}
```

返回：

```json
{
  "risk_score": 0.8,
  "risk_level": "high",
  "fraud_type": "刷单返利",
  "fraud_type_id": "F001",
  "current_state": "垫付阶段",
  "matched_keywords": ["兼职", "日结", "正规平台", "不交押金", "垫付"],
  "action_suggestion": "block",
  "warning_text": "⚠️ 极高风险：刷单诈骗垫付阶段，立即终止对话！",
  "reasoning": "刷单返利话术状态机路径: S0(初始) → S1(诱饵抛出) → S2(信任建立) → S3(小额返利) → S4(垫付诱导)"
}
```

### 检测敏感信息

```json
{
  "text": "验证码是123456，银行卡号6222021234567894"
}
```

返回：

```json
{
  "found": true,
  "sensitive_items": [
    { "sensitive_type": "verification_code", "value": "123456", "position": 4 },
    { "sensitive_type": "bank_card", "value": "6222****7894", "position": 14 }
  ]
}
```

## 项目架构

```
owlguard-mcp/
├── src/
│   ├── main.rs                    # 入口
│   ├── lib.rs                     # 库导出
│   ├── mcp/                       # MCP 协议层
│   │   ├── server.rs              # MCP 服务器（rmcp 框架）
│   │   └── tools.rs               # 6个 MCP 工具定义与处理
│   ├── detectors/                 # 检测引擎
│   │   ├── orchestrator.rs        # 扫描编排器（核心调度）
│   │   ├── risk_scorer.rs         # 风险评分器
│   │   ├── sensitive.rs           # 敏感信息检测
│   │   ├── threat_intel.rs        # 威胁情报查询
│   │   └── state_machine/         # 话术状态机
│   │       ├── mod.rs             # 通用状态机引擎
│   │       ├── brushing.rs        # F001 刷单返利
│   │       ├── investment.rs      # F002 虚假投资理财
│   │       ├── fake_shopping.rs   # F003 虚假购物服务
│   │       ├── fake_customer.rs   # F004 冒充电商客服
│   │       ├── loan_credit.rs     # F005 贷款征信
│   │       ├── fake_leader.rs     # F006 冒充领导熟人
│   │       ├── fake_police.rs     # F007 冒充公检法
│   │       ├── romance.rs         # F008 婚恋交友
│   │       ├── game_trade.rs      # F009 游戏虚假交易
│   │       └── flight_change.rs   # F010 机票退改签
│   ├── matchers/                  # 文本匹配引擎
│   │   ├── keyword.rs             # Aho-Corasick 多模匹配
│   │   └── regex_matcher.rs       # 正则匹配
│   ├── rules/                     # 规则加载器
│   │   └── loader.rs              # JSON 规则文件加载
│   └── utils/                     # 工具函数
│       ├── luhn.rs                # Luhn 银行卡校验
│       ├── idcard.rs              # 身份证号校验
│       └── text_clean.rs          # 文本清洗
├── rules/                         # 规则数据文件
│   ├── fraud_rules.json           # 诈骗话术规则
│   ├── sensitive_patterns.json    # 敏感信息模式
│   ├── blacklist_urls.json        # 恶意URL黑名单
│   └── blacklist_apps.json        # 高危APP黑名单
├── benches/                       # 性能基准测试
│   └── scan_benchmark.rs
└── tests/                         # 集成测试
    └── integration_test.rs
```

## 技术原理

### 话术状态机

传统关键词匹配只能识别单一风险词，无法判断诈骗的递进过程。枭卫采用**话术状态机**技术：

1. **状态不可逆**：诈骗话术只能向前推进（S0→S1→S2→...），不会回退
2. **路径追踪**：记录完整的状态转换路径，提供可解释的推理链
3. **风险递增**：随着状态推进，风险评分和告警级别逐步提升
4. **多机并行**：10 个状态机并行运行，取最高风险结果

### 风险评分模型

综合诈骗状态机评分与敏感信息检测结果：

| 条件 | 风险等级 | 建议动作 |
|------|---------|---------|
| 诈骗评分 > 0.5 + 验证码 | Critical | Block |
| 诈骗评分 > 0.3 + 银行卡/身份证 | High | Block |
| 诈骗评分 > 0.6（状态机自身建议） | High | Block/Warn |
| 诈骗评分 > 0.3 | Medium | Warn |
| 诈骗评分 > 0.1 | Low | Monitor |
| 仅敏感信息 | Medium | Warn |
| 无风险 | None | None |

## 贡献指南

欢迎贡献！请遵循以下步骤：

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 确保所有测试通过 (`cargo test`)
4. 确保代码质量 (`cargo clippy -- -D warnings`)
5. 提交变更 (`git commit -m '添加某个特性'`)
6. 推送分支 (`git push origin feature/amazing-feature`)
7. 创建 Pull Request

## 许可证

本项目基于 [Apache License 2.0](LICENSE) 开源。

## 致谢

- [rmcp](https://github.com/anthropics/rmcp) — Rust MCP 协议实现
- [aho-corasick](https://github.com/BurntSushi/aho-corasick) — 多模式字符串匹配
- [MCP 协议规范](https://modelcontextprotocol.io/) — Model Context Protocol
