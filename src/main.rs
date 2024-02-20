use anyhow::Context;

use p2p_handshake_solana::solana_client::SolanaClient;
use p2p_handshake_solana::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber(
        "p2p_handshake_solana".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    // TODO Parse uri and timeout argument
    let uri = "http://127.0.0.1:8899";
    // TODO Document class
    let solana_client = SolanaClient::new(uri.to_owned());
    solana_client
        .handshake()
        .await
        .context("Failed to perform handshake")?;
    tracing::info!("Handshake completed succesfully");
    Ok(())
}
