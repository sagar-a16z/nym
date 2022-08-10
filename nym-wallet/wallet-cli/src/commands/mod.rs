// Copyright 2020 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

use crate::Args;
use clap::Subcommand;
use network_defaults::NymNetworkDetails;
use validator_client::nymd::wallet::DirectSecp256k1HdWallet;

mod account;
mod balance;
mod bond;
mod delegate;
mod sender;
mod undelegate;

static DEFAULT_MIX_PORT: u16 = 1789;
static DEFAULT_VERLOC_PORT: u16 = 1790;
static DEFAULT_HTTP_API_PORT: u16 = 8000;

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Generate and Sign an offline bonding transaction
    BondOffline(bond::Bond),
    /// Get the Sequence and Account numbers for a given account
    AccountSequence(account::Account),
    /// Send a signed transaction encoded as a serde_json string
    SendTransaction(sender::Send),
    /// Delegate tokens to a node
    DelegateOffline(delegate::Delegate),
    /// Undelegate tokens from a node
    UndelegateOffline(undelegate::Undelegate),
    /// Get the balance and delegations of an account
    Balance(balance::Balance), // UnBond(unbond::UnBond),
}

pub(crate) struct NetworkInfo {
    pub network_details: NymNetworkDetails,
    pub chain_id: String,
}

pub(crate) async fn execute(
    args: Args,
    network_info: NetworkInfo,
    wallet: Option<DirectSecp256k1HdWallet>,
) {
    match &args.command {
        Commands::BondOffline(m) => {
            bond::execute(m, network_info, wallet.expect("Invalid Wallet")).await
        }
        Commands::AccountSequence(m) => account::execute(m, network_info).await,
        Commands::SendTransaction(m) => sender::execute(m, network_info).await,
        Commands::DelegateOffline(m) => {
            delegate::execute(m, network_info, wallet.expect("Invalid Wallet")).await
        }
        Commands::UndelegateOffline(m) => {
            undelegate::execute(m, network_info, wallet.expect("Invalid Wallet")).await
        }
        Commands::Balance(m) => balance::execute(m, network_info).await,
    }
}
