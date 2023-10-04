use crate::NetworkInfo;
use clap::Args;
use std::str::FromStr;
use validator_client::nymd::AccountId;
use validator_client::{Client, Config};

#[derive(Args, Clone)]
pub(crate) struct Account {
    /// The host address (where nymd is running)
    #[clap(long)]
    host: Option<String>,
    /// The account public key to lookup
    #[clap(long)]
    account_id: String,
    // advanced options
    #[clap(long)]
    mix_port: Option<u16>,
    #[clap(long)]
    verloc_port: Option<u16>,
    #[clap(long)]
    http_api_port: Option<u16>,
}

pub(crate) async fn execute(args: &Account, network_info: NetworkInfo) {
    // setup a client, and look up the account info.
    let mut config =
        Config::try_from_nym_network_details(&network_info.network_details).expect("no config");
    if let Some(host) = &args.host {
        config = config.with_nymd_url(host.parse().expect("couldn't parse host url"));
    }
    let account_id = &args.account_id;

    let client = Client::new_query(config).expect("Unable to connect to Mainnet");
    let account_info = client
        .nymd
        .account_sequence_for_address(
            &AccountId::from_str(account_id).expect("Invalid account address"),
        )
        .await
        .expect("Could not get account sequence");
    println!(
        "Sequence Number: {}\n Account Number: {}",
        account_info.sequence, account_info.account_number
    );
}

#[cfg(test)]
mod tests {
    use crate::commands::account::*;
    use network_defaults::all::Network::MAINNET;

    #[tokio::test]
    async fn test_account_info() {
        let args = Account {
            host: None,
            account_id: "n1ef8fjswyuxnv9779qfug042aca8heven66ptnx".to_string(),
            mix_port: None,
            verloc_port: None,
            http_api_port: None,
        };

        execute(
            &args,
            NetworkInfo {
                network_details: MAINNET.details(),
                chain_id: "nym-mixnet".to_string(),
            },
        )
        .await;
    }
}
