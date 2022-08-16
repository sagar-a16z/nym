use crate::commands::{DEFAULT_HTTP_API_PORT, DEFAULT_MIX_PORT, DEFAULT_VERLOC_PORT};
use crate::NetworkInfo;
use clap::Args;
use mixnet_contract_common::MixNode;
use validator_client::nymd::wallet::DirectSecp256k1HdWallet;
use validator_client::nymd::Coin;
use validator_client::{Client, Config};

#[derive(Args, Clone)]
pub(crate) struct Bond {
    /// The mixnode identity key
    #[clap(long)]
    identity_key: String,
    /// The mixnode sphinx key
    #[clap(long)]
    sphinx_key: String,
    /// The mixnode owner signature
    #[clap(long)]
    owner_signature: String,
    /// The profit margin percentage this bonding will consume
    #[clap(long)]
    profit_margin_percent: u8,
    /// The value of the pledge being made
    #[clap(long)]
    pledge_amount: u128,
    /// The host address (where nymd is running)
    #[clap(long)]
    host: String,
    /// The version of the mixnode
    #[clap(long)]
    version: String,
    /// The Sequence number to sue for the offline signature
    #[clap(long)]
    sequence_number: u64,
    /// Account number to use for the offline signature
    #[clap(long)]
    account_number: u64,
    // advanced options
    #[clap(long)]
    mix_port: Option<u16>,
    #[clap(long)]
    verloc_port: Option<u16>,
    #[clap(long)]
    http_api_port: Option<u16>,
}

pub(crate) fn execute(args: &Bond, network_info: NetworkInfo, wallet: DirectSecp256k1HdWallet) {
    println!("Attempt to bond to mixnode {}...", args.identity_key);
    let mix_node = MixNode {
        host: args.host.clone(),
        mix_port: args.mix_port.unwrap_or(DEFAULT_MIX_PORT),
        verloc_port: args.verloc_port.unwrap_or(DEFAULT_VERLOC_PORT),
        http_api_port: args.http_api_port.unwrap_or(DEFAULT_HTTP_API_PORT),
        sphinx_key: args.sphinx_key.clone(),
        identity_key: args.identity_key.clone(),
        version: args.version.clone(),
        profit_margin_percent: args.profit_margin_percent,
    };

    let config =
        Config::try_from_nym_network_details(&network_info.network_details).expect("no config");

    // TODO support non-offline stuff too...
    let offline_signer = Client::new_offline_signing(config, wallet);

    let pledge: Coin = Coin {
        amount: args.pledge_amount,
        denom: network_info
            .network_details
            .chain_details
            .mix_denom
            .base
            .clone(),
    };
    let result = offline_signer.nymd.execute_offline_bond_mixnode(
        mix_node,
        args.owner_signature.clone(),
        pledge,
        args.account_number,
        args.sequence_number,
        network_info.chain_id.parse().expect("Invalid Chain ID"),
    );

    match result {
        Ok(response) => {
            let json = serde_json::to_string(&response.data).unwrap();
            println!("Bond Transaction: {:x?}", json)
        }
        Err(error) => println!("Failed to send transaction with Error: {}", error),
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::bond::*;
    use network_defaults::all::Network::SANDBOX;

    #[test]
    fn test_generate_tx() {
        let mnemonic = "drill poet latin puzzle fork lift rocket magic width hello radio glue loop electric jacket guide job goat dust provide input spoon wall thumb";
        let args = Bond {
            identity_key: "9c8rzttr9xzP1qfzVBFUaUN3jHREshKDnn3gugg7uhwA".to_string(),
            sphinx_key: "6AtSiDL7MvsDudhqiik6DVsKqyMjLzR5r9kuHewHab1k".to_string(),
            owner_signature: "4zka4Ne4mkmpu93g8ASGLBixYRDqwmRoh55sstW7PeQA5yhToRwKAF8WXpp9M1ekqdDdQMDnguMiTDokeWe96wB2".to_string(),
            profit_margin_percent: 100,
            pledge_amount: 1_000_000_000_000,
            host: "192.168.1.1".to_string(),
            version: "1.0".to_string(),
            account_number: 360,
            sequence_number: 2,
            mix_port: None,
            verloc_port: None,
            http_api_port: None,
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
