use anyhow::Context;
use gateway::get_balance::get_balance;
use gateway::get_block_height::block_height;
use gateway::get_history_by_height::history_by_height;
use gateway::state::AppState;
use axum::routing::get;
use axum::Router;

use clap::Parser;

pub mod gateway;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
  let args: gateway::config::Config = gateway::config::Config::parse();
  let listen_port = args.listen_port;

  let app_state = AppState::init(args).await?;

  let routes = setup_routes(app_state);
  let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{listen_port}"))
      .await
      .with_context(|| format!("Failed to bind to port {listen_port}"))?;

  axum::serve(listener, routes)
        .await
        .with_context(|| "Axum server encountered an error")?;
    Ok(())
}

pub fn setup_routes(app_state: AppState) -> Router {
  Router::new()
    .route("/rpc/block_height", get(block_height))
    .route("/rpc/history/:height", get(history_by_height))
    .route("/rpc/balance", get(get_balance))
    .with_state(app_state.into())
}
