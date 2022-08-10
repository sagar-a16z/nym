use crate::NetworkInfo;
use clap::Args;
use std::str::FromStr;
use validator_client::nymd::AccountId;
use validator_client::{Client, Config};

#[derive(Args, Clone)]
pub(crate) struct Balance {
    /// The account public key to lookup
    #[clap(long)]
    account_id: String,
}

pub(crate) async fn execute(args: &Balance, network_info: NetworkInfo) {
    // setup a client, and look up the account info.
    let config =
        Config::try_from_nym_network_details(&network_info.network_details).expect("no config");
    let account_id = &args.account_id;

    let client = Client::new_query(config).expect("Unable to connect to Mainnet");

    let balance = client
        .nymd
        .get_balance(
            &AccountId::from_str(account_id).expect("Invalid account address"),
            network_info
                .network_details
                .chain_details
                .mix_denom
                .base
                .clone(),
        )
        .await;
    match balance {
        Ok(balance) => {
            println!("Balance {}", balance.unwrap_or_default());
        }
        Err(error) => {
            println!(
                "Failed to get balance for denom:{}. Err:{}",
                network_info.network_details.chain_details.mix_denom.base, error
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::balance::*;
    use network_defaults::all::Network::SANDBOX;

    #[tokio::test]
    async fn test_account_balance() {
        let args = Balance {
            account_id: "nymt129zh9a59lhp87sxf8ax8tljzsz2tn0yg2z0l30".to_string(),
        };

        execute(
            &args,
            NetworkInfo {
                network_details: SANDBOX.details(),
                chain_id: "nym-sandbox".to_string(),
            },
        )
        .await;
    }
}
