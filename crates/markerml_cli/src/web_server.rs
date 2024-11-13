use crate::common;
use anyhow::{Context, Result};
use arc_swap::ArcSwap;
use async_channel::{Receiver, Sender};
use axum::body::Body;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use notify::{RecursiveMode, Watcher};
use serde::Serialize;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Application state
#[derive(Clone)]
struct AppState {
    /// Current code. Used for establishing new web socket connection
    pub current: Arc<ArcSwap<CodeUpdateMessage>>,
    /// Channel for receiving code updates
    pub updates: Receiver<Arc<CodeUpdateMessage>>,
}

/// Message with code
#[derive(Serialize)]
#[serde(untagged)]
enum CodeUpdateMessage {
    /// Contains valid compiled code
    Code { code: String },
    /// Contains information about compilation error
    Error { error: String },
    /// Used for empty files
    Empty,
}

/// Embedded live-reloading html page
const INDEX_HTML: &str = include_str!("../web/index.html");
/// Embedded styles for live-reloading html page
const STYLE: &str = include_str!("../web/style.css");
/// Embedded script for live reloading
const SCRIPT: &str = include_str!("../web/script.js");

/// Start the web server watching specified file with code
pub async fn run_web_server(filename: impl AsRef<Path>, port: u16) -> Result<()> {
    let (tx, rx) = async_channel::unbounded();
    let page = Arc::new(ArcSwap::from_pointee(CodeUpdateMessage::Empty));
    let app = Router::new()
        .route("/listen", get(listen))
        .route("/", get(index_html))
        .route("/script.js", get(script))
        .route("/style.css", get(style))
        .with_state(AppState {
            current: page.clone(),
            updates: rx,
        });

    let filename = filename.as_ref().to_owned();
    tokio::spawn(async move { watch_file(filename, page, tx).await });

    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("Couldn't start web server")?;
    axum::serve(listener, app)
        .await
        .context("Couldn't start web server")?;

    Ok(())
}

/// Endpoint for live-reloading html page
async fn index_html() -> Html<&'static str> {
    Html(INDEX_HTML)
}

/// Endpoint for live-reloading script
async fn script() -> impl IntoResponse {
    Body::from(SCRIPT)
}

/// Endpoint for live-reloading html page style
async fn style() -> impl IntoResponse {
    Body::from(STYLE)
}

/// Endpoint for connecting to websocket that notifies when code changes
async fn listen(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Function that watches file changes and compiles code on demand.
/// Updates are reflected in the application state.
async fn watch_file(
    filename: PathBuf,
    page: Arc<ArcSwap<CodeUpdateMessage>>,
    updates: Sender<Arc<CodeUpdateMessage>>,
) -> Result<()> {
    let (tx, rx) = async_channel::unbounded();

    let mut watcher = notify::recommended_watcher(move |event: notify::Result<notify::Event>| {
        let Ok(event) = event else { return };
        if let notify::EventKind::Modify(_) = event.kind {
            tx.send_blocking(()).unwrap()
        }
    })
    .context("Couldn't watch file changes")?;

    watcher
        .watch(&filename, RecursiveMode::NonRecursive)
        .context("Couldn't watch file changes")?;

    let update_code = || async {
        let res = Arc::new(match common::parse_file(&filename) {
            Ok(code) => {
                println!("Code updated!");
                CodeUpdateMessage::Code { code }
            }
            Err(err) => {
                println!("Compilation error: {err}");
                CodeUpdateMessage::Error {
                    error: err.to_string(),
                }
            }
        });

        updates.send(res.clone()).await?;
        page.store(res);

        Ok::<(), anyhow::Error>(())
    };

    update_code().await?;
    loop {
        rx.recv().await?;

        update_code().await?;
    }
}

/// Handles websocket connection:
/// - Sends initial code on connection
/// - Sends any update when the code is changed
async fn handle_socket(mut socket: WebSocket, state: AppState) {
    {
        let result = state.current.load().clone();
        let message = serde_json::to_string(result.as_ref()).unwrap_or_default();

        socket.send(Message::Text(message)).await.unwrap();
    }

    while let Ok(result) = state.updates.recv().await {
        let message = serde_json::to_string(result.as_ref()).unwrap_or_default();

        socket.send(Message::Text(message)).await.unwrap();
    }
}
