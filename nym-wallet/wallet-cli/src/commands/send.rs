use crate::NetworkInfo;
use clap::Args;
use std::str::FromStr;
use validator_client::nymd::wallet::DirectSecp256k1HdWallet;
use validator_client::nymd::{AccountId, Coin};
use validator_client::{Client, Config};

#[derive(Args, Clone)]
pub(crate) struct Send {
    /// The token you want to send ("nym" or "nyx")
    #[clap(short, long, default_value = "nym")]
    token: String,
    /// The Mixnode Identity to Delegate tokens
    #[clap(long)]
    recipient: String,
    /// The value of the delegation being made
    #[clap(long)]
    amount: u128,
    /// The Sequence number to sue for the offline signature
    #[clap(long)]
    sequence_number: u64,
    /// Account number to use for the offline signature
    #[clap(long)]
    account_number: u64,
    /// A message to include with this transfer
    #[clap(long)]
    memo: Option<String>,
}

pub(crate) fn execute(args: &Send, network_info: NetworkInfo, wallet: DirectSecp256k1HdWallet) {
    // setup a client, and look up the account info.
    let config =
        Config::try_from_nym_network_details(&network_info.network_details).expect("no config");
    let offline_signer = Client::new_offline_signing(config, wallet);
    let denom = match args.token.as_str() {
        "nym" | "NYM" | "Nym" => network_info.network_details.chain_details.mix_denom,
        "nyx" | "NYX" | "Nyx" => network_info.network_details.chain_details.stake_denom,
        _ => panic!("Invalid token type"),
    };

    let amount: Coin = Coin {
        amount: args.amount,
        denom: denom.base.clone(),
    };
    let result = offline_signer.nymd.execute_offline_send(
        &AccountId::from_str(&args.recipient).expect("Invalid account address"),
        vec![amount],
        args.memo.clone(),
        args.account_number,
        args.sequence_number,
        network_info.chain_id.parse().expect("Invalid Chain ID"),
    );
    match result {
        Ok(response) => {
            let json = serde_json::to_string(&response.data).unwrap();
            println!("Send Tokens Transaction: {:x?}", json);
        }
        Err(error) => println!("Failed to send transaction with Error: {}", error),
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::send::*;
    use network_defaults::all::Network::SANDBOX;

    #[tokio::test]
    async fn test_generate_tx() {
        let mnemonic = "drill poet latin puzzle fork lift rocket magic width hello radio glue loop electric jacket guide job goat dust provide input spoon wall thumb";

        let args = Send {
            recipient: "nymt1g5vfckav6uwgayquwlfd4fca0kp74wllqnf500".to_string(),
            amount: 1_000_000,
            sequence_number: 0,
            account_number: 450,
            memo: Some("Testing testing 123".to_string()),
        };
        let wallet = DirectSecp256k1HdWallet::from_mnemonic(
            &SANDBOX.bech32_prefix(),
            mnemonic.parse().unwrap(),
        )
        .unwrap();

        execute(
            &args,
            NetworkInfo {
                network_details: SANDBOX.details(),
                chain_id: "nym-sandbox".to_string(),
            },
            wallet,
        );
    }
}
