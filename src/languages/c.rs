use crate::token::TokenKind;
use logos::Logos;
use lumen_derive::LumenLanguage;

#[derive(Logos, LumenLanguage, Debug, Clone, Copy, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum CToken {
    // Keywords
    #[lumen_token(Keyword)]
    #[regex(r"auto|break|case|char|const|continue|default|do|double|else|enum|extern|float|for|goto|if|int|long|register|return|short|signed|sizeof|static|struct|switch|typedef|union|unsigned|void|volatile|while")]
    Keyword,

    // C99/C11 keywords
    #[lumen_token(Keyword)]
    #[regex(r"inline|restrict|_Bool|_Complex|_Imaginary|_Alignas|_Alignof|_Atomic|_Static_assert|_Noreturn|_Thread_local|_Generic")]
    ModernKeyword,

    // Preprocessor directives
    #[lumen_token(Attribute)]
    #[regex(r"#[ \t]*(include|define|undef|ifdef|ifndef|if|else|elif|endif|error|pragma|line)[^\n]{0,2000}")]
    Preprocessor,

    // String literals
    #[lumen_token(String)]
    #[regex(r#""([^"\\]|\\[\s\S])*""#)]
    String,

    // Character literals
    #[lumen_token(String)]
    #[regex(r"'([^'\\]|\\[\s\S])'")]
    Char,

    // Numbers
    #[lumen_token(Number)]
    #[regex(r"0[xX][0-9a-fA-F]+[uUlL]*")]
    #[regex(r"0[0-7]+[uUlL]*")]
    #[regex(r"[0-9]+\.[0-9]*([eE][+-]?[0-9]+)?[fFlL]?")]
    #[regex(r"[0-9]+[eE][+-]?[0-9]+[fFlL]?")]
    #[regex(r"\.[0-9]+([eE][+-]?[0-9]+)?[fFlL]?")]
    #[regex(r"[0-9]+[uUlL]*")]
    Number,

    // Comments
    #[lumen_token(Comment)]
    #[regex(r"//[^\n\r]{0,2000}")]
    LineComment,

    #[lumen_token(Comment)]
    #[regex(r"/\*([^*]|(\*+[^*/]))*\*+/")]
    BlockComment,

    // Identifiers
    #[lumen_token(Identifier)]
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    // Operators
    #[lumen_token(Operator)]
    #[token("++")]
    #[token("--")]
    #[token("->")]
    #[token("<<")]
    #[token(">>")]
    #[token("<=")]
    #[token(">=")]
    #[token("==")]
    #[token("!=")]
    #[token("&&")]
    #[token("||")]
    #[token("+=")]
    #[token("-=")]
    #[token("*=")]
    #[token("/=")]
    #[token("%=")]
    #[token("&=")]
    #[token("|=")]
    #[token("^=")]
    #[token("<<=")]
    #[token(">>=")]
    #[token("+")]
    #[token("-")]
    #[token("*")]
    #[token("/")]
    #[token("%")]
    #[token("&")]
    #[token("|")]
    #[token("^")]
    #[token("~")]
    #[token("!")]
    #[token("<")]
    #[token(">")]
    #[token("=")]
    #[token("?")]
    #[token(":")]
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
    Punctuation,
}
