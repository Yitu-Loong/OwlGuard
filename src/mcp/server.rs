use std::future::Future;

use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::router::tool::ToolRouter,
    model::{ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
};

use super::tools;

#[derive(Debug, Clone)]
pub struct OwlGuardServer {
    tool_router: ToolRouter<Self>,
}

impl Default for OwlGuardServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router]
impl OwlGuardServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "扫描对话，检测10类诈骗话术。输入对话数组，返回风险评分、诈骗类型、当前状态和建议动作。")]
    async fn scan_conversation(
        &self,
        rmcp::handler::server::tool::Parameters(req): rmcp::handler::server::tool::Parameters<tools::ScanConversationRequest>,
    ) -> String {
        tools::handle_scan_conversation(req)
    }

    #[tool(description = "检测文本中的敏感信息（银行卡号、身份证号、验证码等）")]
    async fn detect_sensitive(
        &self,
        rmcp::handler::server::tool::Parameters(req): rmcp::handler::server::tool::Parameters<tools::DetectSensitiveRequest>,
    ) -> String {
        tools::handle_detect_sensitive(req)
    }

    #[tool(description = "检测URL是否为恶意/钓鱼链接")]
    async fn check_url(
        &self,
        rmcp::handler::server::tool::Parameters(req): rmcp::handler::server::tool::Parameters<tools::CheckUrlRequest>,
    ) -> String {
        tools::handle_check_url(req)
    }

    #[tool(description = "检测APP名称是否为高危诱导下载APP")]
    async fn check_app(
        &self,
        rmcp::handler::server::tool::Parameters(req): rmcp::handler::server::tool::Parameters<tools::CheckAppRequest>,
    ) -> String {
        tools::handle_check_app(req)
    }

    #[tool(description = "获取当前规则库版本信息")]
    async fn get_rules_version(&self) -> String {
        tools::handle_get_rules_version()
    }

    #[tool(description = "批量扫描多条对话（用于后台审核）")]
    async fn batch_scan(
        &self,
        rmcp::handler::server::tool::Parameters(req): rmcp::handler::server::tool::Parameters<tools::BatchScanRequest>,
    ) -> String {
        tools::handle_batch_scan(req)
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for OwlGuardServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("枭卫（OwlGuard）MCP — 开源反诈智能体MCP服务，通过话术状态机技术实时检测10类电信诈骗模式".into()),
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            ..Default::default()
        }
    }
}

pub async fn run() -> anyhow::Result<()> {
    let server = OwlGuardServer::new();
    let transport = rmcp::transport::stdio();
    let service = server
        .serve(transport)
        .await
        .map_err(|e| anyhow::anyhow!("MCP服务启动失败: {e}"))?;

    log::info!("枭卫MCP服务器已启动 (stdio传输)");
    service.waiting().await.map_err(|e| anyhow::anyhow!("服务运行错误: {e}"))?;

    Ok(())
}
