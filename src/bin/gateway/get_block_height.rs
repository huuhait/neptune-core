use neptune_cash::models::blockchain::block::block_height::BlockHeight;
use axum::extract::State;
use axum::response::Json;
use axum::response::Response;
use std::sync::Arc;
use tarpc::context;

use super::http_util::rpc_err;
use super::http_util::rpc_method_err;
use super::state::AppState;

#[axum::debug_handler]
pub async fn block_height(
    State(state): State<Arc<AppState>>,
  ) -> Result<Json<BlockHeight>, Response> {
    let s = state.load();
    let block_height = s
        .rpc_client
        .block_height(context::current(), s.token())
        .await
        .map_err(rpc_err)?
        .map_err(rpc_method_err)?;

    Ok(Json(block_height))
}
