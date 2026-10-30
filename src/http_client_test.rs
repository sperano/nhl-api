use super::{Endpoint, HttpClient, MAX_ERROR_BODY_BYTES, MAX_REDIRECTS};
use crate::config::ClientConfig;
use crate::error::NHLApiError;

/// UTF-8 replacement character `String::from_utf8_lossy` substitutes for
/// invalid byte sequences.
const REPLACEMENT_CHAR: char = '\u{FFFD}';

/// Response type for requests whose body is never meant to deserialize.
#[derive(Debug, serde::Deserialize)]
struct Empty {}

fn http_client(config: ClientConfig) -> HttpClient {
    HttpClient::new(config).unwrap()
}

async fn get(
    client: &HttpClient,
    server: &mockito::Server,
    resource: &str,
) -> Result<Empty, NHLApiError> {
    client
        .get_json(Endpoint::Custom(server.url()), resource, None)
        .await
}

// ===== Origin override =====

#[test]
fn test_base_url_origin_override_keeps_endpoint_path() {
    let client = http_client(ClientConfig::default()).with_origin_override("http://127.0.0.1:1/");
    let cases = [
        (Endpoint::ApiWebV1, "http://127.0.0.1:1/v1/"),
        (Endpoint::ApiCore, "http://127.0.0.1:1/"),
        (Endpoint::ApiStats, "http://127.0.0.1:1/stats/rest/"),
        (Endpoint::SearchV1, "http://127.0.0.1:1/api/v1/"),
    ];
    for (endpoint, expected) in cases {
        assert_eq!(client.base_url(&endpoint), expected, "{endpoint:?}");
    }
}

#[test]
fn test_base_url_without_override_is_endpoint_base() {
    let client = http_client(ClientConfig::default());
    assert_eq!(
        client.base_url(&Endpoint::ApiWebV1),
        Endpoint::ApiWebV1.base_url()
    );
}

// ===== Error bodies that are not valid UTF-8 =====

#[tokio::test]
async fn test_get_json_non_utf8_error_body_is_lossily_decoded() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("GET", "/binary-error")
        .with_status(500)
        .with_body(b"bad \xff\xfe bytes".as_slice())
        .create_async()
        .await;

    let result = get(
        &http_client(ClientConfig::default()),
        &server,
        "binary-error",
    )
    .await;

    match result.unwrap_err() {
        NHLApiError::ServerError { message, .. } => {
            let expected = format!("bad {REPLACEMENT_CHAR}{REPLACEMENT_CHAR} bytes");
            assert!(message.ends_with(&expected), "got: {message}");
        }
        other => panic!("Expected ServerError, got {other:?}"),
    }
}

#[tokio::test]
async fn test_get_json_error_body_cut_inside_multibyte_char() {
    // The cap falls between the two bytes of 'é', leaving a lone lead byte.
    let body = format!("{}é", "a".repeat(MAX_ERROR_BODY_BYTES - 1));
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("GET", "/split-char")
        .with_status(400)
        .with_body(&body)
        .create_async()
        .await;

    let result = get(&http_client(ClientConfig::default()), &server, "split-char").await;

    match result.unwrap_err() {
        NHLApiError::BadRequest { message, .. } => {
            let expected = format!("a{REPLACEMENT_CHAR}");
            assert!(message.ends_with(&expected), "got: {message}");
        }
        other => panic!("Expected BadRequest, got {other:?}"),
    }
}

// ===== Redirect policy =====

const REDIRECT_STATUS: usize = 302;

async fn redirect_to_target(server: &mut mockito::Server, expected_hits: usize) -> mockito::Mock {
    server
        .mock("GET", "/old")
        .with_status(REDIRECT_STATUS)
        .with_header("location", "/new")
        .create_async()
        .await;
    server
        .mock("GET", "/new")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{}")
        .expect(expected_hits)
        .create_async()
        .await
}

#[tokio::test]
async fn test_get_json_follows_redirect_when_enabled() {
    let mut server = mockito::Server::new_async().await;
    let target = redirect_to_target(&mut server, 1).await;
    let client = http_client(ClientConfig::default().with_follow_redirects(true));

    let result = get(&client, &server, "old").await;

    assert!(
        result.is_ok(),
        "redirect should be followed: {:?}",
        result.err()
    );
    target.assert_async().await;
}

#[tokio::test]
async fn test_get_json_does_not_follow_redirect_when_disabled() {
    let mut server = mockito::Server::new_async().await;
    let target = redirect_to_target(&mut server, 0).await;
    let client = http_client(ClientConfig::default().with_follow_redirects(false));

    let result = get(&client, &server, "old").await;

    match result.unwrap_err() {
        NHLApiError::ApiError { status_code, .. } => {
            assert_eq!(usize::from(status_code), REDIRECT_STATUS);
        }
        other => panic!("Expected ApiError for the unfollowed redirect, got {other:?}"),
    }
    target.assert_async().await;
}

#[tokio::test]
async fn test_get_json_stops_after_max_redirects() {
    let mut server = mockito::Server::new_async().await;
    let redirect_loop = server
        .mock("GET", "/loop")
        .with_status(REDIRECT_STATUS)
        .with_header("location", "/loop")
        .expect(MAX_REDIRECTS + 1)
        .create_async()
        .await;
    let client = http_client(ClientConfig::default().with_follow_redirects(true));

    let result = get(&client, &server, "loop").await;

    assert!(
        matches!(result, Err(NHLApiError::RequestError(_))),
        "expected a redirect-limit RequestError, got {result:?}"
    );
    redirect_loop.assert_async().await;
}
