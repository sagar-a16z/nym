use clap::Args;
use network_defaults::NymNetworkDetails;
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
    /// The chain-id (ex: nym, nym-sandbox, etc)
    #[clap(long)]
    chain_id: String,
}

pub(crate) async fn execute(
    args: &Undelegate,
    network_details: NymNetworkDetails,
    wallet: DirectSecp256k1HdWallet,
) {
    // setup a client, and look up the account info.
    let config = Config::try_from_nym_network_details(&network_details).expect("no config");
    let offline_signer = Client::new_offline_signing(config, wallet);
    let result = offline_signer
        .nymd
        .remove_mixnode_delegation_offline(
            args.mixnode_identity.clone(),
            args.account_number,
            args.sequence_number,
            args.chain_id.parse().expect("Invalid Chain ID"),
        )
        .await;
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

    #[tokio::test]
    async fn test_generate_tx() {
        let mnemonic = "drill poet latin puzzle fork lift rocket magic width hello radio glue loop electric jacket guide job goat dust provide input spoon wall thumb";

        let args = Undelegate {
            mixnode_identity: "".to_string(),
            sequence_number: 0,
            account_number: 450,
            chain_id: "nym-sandbox".to_string(),
        };
        let wallet = DirectSecp256k1HdWallet::from_mnemonic(
            &SANDBOX.bech32_prefix(),
            mnemonic.parse().unwrap(),
        )
        .unwrap();

        execute(&args, SANDBOX.details(), wallet).await;
    }
}
