use axum::extract::State;
use axum::response::Json;
use axum::response::Response;
use std::sync::Arc;
use tarpc::context;

use super::http_util::rpc_err;
use super::http_util::rpc_method_err;
use super::state::AppState;

#[axum::debug_handler]
pub async fn get_balance(
    State(state): State<Arc<AppState>>,
  ) -> Result<Json<String>, Response> {
    let s = state.load();
    let balance = s
        .rpc_client
        .confirmed_available_balance(context::current(), s.token())
        .await
        .map_err(rpc_err)?
        .map_err(rpc_method_err)?;


    Ok(Json(balance.to_string()))
}
