use crate::NetworkInfo;
use clap::Args;
use validator_client::nymd::wallet::DirectSecp256k1HdWallet;
use validator_client::{Client, Config};

#[derive(Args, Clone)]
pub(crate) struct Undelegate {
    /// The Mixnode Identity to Delegate tokens
    #[clap(long)]
    mixnode_identity: String,
    /// The Sequence number to sue for the offline signature
    #[clap(long)]
    sequence_number: u64,
    /// Account number to use for the offline signature
    #[clap(long)]
    account_number: u64,
}

pub(crate) fn execute(
    args: &Undelegate,
    network_info: NetworkInfo,
    wallet: DirectSecp256k1HdWallet,
) {
    // setup a client, and look up the account info.
    let config =
        Config::try_from_nym_network_details(&network_info.network_details).expect("no config");
    let offline_signer = Client::new_offline_signing(config, wallet);
    let result = offline_signer
        .nymd
        .execute_offline_remove_mixnode_delegation(
            args.mixnode_identity.clone(),
            args.account_number,
            args.sequence_number,
            network_info.chain_id.parse().expect("Invalid Chain ID"),
        );
    match result {
        Ok(response) => {
            let json = serde_json::to_string(&response.data).unwrap();
            println!("Undelegate Transaction: {:x?}", json);
        }
        Err(error) => println!("Failed to send transaction with Error: {}", error),
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::undelegate::*;
    use network_defaults::all::Network::SANDBOX;

    #[test]
    fn test_generate_tx() {
        let mnemonic = "drill poet latin puzzle fork lift rocket magic width hello radio glue loop electric jacket guide job goat dust provide input spoon wall thumb";

        let args = Undelegate {
            mixnode_identity: "".to_string(),
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
