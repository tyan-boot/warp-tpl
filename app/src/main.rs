mod errors;
mod views;

use views::build_server;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let server = build_server();

    server.run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}
