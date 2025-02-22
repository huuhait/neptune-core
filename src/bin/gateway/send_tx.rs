// use axum::extract::State;
// use axum::response::Json;
// use axum::response::Response;
// use neptune_cash::models::state::wallet::utxo_notification::PrivateNotificationData;
// use std::sync::Arc;
// use tarpc::context;

// use super::http_util::rpc_err;
// use super::http_util::rpc_method_err;
// use super::state::AppState;

// #[derive(serde::Deserialize)]
// pub struct SendTxPayload {
//   pub to_address: String,
//   pub amount: String,
//   pub fee: String,
// }

// #[axum::debug_handler]
// pub async fn send_tx(
//     Json(payload): Json<SendTxPayload>,
//     State(state): State<Arc<AppState>>,
//   ) -> Result<Json<PrivateNotificationData>, Response> {
//     let s = state.load();

//     let valid_amount = s.rpc_client.validate_amount(context::current(), s.token(), payload.amount).await
//       .map_err(rpc_err)?
//       .map_err(rpc_method_err)?
//       .unwrap();

//     let valid_fee = s.rpc_client.validate_amount(context::current(), s.token(), payload.fee).await
//       .map_err(rpc_err)?
//       .map_err(rpc_method_err)?
//       .unwrap();

//     let to_address = s.rpc_client.validate_address(context::current(), s.token(), payload.to_address, s.network).await
//       .map_err(rpc_err)?
//       .map_err(rpc_method_err)?
//       .unwrap();

//     let sent_tx = s
//         .rpc_client
//         .send(
//           context::current(),
//           s.token(),
//           valid_amount,
//           to_address,
//           neptune_cash::models::state::wallet::utxo_notification::UtxoNotificationMedium::OnChain,
//           neptune_cash::models::state::wallet::utxo_notification::UtxoNotificationMedium::OnChain,
//           valid_fee,
//         )
//         .await
//         .map_err(rpc_err)?
//         .map_err(rpc_method_err)?;

//       // take out first value of sent_tx
//     let (_, private_notification_data) = sent_tx;

//     let first_private_notification_data = private_notification_data.into_iter().next().unwrap();

//     Ok(Json(first_private_notification_data))
// }
