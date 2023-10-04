use crate::NetworkInfo;
use clap::Args;
use clap::Subcommand;
use mixnet_contract_common::Coin;
use std::fmt;
use validator_client::nymd::wallet::DirectSecp256k1HdWallet;
use validator_client::nymd::VestingQueryClient;
use validator_client::{Client, Config};

#[derive(Subcommand, Clone)]
enum Operation {
    /// Check the vested balance
    Balance(BalanceArgs),
    /// Withdraw vested rewards
    WithdrawVested(WithdrawArgs),
}

#[derive(Args, Clone)]
struct BalanceArgs {
    /// Account address, only used when checking rewards balance
    #[clap(long)]
    account_address: String,
}

#[derive(Args, Clone)]
struct NonceArgs {
    /// The Sequence number to use for the offline signature
    #[clap(long)]
    sequence_number: u64,
    /// Account number to use for the offline signature
    #[clap(long)]
    account_number: u64,
}

#[derive(Args, Clone)]
struct WithdrawArgs {
    /// Number of args
    #[clap(long)]
    amount: u128,
    #[clap(flatten)]
    nonce: NonceArgs,
}

#[derive(Args, Clone)]
pub(crate) struct Vesting {
    /// The reward operation to perform
    #[clap(subcommand)]
    operation: Operation,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operation::Balance(_) => write!(f, "Vested Balance"),
            Operation::WithdrawVested(_) => write!(f, "Withdraw Vested"),
        }
    }
}

pub(crate) async fn execute(
    args: &Vesting,
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
                .vested_coins(&balance_args.account_address, None)
                .await;
            match result {
                Ok(rewards) => println!("Vested Balance: {} {}", rewards.amount, rewards.denom),
                Err(error) => {
                    println!("Failed to check balance with Error: {}", error)
                }
            }
            return;
        }
        Operation::WithdrawVested(withdraw_args) => {
            let offline_signer =
                Client::new_offline_signing(config, wallet.expect("Invalid Wallet"));

            let amount = Coin::new(
                withdraw_args.amount,
                network_info
                    .network_details
                    .chain_details
                    .mix_denom
                    .base
                    .clone(),
            );
            offline_result = offline_signer.nymd.execute_offline_withdraw_vested(
                amount,
                withdraw_args.nonce.account_number,
                withdraw_args.nonce.sequence_number,
                network_info.chain_id.parse().expect("Invalid Chain ID"),
            );
        }
    }

    match offline_result {
        Ok(response) => {
            let json = serde_json::to_string(&response.data).unwrap();
            println!("{} Vesting Transaction: {:x?}", args.operation, json);
        }
        Err(error) => println!("Failed to generate transaction with Error: {}", error),
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::rewards::*;
    use crate::commands::vesting::{Operation, WithdrawArgs};
    use network_defaults::all::Network::SANDBOX;

    #[tokio::test]
    async fn test_generate_withdraw_tx() {
        let mnemonic = "drill poet latin puzzle fork lift rocket magic width hello radio glue loop electric jacket guide job goat dust provide input spoon wall thumb";

        let args = Vesting {
            operation: Operation::WithdrawVested(WithdrawArgs {
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
    async fn test_check_vested_balance() {
        let args = Vesting {
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
