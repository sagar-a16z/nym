// Copyright 2021 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

use crate::{DenomDetails, ValidatorDetails};

pub(crate) const BECH32_PREFIX: &str = "n";

pub(crate) const MIX_DENOM: DenomDetails = DenomDetails::new("unym", "nym", 6);
pub(crate) const STAKE_DENOM: DenomDetails = DenomDetails::new("unyx", "nyx", 6);

pub(crate) const BANDWIDTH_CLAIM_CONTRACT_ADDRESS: &str =
    "n1pd7kfgvr5tpcv0xnlv46c4jsq9jg2r799xxrcwqdm4l2jhq2pjwqrmz5ju";
pub(crate) const REWARDING_VALIDATOR_ADDRESS: &str =
    "n1ahg0erc2fs6xx3j5m8sfx3ryuzdjh6kf6qm9plsf865fltekyrfsesac6a";
pub(crate) const MIXNET_CONTRACT_ADDRESS: &str =
    "n1xr3rq8yvd7qplsw5yx90ftsr2zdhg4e9z60h5duusgxpv72hud3sjkxkav";
pub(crate) const VESTING_CONTRACT_ADDRESS: &str =
    "n1unyuj8qnmygvzuex3dwmg9yzt9alhvyeat0uu0jedg2wj33efl5qackslz";
pub(crate) const COCONUT_BANDWIDTH_CONTRACT_ADDRESS: &str =
    "n16a32stm6kknhq5cc8rx77elr66pygf2hfszw7wvpq746x3uffylqkjar4l";
pub(crate) const MULTISIG_CONTRACT_ADDRESS: &str =
    "n14ph4e660eyqz0j36zlkaey4zgzexm5twkmjlqaequxr2cjm9eprqsmad6k";

pub(crate) const STATISTICS_SERVICE_DOMAIN_ADDRESS: &str = "http://0.0.0.0";
pub(crate) fn validators() -> Vec<ValidatorDetails> {
    vec![ValidatorDetails::new(
        "https://sandbox-validator1.nymtech.net",
        Some("https://sandbox-nym-api1.nymtech.net/api"),
    )]
}
