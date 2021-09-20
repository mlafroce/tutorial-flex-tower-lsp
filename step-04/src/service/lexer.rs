use crate::service::lexer::LexerResponse::TokenResponse;
use crate::token_map::{Token, TokenMap};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower_service::Service;

#[derive(Default)]
pub struct Lexer {
    docs_token_map: HashMap<String, TokenMap>,
}

#[derive(Debug)]
pub enum LexerResponse {
    Parsed,
    TokenResponse(Option<Token>),
}

#[derive(Debug)]
pub enum LexerError {
    FileNotFound(String),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum LexerRequest {
    ParseFile {
        path: String,
        force: bool,
    },
    GetTokenAt {
        path: String,
        lineno: usize,
        column: usize,
    },
}

impl Service<LexerRequest> for Lexer {
    type Response = LexerResponse;
    type Error = LexerError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: LexerRequest) -> Self::Future {
        let response = match request {
            LexerRequest::ParseFile { path, force } => self.handle_parse_request(&path, force),
            LexerRequest::GetTokenAt {
                path,
                lineno,
                column,
            } => self.handle_get_token_request(&path, lineno, column),
        };
        let fut = async { response };
        Box::pin(fut)
    }
}

impl Lexer {
    fn handle_parse_request(
        &mut self,
        path: &str,
        force: bool,
    ) -> Result<LexerResponse, LexerError> {
        if !self.docs_token_map.contains_key(path) || force {
            let mut token_map = TokenMap::default();
            if token_map.parse_tokens(path).is_err() {
                log::error!("Couldn't parse {}", path);
                return Err(LexerError::FileNotFound(path.to_string()));
            }
            self.docs_token_map.insert(path.to_owned(), token_map);
        }
        Ok(LexerResponse::Parsed)
    }

    fn handle_get_token_request(
        &mut self,
        path: &str,
        lineno: usize,
        column: usize,
    ) -> Result<LexerResponse, LexerError> {
        self.handle_parse_request(path, false)?;
        let token;
        let token_map = self.docs_token_map.get(path).unwrap();
        token = token_map.get_token_at(lineno, column);
        Ok(TokenResponse(token))
    }
}
