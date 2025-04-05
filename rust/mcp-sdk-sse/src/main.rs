use rmcp::transport::sse_server::SseServer;
use rmcp::{
    Error as McpError, ServerHandler,
    model::{
        CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    tool,
};
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    {self},
};

#[derive(Clone)]
struct UUID;

#[tool(tool_box)]
impl UUID {
    fn new() -> Self {
        UUID
    }

    #[tool(name = "sdk-sse-uuidv4", description = "Generate a UUIDv4")]
    async fn uuidv4(&self) -> Result<CallToolResult, McpError> {
        let uuid = uuid::Uuid::new_v4();
        Ok(CallToolResult::success(vec![Content::text(
            uuid.to_string(),
        )]))
    }

    #[tool(name = "sdk-sse-uuidv7", description = "Generate a UUIDv7")]
    async fn uuidv7(&self) -> Result<CallToolResult, McpError> {
        let ts = uuid::Timestamp::now(uuid::NoContext);
        let uuid = uuid::Uuid::new_v7(ts);
        Ok(CallToolResult::success(vec![Content::text(
            uuid.to_string(),
        )]))
    }
}

#[tool(tool_box)]
impl ServerHandler for UUID {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                // .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "This is a UUID tool that can generate UUIDv4 and UUIDv7.".to_string(),
            ),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let ct = SseServer::serve("127.0.0.1:8000".parse()?)
        .await?
        .with_service(UUID::new);

    tokio::signal::ctrl_c().await?;
    ct.cancel();
    Ok(())
}
