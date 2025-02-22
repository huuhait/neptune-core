use std::net::Ipv4Addr;
use std::net::SocketAddr;

use crate::gateway::config::Config;
use neptune_cash::rpc_server::RPCClient;
use neptune_cash::config_models::data_directory::DataDirectory;
use neptune_cash::config_models::network::Network;
use neptune_cash::rpc_auth;
use neptune_cash::rpc_server::error::RpcError;
use tarpc::client;
use tarpc::context;
use tarpc::tokio_serde::formats::Json;
use arc_swap::ArcSwap;
use std::sync::Arc;

pub struct AppStateInner {
    pub config: Config,
    pub rpc_client: RPCClient,
    pub network: Network,
    pub token: rpc_auth::Token,
}

impl AppStateInner {
  pub fn token(&self) -> rpc_auth::Token {
      self.token
  }
}

#[derive(Clone)]
pub struct AppState(Arc<ArcSwap<AppStateInner>>);

impl std::ops::Deref for AppState {
  type Target = Arc<ArcSwap<AppStateInner>>;

  fn deref(&self) -> &Self::Target {
      &self.0
  }
}

impl From<(Config, RPCClient, Network, rpc_auth::Token)> for AppState {
  fn from(
      (config, rpc_client, network, token): (
          Config,
          RPCClient,
          Network,
          rpc_auth::Token,
      ),
  ) -> Self {
      Self(Arc::new(ArcSwap::from_pointee(AppStateInner {
          network,
          config,
          rpc_client,
          token,
      })))
  }
}

impl AppState {
    pub async fn init(config: Config) -> Result<Self, anyhow::Error> {
      let server_socket = SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::LOCALHOST), config.port);
      let transport = tarpc::serde_transport::tcp::connect(server_socket, Json::default).await?;

      let client = RPCClient::new(client::Config::default(), transport).spawn();

      let rpc_auth::CookieHint {
          data_directory,
          network,
      } = match get_cookie_hint(&client, &config).await {
          Ok(h) => h,
          Err(e) => {
              eprintln!("{e}");
              eprintln!(
                  "Could not ping neptune-core. Do configurations match? Or is it still starting up?"
              );
              std::process::exit(1);
          }
      };

      let token: rpc_auth::Token = match rpc_auth::Cookie::try_load(&data_directory).await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Unable to load RPC auth cookie. error = {}", e);
            std::process::exit(2)
        }
      }
      .into();

      Ok(Self::from((
        config,
        client,
        network,
        token,
      )))
    }
}

// returns result with a CookieHint{ data_directory, network }.
//
// We use the data-dir provided by user if present.
//
// Otherwise, we call cookie_hint() RPC to obtain data-dir.
// But the API might be disabled, which we detect and fallback to the default data-dir.
async fn get_cookie_hint(
  client: &RPCClient,
  args: &Config,
) -> anyhow::Result<rpc_auth::CookieHint> {
  async fn fallback(client: &RPCClient, args: &Config) -> anyhow::Result<rpc_auth::CookieHint> {
      let network = client.network(context::current()).await??;
      let data_directory = DataDirectory::get(args.data_dir.clone(), network)?;
      Ok(rpc_auth::CookieHint {
          data_directory,
          network,
      })
  }

  if args.data_dir.is_some() {
      return fallback(client, args).await;
  }

  let result = client.cookie_hint(context::current()).await?;

  match result {
      Ok(hint) => Ok(hint),
      Err(RpcError::CookieHintDisabled) => fallback(client, args).await,
      Err(e) => Err(e.into()),
  }
}
