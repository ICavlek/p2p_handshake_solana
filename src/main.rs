use clap::Parser;

use p2p_handshake_solana::parser_arguments::Arguments;
use p2p_handshake_solana::solana::client::SolanaClient;
use p2p_handshake_solana::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();

    let subscriber = get_subscriber(
        "p2p_handshake_solana".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let solana_client = SolanaClient::new(args.uri, args.timeout);
    match solana_client.handshake().await {
        Ok(_) => tracing::info!("Successfully performed handshake"),
        Err(e) => {
            tracing::error!("Error performing handshake: {}", e);
            return Err(e);
        }
    };

    Ok(())
}
