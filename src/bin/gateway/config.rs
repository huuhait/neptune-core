use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[clap(name = "neptune-gateway", about = "An RPC client gateway")]
pub struct Config {
    /// Sets the neptune-core rpc server localhost port to connect to.
    #[clap(short, long, default_value = "9799", value_name = "port")]
    pub port: u16,

    #[clap(short, long, default_value = "8880", value_name = "listen_port")]
    pub listen_port: u16,

    /// neptune-core data directory containing wallet and blockchain state
    #[clap(long)]
    pub data_dir: Option<PathBuf>,
}
