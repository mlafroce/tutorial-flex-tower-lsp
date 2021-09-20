mod service;
mod token_map;

use crate::service::lsp::Backend;
use tokio::net::TcpListener;
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:9257").await.unwrap();
    let (stream, _) = listener.accept().await.unwrap();

    let (read, write) = tokio::io::split(stream);

    let (service, messages) = LspService::new(Backend::new);
    Server::new(read, write)
        .interleave(messages)
        .serve(service)
        .await;
}
