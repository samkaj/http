use server::HttpServer;

pub mod server; // TODO: move

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = HttpServer::new("localhost", 8080, "/");
    server.serve().await?;

    Ok(())
}
