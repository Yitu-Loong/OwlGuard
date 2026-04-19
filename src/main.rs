use owlguard_mcp::mcp;

#[tokio::main]
async fn main() {
    env_logger::init();

    if let Err(e) = mcp::server::run().await {
        log::error!("MCP服务启动失败: {e}");
        std::process::exit(1);
    }
}
