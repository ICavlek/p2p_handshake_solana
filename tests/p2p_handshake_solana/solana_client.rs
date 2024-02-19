use p2p_handshake_solana::solana_client::SolanaClient;
use wiremock::{matchers::method, Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn handshake_returns_200_for_valid_form_data() {
    let mock_server = MockServer::start().await;
    let solana_client = SolanaClient::new(mock_server.uri());

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
    let response = solana_client.get_version().await.unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

// TODO Connection time out test
// TODO Connection failed test
// TODO Wrong data test
// TODO Correct data test
