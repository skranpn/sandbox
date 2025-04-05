use poem::{EndpointExt, Route, Server, listener::TcpListener, middleware::Cors};
use poem_mcpserver::{McpServer, Tools, sse::sse_endpoint, tool::Text};

struct UUID;

/// This server provides a uuid tool that can generate UUIDv4 and UUIDv7.
///
/// The uuid tool has the following commands:
/// - `sse_uuidv4`: Generate a UUIDv4
/// - `sse_uuidv7`: Generate a UUIDv7
#[Tools]
impl UUID {
    /// Generate a UUIDv4
    async fn sse_uuidv4(&self) -> Text<String> {
        let uuid = uuid::Uuid::new_v4();
        Text(uuid.to_string())
    }

    /// Generate a UUIDv7
    async fn sse_uuidv7(&self) -> Text<String> {
        let ts = uuid::Timestamp::now(uuid::NoContext);
        let uuid = uuid::Uuid::new_v7(ts);
        Text(uuid.to_string())
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000");
    let app = Route::new()
        .at("/sse", sse_endpoint(|_| McpServer::new().tools(UUID)))
        .with(Cors::new());
    Server::new(listener).run(app).await
}
