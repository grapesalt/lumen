use std::borrow::Cow;

pub mod languages;
pub mod token;

use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Tokens<'a> {
    tokens: Box<[Token]>,
    source: Cow<'a, str>,
}

impl<'a> Tokens<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(tokens: Vec<Token>, source: T) -> Self {
        Self {
            tokens: tokens.into_boxed_slice(),
            source: source.into(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Token> {
        self.tokens.iter()
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}
