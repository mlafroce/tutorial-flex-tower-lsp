use log::info;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use crate::lexer::Lexer;

pub struct Backend {
    client: Client,
    lexer: Lexer
}

impl Backend {
    pub fn new(client: Client) -> Self{
        let lexer = Lexer::default();
        Self { client, lexer }
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
        info!("Hovering over {:?}", params);
        let hover_text= "#HoverDemo\n---\nEsto es un hover formateado con Python-Markdown\n\nPermite:\n\n* Listas\n* *Cursiva* y **negrita**\n* `Código`.".to_owned();
        let content = MarkupContent {
            kind: MarkupKind::Markdown,
            value: hover_text,
        };
        Ok(Some(Hover {
            contents: HoverContents::Markup(content),
            range: None,
        }))
    }
}
