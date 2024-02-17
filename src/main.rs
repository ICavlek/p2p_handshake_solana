use p2p_handshake_solana::solana_client::SolanaClient;
use p2p_handshake_solana::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber(
        "p2p_handshake_solana".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);
    let solana_client = SolanaClient::new();
    let response = solana_client.handshake().await;
    println!("{:#?}", response);
}
