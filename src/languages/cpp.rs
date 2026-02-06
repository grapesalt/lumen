use crate::token::TokenKind;
use logos::Logos;
use lumen_derive::LumenLanguage;

#[derive(Logos, LumenLanguage, Debug, Clone, Copy, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum CppToken {
    // Keywords
    #[lumen_token(Keyword)]
    #[regex(r"alignas|alignof|and|and_eq|asm|auto|bitand|bitor|bool|break|case|catch|char|char8_t|char16_t|char32_t|class|compl|concept|const|consteval|constexpr|constinit|const_cast|continue|co_await|co_return|co_yield|decltype|default|delete|do|double|dynamic_cast|else|enum|explicit|export|extern|false|float|for|friend|goto|if|inline|int|long|mutable|namespace|new|noexcept|not|not_eq|nullptr|operator|or|or_eq|private|protected|public|register|reinterpret_cast|requires|return|short|signed|sizeof|static|static_assert|static_cast|struct|switch|template|this|thread_local|throw|true|try|typedef|typeid|typename|union|unsigned|using|virtual|void|volatile|wchar_t|while|xor|xor_eq")]
    Keyword,

    // C++20/23 contextual keywords
    #[lumen_token(Keyword)]
    #[regex(r"final|override|import|module|atomic_cancel|atomic_commit|atomic_noexcept|synchronized|reflexpr")]
    ContextualKeyword,

    // Attributes
    #[lumen_token(Attribute)]
    #[regex(r"\[\[[ \t]*[a-zA-Z_:][a-zA-Z0-9_:]*[^\]]{0,200}\]\]")]
    Attribute,

    // Preprocessor directives
    #[lumen_token(Attribute)]
    #[regex(r"#[ \t]*(include|define|undef|ifdef|ifndef|if|else|elif|endif|error|pragma|line|warning|import)[^\n]{0,2000}")]
    Preprocessor,

    // String literals
    #[lumen_token(String)]
    #[regex(r#"(u8|u|U|L)?"([^"\\]|\\[\s\S])*"(_[a-zA-Z_][a-zA-Z0-9_]*)?"#)]
    String,

    // Raw string literals
    #[lumen_token(String)]
    #[regex(r###"(u8|u|U|L)?R"[^(]{0,16}\([\s\S]{0,10000}\)[^"]{0,16}""###)]
    RawString,

    // Character literals
    #[lumen_token(String)]
    #[regex(r"(u8|u|U|L)?'([^'\\]|\\[\s\S])'(_[a-zA-Z_][a-zA-Z0-9_]*)?")]
    Char,

    // Numbers
    #[lumen_token(Number)]
    #[regex(r"0[xX][0-9a-fA-F']+(\.[0-9a-fA-F']+)?([pP][+-]?[0-9']+)?[fFlLuUzZ]*(_[a-zA-Z_][a-zA-Z0-9_]*)?")]
    #[regex(r"0[bB][01']+[uUlLzZ]*(_[a-zA-Z_][a-zA-Z0-9_]*)?")]
    #[regex(r"0[0-7']+[uUlLzZ]*(_[a-zA-Z_][a-zA-Z0-9_]*)?")]
    #[regex(r"[0-9][0-9']*\.[0-9']*([eE][+-]?[0-9']+)?[fFlLzZ]*(_[a-zA-Z_][a-zA-Z0-9_]*)?")]
    #[regex(r"[0-9][0-9']*[eE][+-]?[0-9']+[fFlLzZ]*(_[a-zA-Z_][a-zA-Z0-9_]*)?")]
    #[regex(r"\.[0-9']+([eE][+-]?[0-9']+)?[fFlLzZ]*(_[a-zA-Z_][a-zA-Z0-9_]*)?")]
    #[regex(r"[0-9][0-9']*[uUlLzZ]*(_[a-zA-Z_][a-zA-Z0-9_]*)?")]
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
    #[token("::")]
    #[token(".*")]
    #[token("->*")]
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
    #[token("<=>")]
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
    #[token("...")]
    #[token("#")]
    #[token("##")]
    Punctuation,
}
