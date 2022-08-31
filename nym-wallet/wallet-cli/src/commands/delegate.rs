use crate::NetworkInfo;
use clap::Args;
use validator_client::nymd::wallet::DirectSecp256k1HdWallet;
use validator_client::nymd::Coin;
use validator_client::{Client, Config};

#[derive(Args, Clone)]
pub(crate) struct Delegate {
    /// The Mixnode Identity to Delegate tokens
    #[clap(long)]
    mixnode_identity: String,
    /// The value of the delegation being made
    #[clap(long)]
    amount: u128,
    /// The Sequence number to sue for the offline signature
    #[clap(long)]
    sequence_number: u64,
    /// Account number to use for the offline signature
    #[clap(long)]
    account_number: u64,
}

pub(crate) fn execute(args: &Delegate, network_info: NetworkInfo, wallet: DirectSecp256k1HdWallet) {
    // setup a client, and look up the account info.
    let config =
        Config::try_from_nym_network_details(&network_info.network_details).expect("no config");
    let offline_signer = Client::new_offline_signing(config, wallet);
    let amount: Coin = Coin {
        amount: args.amount,
        denom: network_info
            .network_details
            .chain_details
            .mix_denom
            .base
            .clone(),
    };
    let result = offline_signer.nymd.execute_offline_delegate_to_mixnode(
        args.mixnode_identity.clone(),
        amount,
        args.account_number,
        args.sequence_number,
        network_info.chain_id.parse().expect("Invalid Chain ID"),
    );
    match result {
        Ok(response) => {
            let json = serde_json::to_string(&response.data).unwrap();
            println!("Delegate Transaction: {:x?}", json);
        }
        Err(error) => println!("Failed to send transaction with Error: {}", error),
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::delegate::*;
    use network_defaults::all::Network::SANDBOX;

    #[test]
    fn test_generate_tx() {
        let mnemonic = "drill poet latin puzzle fork lift rocket magic width hello radio glue loop electric jacket guide job goat dust provide input spoon wall thumb";

        let args = Delegate {
            mixnode_identity: "".to_string(),
            amount: 1_000_000_000_000,
            sequence_number: 0,
            account_number: 450,
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
