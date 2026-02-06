use crate::token::TokenKind;
use logos::Logos;
use lumen_derive::LumenLanguage;

#[derive(Logos, LumenLanguage, Debug, Clone, Copy, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum LuaToken {
    // Keywords
    #[lumen_token(Keyword)]
    #[regex(r"and|break|do|else|elseif|end|false|for|function|goto|if|in|local|nil|not|or|repeat|return|then|true|until|while")]
    Keyword,

    // String literals (single, double, and long brackets)
    #[lumen_token(String)]
    #[regex(r#"'([^'\\]|\\[\s\S])*'"#)]
    #[regex(r#""([^"\\]|\\[\s\S])*""#)]
    SingleString,

    #[lumen_token(String)]
    #[regex(r"\[\[[\s\S]{0,10000}\]\]")]
    #[regex(r"\[=\[[\s\S]{0,10000}\]=\]")]
    #[regex(r"\[==\[[\s\S]{0,10000}\]==\]")]
    #[regex(r"\[===\[[\s\S]{0,10000}\]===\]")]
    LongString,

    // Numbers (hex, decimal, scientific notation)
    #[lumen_token(Number)]
    #[regex(r"0[xX][0-9a-fA-F]+(\.[0-9a-fA-F]+)?([pP][+-]?[0-9]+)?")]
    #[regex(r"[0-9]+\.[0-9]*([eE][+-]?[0-9]+)?")]
    #[regex(r"[0-9]+[eE][+-]?[0-9]+")]
    #[regex(r"\.[0-9]+([eE][+-]?[0-9]+)?")]
    #[regex(r"[0-9]+")]
    Number,

    // Comments
    #[lumen_token(Comment)]
    #[regex(r"--[^\n\r\[]{0,2000}")]
    LineComment,

    #[lumen_token(Comment)]
    #[regex(r"--\[\[[\s\S]{0,10000}\]\]")]
    #[regex(r"--\[=\[[\s\S]{0,10000}\]=\]")]
    #[regex(r"--\[==\[[\s\S]{0,10000}\]==\]")]
    BlockComment,

    // Identifiers
    #[lumen_token(Identifier)]
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    // Operators
    #[lumen_token(Operator)]
    #[token("==")]
    #[token("~=")]
    #[token("<=")]
    #[token(">=")]
    #[token("<<")]
    #[token(">>")]
    #[token("//")]
    #[token("..")]
    #[token("...")]
    #[token("+")]
    #[token("-")]
    #[token("*")]
    #[token("/")]
    #[token("%")]
    #[token("^")]
    #[token("#")]
    #[token("&")]
    #[token("|")]
    #[token("~")]
    #[token("<")]
    #[token(">")]
    #[token("=")]
    Operator,

    // Punctuation
    #[lumen_token(Punctuation)]
    #[token("(")]
    #[token(")")]
    #[token("[")]
    #[token("]")]
    #[token("{")]
    #[token("}")]
    #[token(",")]
    #[token(";")]
    #[token(".")]
    #[token(":")]
    #[token("::")]
    Punctuation,
}
