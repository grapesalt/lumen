use crate::token::TokenKind;
use logos::Logos;
use lumen_derive::LumenLanguage;

#[derive(Logos, LumenLanguage, Debug, Clone, Copy, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum RustToken {
    // Keywords
    #[lumen_token(Keyword)]
    #[regex(r"as|async|await|break|const|continue|crate|dyn|else|enum|extern|false|fn|for|if|impl|in|let|loop|match|mod|move|mut|pub|ref|return|self|Self|static|struct|super|trait|true|type|unsafe|use|where|while")]
    Keyword,

    // Storage modifiers
    #[lumen_token(Keyword)]
    #[regex(r"abstract|become|box|do|final|macro|override|priv|typeof|unsized|virtual|yield")]
    StorageModifier,

    // Types
    #[lumen_token(Type)]
    #[regex(r"i8|i16|i32|i64|i128|isize|u8|u16|u32|u64|u128|usize|f32|f64|bool|char|str")]
    BuiltinType,

    // String literals
    #[lumen_token(String)]
    #[regex(r#""([^"\\]|\\.)*""#)]
    #[regex(r###"r#*"[^"]*"#*"###)]
    String,

    // Character literals
    #[lumen_token(String)]
    #[regex(r"'([^'\\]|\\.)'")]
    Char,

    // Byte string literals
    #[lumen_token(String)]
    #[regex(r#"b"([^"\\]|\\.)*""#)]
    #[regex(r###"br#*"[^"]*"#*"###)]
    ByteString,

    // Numbers
    #[lumen_token(Number)]
    #[regex(r"0b[01_]+")]
    #[regex(r"0o[0-7_]+")]
    #[regex(r"0x[0-9a-fA-F_]+")]
    #[regex(r"[0-9][0-9_]*(\.[0-9_]+)?([eE][+-]?[0-9_]+)?")]
    Number,

    // Comments
    #[lumen_token(Comment)]
    #[regex(r"//([^\n])*")]
    LineComment,

    #[lumen_token(Comment)]
    #[regex(r"/\*([^*]|(\*+[^*/]))*\*+/")]
    BlockComment,

    #[lumen_token(DocComment)]
    #[regex(r"///([^\n])*")]
    #[regex(r"//!([^\n])*")]
    DocComment,

    // Attributes
    #[lumen_token(Attribute)]
    #[regex(r"#!?\[[^\]]*\]")]
    Attribute,

    // Identifiers
    #[lumen_token(Identifier)]
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    // Lifetime
    #[lumen_token(Label)]
    #[regex(r"'[a-zA-Z_][a-zA-Z0-9_]*")]
    Lifetime,

    // Operators
    #[lumen_token(Operator)]
    #[regex(r"[+\-*/%=!<>&|^~?:]+|\.\.=?|\.\.\.|->|=>|@")]
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
    #[token("#")]
    #[token("$")]
    Punctuation,

    // Macros
    #[lumen_token(Macro)]
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*!")]
    Macro,
}
