use rmcp::transport::stdio;
use rmcp::{
    Error as McpError, ServerHandler, ServiceExt,
    model::{
        CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    tool,
};
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct UUID;

#[tool(tool_box)]
impl UUID {
    #[tool(name = "sdk-uuidv4", description = "Generate a UUIDv4")]
    async fn uuidv4(&self) -> Result<CallToolResult, McpError> {
        let uuid = uuid::Uuid::new_v4();
        Ok(CallToolResult::success(vec![Content::text(
            uuid.to_string(),
        )]))
    }

    #[tool(name = "sdk-uuidv7", description = "Generate a UUIDv7")]
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
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting MCP server");

    let service = UUID.serve(stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    service.waiting().await?;
    Ok(())
}
