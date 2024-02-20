use std::time::Duration;

use p2p_handshake_solana::{
    domain::{DataReceive, DataSend},
    solana_client::{SolanaClient, SolanaClientError},
};
use wiremock::{matchers::method, Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn handshake_returns_200_for_valid_form_data() {
    let mock_server = MockServer::start().await;
    let solana_client = SolanaClient::new(mock_server.uri());
    let data = DataSend::default();
    let data_receive_default = DataReceive::default();

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(data_receive_default))
        .mount(&mock_server)
        .await;
    let response = solana_client.get_version(data).await.unwrap();

    assert!(matches!(response, _data_receive_default));
}

#[tokio::test]
async fn remote_node_returns_500_for_internal_server_error() {
    let mock_server = MockServer::start().await;
    let solana_client = SolanaClient::new(mock_server.uri());
    let data = DataSend::default();

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;
    let response = solana_client.get_version(data).await;

    assert!(matches!(
        response,
        Err(SolanaClientError::HttpResponseError)
    ));
}

#[tokio::test]
async fn remote_node_returns_408_for_connection_timeout_error() {
    let mock_server = MockServer::start().await;
    let solana_client = SolanaClient::new(mock_server.uri());
    let data = DataSend::default();

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(408))
        .mount(&mock_server)
        .await;
    let response = solana_client.get_version(data).await;

    assert!(matches!(
        response,
        Err(SolanaClientError::HttpResponseError)
    ));
}

#[tokio::test]
async fn remote_node_returns_data_after_connection_timeout() {
    let mock_server = MockServer::start().await;
    let solana_client = SolanaClient::new(mock_server.uri());
    let data = DataSend::default();

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_millis(300)))
        .mount(&mock_server)
        .await;
    let response = solana_client.get_version(data).await;
    assert!(matches!(
        response,
        Err(SolanaClientError::UnexpectedError(_))
    ));
}
// TODO Wrong data test
