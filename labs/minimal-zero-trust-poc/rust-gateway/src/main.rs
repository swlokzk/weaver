use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, Method, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};
use hmac::{Hmac, Mac};
use reqwest::Client;
use sha2::Sha256;
use std::{env, net::SocketAddr, sync::Arc};

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
struct AppState {
    trusted_client_fp: String,
    signing_secret: String,
    mcp_server_url: String,
    http_client: Client,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        trusted_client_fp: env::var("TRUSTED_CLIENT_CERT_FP").unwrap_or_else(|_| "demo-client-fp".to_string()),
        signing_secret: env::var("SIGNING_SECRET").unwrap_or_else(|_| "demo-signing-secret".to_string()),
        mcp_server_url: env::var("MCP_SERVER_URL").unwrap_or_else(|_| "http://127.0.0.1:8002".to_string()),
        http_client: Client::new(),
    });

    let app = Router::new()
        .route("/gateway/mcp", post(forward_mcp))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn forward_mcp(
    State(state): State<Arc<AppState>>,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let client_fp = headers
        .get("x-client-cert-fingerprint")
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default();

    if client_fp != state.trusted_client_fp {
        return (StatusCode::UNAUTHORIZED, "invalid mTLS client").into_response();
    }

    let signature = headers
        .get("x-signature")
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default();

    let signing_input = format!("{}\n{}\n{}", method, "/gateway/mcp", String::from_utf8_lossy(&body));
    if !verify_signature(&signing_input, &state.signing_secret, signature) {
        return (StatusCode::UNAUTHORIZED, "invalid signature").into_response();
    }

    let target_url = format!("{}/mcp", state.mcp_server_url);
    let upstream = state
        .http_client
        .post(target_url)
        .header("content-type", "application/json")
        .body(body)
        .send()
        .await;

    match upstream {
        Ok(resp) => {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_else(|_| "{}".to_string());
            (status, text).into_response()
        }
        Err(_) => (StatusCode::BAD_GATEWAY, "upstream error").into_response(),
    }
}

fn verify_signature(message: &str, secret: &str, provided_hex: &str) -> bool {
    let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_bytes()) else {
        return false;
    };
    mac.update(message.as_bytes());
    let expected = mac.finalize().into_bytes();

    match hex::decode(provided_hex) {
        Ok(provided) => expected.as_slice() == provided,
        Err(_) => false,
    }
}
