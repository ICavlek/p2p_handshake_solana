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
    let solana_client = SolanaClient::new();
    solana_client
        .handshake()
        .await
        .context("Failed to perform handshake")?;
    Ok(())
}
