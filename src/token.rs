use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Keyword,
    Type,
    Function,
    Variable,
    Identifier,
    Constant,
    String,
    Char,
    Number,
    Boolean,
    Comment,
    DocComment,
    Operator,
    Punctuation,
    Attribute,
    Macro,
    Label,
    Namespace,
    Escape,
    Error,
    Text,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Range<usize>,
}

impl Token {
    pub fn new(kind: TokenKind, span: Range<usize>) -> Self {
        Self { kind, span }
    }

    pub fn value<'a>(&self, source: &'a str) -> &'a str {
        &source[self.span.clone()]
    }
}

pub trait LanguageToken: Sized {
    fn to_token(&self) -> Token;
}
