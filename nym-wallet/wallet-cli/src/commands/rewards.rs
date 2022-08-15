use crate::NetworkInfo;
use clap::Args;
use clap::Subcommand;
use std::fmt;
use validator_client::nymd::wallet::DirectSecp256k1HdWallet;
use validator_client::{Client, Config};

#[derive(Subcommand, Clone)]
enum Operation {
    /// Check the rewards balance
    Balance(BalanceArgs),
    /// Compound rewards
    CompoundOffline(RewardsArgs),
    /// Claim rewards
    ClaimOffline(RewardsArgs),
}

#[derive(Args, Clone)]
struct BalanceArgs {
    /// Account address, only used when checking rewards balance
    #[clap(long)]
    account_address: String,
}

#[derive(Args, Clone)]
struct RewardsArgs {
    /// The Sequence number to sue for the offline signature
    #[clap(long)]
    sequence_number: u64,
    /// Account number to use for the offline signature
    #[clap(long)]
    account_number: u64,
}

#[derive(Args, Clone)]
pub(crate) struct Rewards {
    /// The Mixnode Identity to Delegate tokens
    #[clap(long)]
    mixnode_identity: String,
    /// The reward operation to perform
    #[clap(subcommand)]
    operation: Operation,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operation::Balance(_) => write!(f, "Rewards Balance"),
            Operation::CompoundOffline(_) => write!(f, "Compound Offline"),
            Operation::ClaimOffline(_) => write!(f, "Claim Offline"),
        }
    }
}

pub(crate) async fn execute(
    args: &Rewards,
    network_info: NetworkInfo,
    wallet: Option<DirectSecp256k1HdWallet>,
) {
    // setup a client, and look up the account info.
    let config =
        Config::try_from_nym_network_details(&network_info.network_details).expect("no config");

    let offline_result;
    match &args.operation {
        Operation::Balance(balance_args) => {
            let client = Client::new_query(config).expect("Unable to connect to network");
            let result = client
                .nymd
                .get_delegator_rewards(
                    balance_args.account_address.clone(),
                    args.mixnode_identity.clone(),
                    None,
                )
                .await;
            match result {
                Ok(rewards) => println!(
                    "Rewards Balance: {} {}",
                    rewards, network_info.network_details.chain_details.mix_denom.base
                ),
                Err(error) => {
                    println!("Failed to check balance with Error: {}", error)
                }
            }
            return;
        }
        Operation::CompoundOffline(rewards_args) => {
            let offline_signer =
                Client::new_offline_signing(config, wallet.expect("Invalid Wallet"));
            offline_result = offline_signer
                .nymd
                .compound_mixnet_delegator_reward_offline(
                    args.mixnode_identity.clone(),
                    rewards_args.account_number,
                    rewards_args.sequence_number,
                    network_info.chain_id.parse().expect("Invalid Chain ID"),
                );
        }
        Operation::ClaimOffline(rewards_args) => {
            let offline_signer =
                Client::new_offline_signing(config, wallet.expect("Invalid Wallet"));
            offline_result = offline_signer.nymd.claim_mixnet_delegator_reward_offline(
                args.mixnode_identity.clone(),
                rewards_args.account_number,
                rewards_args.sequence_number,
                network_info.chain_id.parse().expect("Invalid Chain ID"),
            );
        }
    }

    match offline_result {
        Ok(response) => {
            let json = serde_json::to_string(&response.data).unwrap();
            println!("{} Reward Transaction: {:x?}", args.operation, json);
        }
        Err(error) => println!("Failed to generate transaction with Error: {}", error),
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::rewards::*;
    use network_defaults::all::Network::SANDBOX;

    #[tokio::test]
    async fn test_generate_compound_tx() {
        let mnemonic = "drill poet latin puzzle fork lift rocket magic width hello radio glue loop electric jacket guide job goat dust provide input spoon wall thumb";

        let args = Rewards {
            mixnode_identity: "HgtcRA3JF29NYgxgv9ysALjTF9dbnEr4b9HvmcmQN6fw".to_string(),
            operation: Operation::CompoundOffline(RewardsArgs {
                sequence_number: 0,
                account_number: 450,
            }),
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
            Some(wallet),
        )
        .await;
    }

    #[tokio::test]
    async fn test_generate_claim() {
        let mnemonic = "drill poet latin puzzle fork lift rocket magic width hello radio glue loop electric jacket guide job goat dust provide input spoon wall thumb";

        let args = Rewards {
            mixnode_identity: "HgtcRA3JF29NYgxgv9ysALjTF9dbnEr4b9HvmcmQN6fw".to_string(),
            operation: Operation::ClaimOffline(RewardsArgs {
                sequence_number: 0,
                account_number: 450,
            }),
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
            Some(wallet),
        )
        .await;
    }

    #[tokio::test]
    async fn test_check_rewards_balance() {
        let args = Rewards {
            mixnode_identity: "HgtcRA3JF29NYgxgv9ysALjTF9dbnEr4b9HvmcmQN6fw".to_string(),
            operation: Operation::Balance(BalanceArgs {
                account_address: "nymt129zh9a59lhp87sxf8ax8tljzsz2tn0yg2z0l30".to_string(),
            }),
        };

        execute(
            &args,
            NetworkInfo {
                network_details: SANDBOX.details(),
                chain_id: "nym-sandbox".to_string(),
            },
            None,
        )
        .await;
    }
}
