use crate::token::TokenKind;
use logos::Logos;
use lumen_derive::LumenLanguage;

#[derive(Logos, LumenLanguage, Debug, Clone, Copy, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum JsonToken {
    // Keywords (JSON primitives)
    #[lumen_token(Keyword)]
    #[token("true")]
    #[token("false")]
    #[token("null")]
    Keyword,

    // String literals
    #[lumen_token(String)]
    #[regex(r#""([^"\\]|\\["\\\/bfnrt]|\\u[0-9a-fA-F]{4})*""#)]
    String,

    // Numbers
    #[lumen_token(Number)]
    #[regex(r"-?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][+-]?[0-9]+)?")]
    Number,

    // Punctuation
    #[lumen_token(Punctuation)]
    #[token("{")]
    #[token("}")]
    #[token("[")]
    #[token("]")]
    #[token(",")]
    #[token(":")]
    Punctuation,
}
