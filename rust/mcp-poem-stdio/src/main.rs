use poem_mcpserver::{McpServer, Tools, stdio::stdio, tool::Text};
use uuid::NoContext;
struct UUID;

/// This server provides a uuid tool that can generate UUIDv4 and UUIDv7.
///
/// The uuid tool has the following commands:
/// - `stdio_uuidv4`: Generate a UUIDv4
/// - `stdio_uuidv7`: Generate a UUIDv7
#[Tools]
impl UUID {
    /// Generate a UUIDv4
    async fn stdio_uuidv4(&self) -> Text<String> {
        let uuid = uuid::Uuid::new_v4();
        Text(uuid.to_string())
    }

    /// Generate a UUIDv7
    async fn stdio_uuidv7(&self) -> Text<String> {
        let ts = uuid::Timestamp::now(NoContext);
        let uuid = uuid::Uuid::new_v7(ts);
        Text(uuid.to_string())
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    stdio(McpServer::new().tools(UUID)).await
}
