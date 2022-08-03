// Copyright 2020 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

use crate::Args;
use clap::Subcommand;
use network_defaults::NymNetworkDetails;
use validator_client::nymd::wallet::DirectSecp256k1HdWallet;

mod account;
mod bond;
mod sender;
// mod unbond;

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
    // UnBond(unbond::UnBond),
}

pub(crate) async fn execute(
    args: Args,
    network_details: NymNetworkDetails,
    wallet: Option<DirectSecp256k1HdWallet>,
) {
    match &args.command {
        Commands::BondOffline(m) => {
            bond::execute(m, network_details, wallet.expect("Invalid Wallet")).await
        }
        Commands::AccountSequence(m) => account::execute(m, network_details).await,
        Commands::SendTransaction(m) => sender::execute(m, network_details).await,
        // Commands::UnBond(m) => unbond::execute(m, wallet),
    }
}
