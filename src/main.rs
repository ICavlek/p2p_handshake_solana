use clap::Parser;

use p2p_handshake_solana::parser_arguments::Arguments;
use p2p_handshake_solana::solana::client_pool::SolanaClientPool;
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

    let solana_client_pool = SolanaClientPool::new(args.uri_nodes, args.timeout);
    solana_client_pool.run().await?;
    Ok(())
}
