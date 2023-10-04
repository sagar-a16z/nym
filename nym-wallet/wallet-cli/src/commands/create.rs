use crate::NetworkInfo;
use clap::Args;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use validator_client::nymd::wallet::DirectSecp256k1HdWallet;

#[derive(Args, Clone)]
pub(crate) struct Create {
    /// Optional argument to specify the number of words in the BIP39 seed phrase
    #[clap(long, short)]
    words: Option<usize>,
}

pub(crate) fn execute(args: &Create, network_info: NetworkInfo) -> Option<PathBuf> {
    let prefix = network_info
        .network_details
        .chain_details
        .bech32_account_prefix;
    let wallet = DirectSecp256k1HdWallet::generate(prefix.as_str(), args.words.unwrap_or(24));
    match wallet {
        Ok(wallet) => {
            let accounts = wallet.try_derive_accounts().unwrap();
            let account_id = accounts[0].address();
            let filename = format!("{}{}", account_id, "_seed_phrase.txt");
            let path = Path::new(&filename);
            let mut file =
                File::create(path).expect("Unable to create file to store wallet seed phrase");
            file.write_all(wallet.mnemonic().as_bytes())
                .expect("Failed to write seed phrase");
            println!(
                "Created Account: {}\nWrote seed phrase to {}",
                account_id,
                path.display()
            );
            return Some(PathBuf::from(path));
        }
        Err(error) => {
            println!("Failed to create new Wallet: {}", error);
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::create::*;
    use network_defaults::all::Network::SANDBOX;
    use std::io::Read;

    #[test]
    fn test_wallet_create() {
        let args = Create { words: Some(12) };
        let network_info = NetworkInfo {
            network_details: SANDBOX.details(),
            chain_id: "nym-sandbox".to_string(),
        };
        let path = execute(&args, network_info);

        // Check that a wallet file was created
        let file = File::open(path.unwrap());
        assert!(file.is_ok());
        let mut mnemonic = String::new();
        assert!(file.unwrap().read_to_string(&mut mnemonic).is_ok());
        let count = mnemonic.split_whitespace().map(|_| 1).sum();
        assert_eq!(Some(count), args.words);
    }
}