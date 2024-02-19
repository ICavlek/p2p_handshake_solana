use p2p_handshake_solana::solana_client::{SolanaClient, SolanaClientError};
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

#[tokio::test]
async fn remote_node_returns_500_for_internal_server_error() {
    let mock_server = MockServer::start().await;
    let solana_client = SolanaClient::new(mock_server.uri());

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;
    let response = solana_client.handshake().await;
    assert!(matches!(
        response,
        Err(SolanaClientError::HttpResponseError)
    ));
}

#[tokio::test]
async fn remote_node_returns_408_for_connection_timeout_error() {
    let mock_server = MockServer::start().await;
    let solana_client = SolanaClient::new(mock_server.uri());

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(408))
        .mount(&mock_server)
        .await;
    let response = solana_client.handshake().await;
    assert!(matches!(
        response,
        Err(SolanaClientError::HttpResponseError)
    ));
}
// TODO Wrong data test
// TODO Correct data test
