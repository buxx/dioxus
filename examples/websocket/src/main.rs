#![allow(non_snake_case)]
use dioxus::prelude::*;

#[cfg(feature = "server")]
use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket},
    routing::any,
    response::{IntoResponse, Response},
    Router,
};

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    init_server(App).await;
}

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! { "Hello" }
}


// The entry point for the server
#[cfg(feature = "server")]
pub async fn init_server(app: fn() -> Element) {
    let address = dioxus_cli_config::fullstack_address_or_localhost();

    let cfg = ServeConfigBuilder::default();
    let router = axum::Router::new()
        .serve_dioxus_application(cfg, app)
        .route("/echo", any(handler));

    let router = router.into_make_service();
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    
    axum::serve(listener, router).await.unwrap();
}

#[cfg(feature = "server")]
async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

#[cfg(feature = "server")]
async fn handle_socket(mut socket: WebSocket) {
    println!("HANDLE");
    while let Some(msg) = socket.recv().await {
        let msg = match msg {
            Ok(msg) => msg,
            Err(error) => {
                println!("DISCONNECTED on error: {}", error);
                return;
            }
        };

        if let Err(error) = socket.send(msg).await {
            println!("DISCONNECTED on send error: {}", error);
            return;
        }
    }
    println!("FINISHED");
}