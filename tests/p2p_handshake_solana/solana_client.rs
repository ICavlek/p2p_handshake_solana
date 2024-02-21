use std::time::Duration;

use p2p_handshake_solana::{
    domain::{DataReceive, DataReceiveError, DataSend},
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

#[tokio::test]
async fn solana_client_returns_error_for_wrong_data_provided() {
    let mock_server = MockServer::start().await;
    let solana_client = SolanaClient::new(mock_server.uri());
    let wrong_data = DataSend::new("wrongMethodName".to_string());
    let wrong_data_default_response = DataReceiveError::default();

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(wrong_data_default_response))
        .mount(&mock_server)
        .await;
    let response = solana_client.get_version(wrong_data).await;

    assert!(matches!(response, Err(SolanaClientError::SentDataError)));
}

#[tokio::test]
async fn solana_client_returns_unexpected_error_for_corrupted_data() {
    let mock_server = MockServer::start().await;
    let solana_client = SolanaClient::new(mock_server.uri());
    let correct_data = DataSend::default();
    let corrupt_data_respond = DataReceive::new("PotentiallyHazardousData".to_string());

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(corrupt_data_respond))
        .mount(&mock_server)
        .await;
    let response = solana_client.get_version(correct_data).await;

    assert!(matches!(
        response,
        Err(SolanaClientError::UnexpectedError(_))
    ));
}
