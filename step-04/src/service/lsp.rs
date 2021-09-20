use crate::service::lexer::{Lexer as LexerService, LexerRequest, LexerResponse};
use log::info;
use std::sync::Mutex;
use tower_lsp::jsonrpc::{Error, Result};
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use tower_service::Service;

pub struct Backend {
    client: Client,
    lexer_service: Mutex<LexerService>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        let lexer_service = Mutex::new(LexerService::default());
        Self {
            client,
            lexer_service,
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::Info, "initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let response_future;
        let pos = params.text_document_position_params.position;
        let pathbuf = params
            .text_document_position_params
            .text_document
            .uri
            .to_file_path()
            .unwrap();
        let path = pathbuf.to_str().unwrap().to_string();
        let lineno = pos.line as usize + 1;
        let request = LexerRequest::GetTokenAt {
            path,
            lineno,
            column: pos.character as usize,
        };
        if let Ok(mut service) = self.lexer_service.lock() {
            response_future = service.call(request);
        } else {
            return Err(Error::internal_error());
        }
        if let Ok(LexerResponse::TokenResponse(Some(token))) = response_future.await {
            let hover_text = format!("**{}** -> tipo {}", token.value, token.token_type);
            let content = MarkupContent {
                kind: MarkupKind::Markdown,
                value: hover_text,
            };
            Ok(Some(Hover {
                contents: HoverContents::Markup(content),
                range: None,
            }))
        } else {
            info!("Invalid request");
            return Err(Error::invalid_request());
        }
    }
}
