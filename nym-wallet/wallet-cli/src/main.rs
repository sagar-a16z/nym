use clap::{crate_version, Parser};
use lazy_static::lazy_static;
use log::LevelFilter;
use network_defaults::all::Network;
use network_defaults::all::Network::{MAINNET, QA, SANDBOX};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use validator_client::nymd::wallet::DirectSecp256k1HdWallet;

mod commands;

lazy_static! {
    pub static ref LONG_VERSION: String = "0.0.1".to_string();
}

// Helper for passing LONG_VERSION to clap
fn long_version_static() -> &'static str {
    &LONG_VERSION
}

#[derive(Parser)]
#[clap(author = "Nymtech", version, about, long_version = long_version_static())]
struct Args {
    /// Path pointing to a keypair file that contains the mnemonic for the client keypair.
    #[clap(long)]
    pub(crate) keypair: Option<std::path::PathBuf>,
    #[clap(subcommand)]
    command: commands::Commands,
    /// The Network to use - One of SANDBOX, QA, or MAINNET (custom is not supported right now)
    #[clap(long)]
    network: Network,
}

#[tokio::main]
async fn main() {
    setup_logging();
    println!("{}", banner());
    let args = Args::parse();
    let mut wallet = None;

    let network_details;
    match &args.network {
        Network::QA => {
            network_details = QA.details();
        }
        Network::SANDBOX => {
            network_details = SANDBOX.details();
        }
        Network::MAINNET => {
            network_details = MAINNET.details();
        }
        Network::CUSTOM { details } => {
            network_details = details.clone();
        }
    }

    if let Some(keypair_path) = &args.keypair {
        let mnemonic = read_mnemonic(keypair_path).expect("unable to parse mnemonic");
        wallet = Some(
            DirectSecp256k1HdWallet::from_mnemonic(
                &network_details.chain_details.bech32_account_prefix,
                mnemonic.parse().unwrap(),
            )
            .expect("invalid wallet"),
        );
    }

    commands::execute(args, network_details, wallet).await;
}

// TODO: Use a utility for this, something that understands what a mnemonic string should look like
fn read_mnemonic(keypair_file: &std::path::PathBuf) -> Result<String, io::Error> {
    let file = File::open(keypair_file)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader
        .read_line(&mut buffer)
        .expect("Unable to read keypair file");
    return Ok(buffer);
}

fn banner() -> String {
    format!(
        r#"

      _ __  _   _ _ __ ___
     | '_ \| | | | '_ \ _ \
     | | | | |_| | | | | | |
     |_| |_|\__, |_| |_| |_|
            |___/

             (wallet-cli - version {:})

    "#,
        crate_version!()
    )
}

fn setup_logging() {
    let mut log_builder = pretty_env_logger::formatted_timed_builder();
    if let Ok(s) = ::std::env::var("RUST_LOG") {
        log_builder.parse_filters(&s);
    } else {
        // default to 'Info'
        log_builder.filter(None, LevelFilter::Info);
    }

    log_builder
        .filter_module("hyper", LevelFilter::Warn)
        .filter_module("tokio_reactor", LevelFilter::Warn)
        .filter_module("reqwest", LevelFilter::Warn)
        .filter_module("mio", LevelFilter::Warn)
        .filter_module("want", LevelFilter::Warn)
        .init();
}
