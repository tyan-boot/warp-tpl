use views::build_server;

use crate::errors::AppError;
use crate::state::AppState;

mod auth;
mod errors;
#[macro_use]
mod state;
mod views;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    env_logger::init();

    let state = AppState::create()?;

    let server = build_server(state);

    server.run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}
