use crate::NetworkInfo;
use clap::Args;
use validator_client::{Client, Config};

#[derive(Args, Clone)]
pub(crate) struct Send {
    /// The host address (where nymd is running)
    #[clap(long)]
    host: Option<String>,
    /// The signed transaction as serde_json encoded string
    #[clap(long)]
    transaction: String,

    // advanced options
    #[clap(long)]
    mix_port: Option<u16>,
    #[clap(long)]
    verloc_port: Option<u16>,
    #[clap(long)]
    http_api_port: Option<u16>,
}

pub(crate) async fn execute(args: &Send, network_info: NetworkInfo) {
    // setup a client, and look up the account info.
    let mut config =
        Config::try_from_nym_network_details(&network_info.network_details).expect("no config");
    if let Some(host) = &args.host {
        config = config.with_nymd_url(host.parse().expect("couldn't parse host url"));
    }

    let client = Client::new_query(config).expect("Unable to connect to Network");
    let raw_tx =
        serde_json::from_str(args.transaction.as_str()).expect("unable to parse transaction");
    let result = client.nymd.broadcast_tx(raw_tx).await;
    match result {
        Ok(response) => println!(
            "Sent Tx. Tx Hash: {}, Index: {}, Height: {}",
            response.hash, response.index, response.height
        ),
        Err(error) => println!("Failed to send transaction with Error: {}", error),
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::sender::*;
    use network_defaults::all::Network::SANDBOX;

    #[tokio::test]
    async fn test_send_transaction() {
        let args = Send {
            host: None,
            transaction: "\"CsMECpwECiQvY29zbXdhc20ud2FzbS52MS5Nc2dFeGVjdXRlQ29udHJhY3QS8wMKK255bXQxY2g4bXM5YzZtYWd0NHZ5eHZ5eWs4dmp3cWVxNDNmanNqcDlocDISK255bXQxZ2hkNzUzc2hqdXdleHh5d21nczR4ejd4MnE3MzJ2Y25zdHowMmoagAN7ImJvbmRfbWl4bm9kZSI6eyJtaXhfbm9kZSI6eyJob3N0IjoiMTAwLjk0LjI1Ni41IiwibWl4X3BvcnQiOjE3ODksInZlcmxvY19wb3J0IjoxNzkwLCJodHRwX2FwaV9wb3J0Ijo4MDAwLCJzcGhpbnhfa2V5IjoiNkF0U2lETDdNdnNEdWRocWlpazZEVnNLcXlNakx6UjVyOWt1SGV3SGFiMWsiLCJpZGVudGl0eV9rZXkiOiI5YzhyenR0cjl4elAxcWZ6VkJGVWFVTjNqSFJFc2hLRG5uM2d1Z2c3dWh3QSIsInZlcnNpb24iOiIxLjAiLCJwcm9maXRfbWFyZ2luX3BlcmNlbnQiOjEwMH0sIm93bmVyX3NpZ25hdHVyZSI6IjR6a2E0TmU0bWttcHU5M2c4QVNHTEJpeFlSRHF3bVJvaDU1c3N0VzdQZVFBNXloVG9Sd0tBRjhXWHBwOU0xZWtxZERkUU1Ebmd1TWlURG9rZVdlOTZ3QjIifX0qFAoEbnltdBIMMTAwMDAwMDAwMDAwEiJPZmZsaW5lIEJvbmRpbmcgbWl4bm9kZSBmcm9tIHJ1c3QhEmIKUApGCh8vY29zbW9zLmNyeXB0by5zZWNwMjU2azEuUHViS2V5EiMKIQIT9ck0t4xjKQUAvXoKW2Tk6EblwYoI2BKd4fYdeYYxRRIECgIIARgBEg4KCgoETnltdBICMTAQBRpAeJk4KEf4wXIwGt3THV7oSzCMFIgR5O8d5AXMGyk+dX8Or8mvWYf6WIX9W+duL3prx/EfLps5S+MwH05kMmNj6g==\"".to_string(),
            mix_port: None,
            verloc_port: None,
            http_api_port: None,
        };

        execute(
            &args,
            NetworkInfo {
                network_details: SANDBOX.details(),
                chain_id: "nym-sandbox".to_string(),
            },
        )
        .await;
    }
}
