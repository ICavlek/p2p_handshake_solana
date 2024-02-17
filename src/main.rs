use p2p_handshake_solana::solana_client::SolanaClient;

#[tokio::main]
async fn main() {
    let solana_client = SolanaClient::new();
    let response = solana_client.handshake().await;
    println!("{:#?}", response);
}
