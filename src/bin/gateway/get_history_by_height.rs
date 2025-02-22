use axum::extract::State;
use axum::extract::Path;
use axum::response::Json;
use axum::response::Response;
use std::sync::Arc;
use tarpc::context;

use neptune_cash::models::blockchain::block::block_height::BlockHeight;
use neptune_cash::prelude::twenty_first::math::digest::Digest;
use neptune_cash::models::proof_abstractions::timestamp::Timestamp;

use super::http_util::rpc_err;
use super::http_util::rpc_method_err;
use super::state::AppState;

#[derive(serde::Serialize)]
pub struct HistoryRecord {
  utxo_digest: Digest,
  sender_randomness: Digest,
  digest: Digest,
  block_height: BlockHeight,
  timestamp: Timestamp,
  receiving_address: String,
  native_currency_amount: String,
}

#[axum::debug_handler]
pub async fn history_by_height(
    Path(height): Path<BlockHeight>,
    State(state): State<Arc<AppState>>,
  ) -> Result<Json<Vec<HistoryRecord>>, Response> {
    let s = state.load();

    let network = s.network;
    let history = s
        .rpc_client
        .history_by_height(context::current(), s.token(), height)
        .await
        .map_err(rpc_err)?
        .map_err(rpc_method_err)?;

    // history is a Vec<(Digest, BlockHeight, Timestamp, SpendingKey, NativeCurrencyAmount)>
    // make it to array of json objects

    let mut history_rows = Vec::new();
    for (leaf_index, sr, digest, block_height, timestamp, spending_key, native_currency_amount) in history.iter() {
      let utxo_digest = s.rpc_client.utxo_digest(context::current(), s.token(), *leaf_index)
        .await
        .map_err(rpc_err)?
        .map_err(rpc_method_err)?
        .unwrap();

      let address = spending_key.to_address().expect("valid spending key");
      let receiving_address = address.to_bech32m(network).expect("valid address");

      // => HistoryRecord { digest, block_height, timestamp, spending_key, native_currency_amount }
      history_rows.push(HistoryRecord {
        utxo_digest: utxo_digest,
        sender_randomness: *sr,
        digest: *digest,
        block_height: *block_height,
        timestamp: *timestamp,
        receiving_address: receiving_address,
        native_currency_amount: native_currency_amount.to_string(),
      });
    }

    Ok(Json(history_rows))
}
