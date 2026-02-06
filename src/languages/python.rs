use crate::token::TokenKind;
use logos::Logos;
use lumen_derive::LumenLanguage;

#[derive(Logos, LumenLanguage, Debug, Clone, Copy, PartialEq)]
#[logos(skip r"[ \t\r\f]+")]
pub enum PythonToken {
    // Keywords
    #[lumen_token(Keyword)]
    #[regex(r"False|None|True|and|as|assert|async|await|break|case|class|continue|def|del|elif|else|except|finally|for|from|global|if|import|in|is|lambda|match|nonlocal|not|or|pass|raise|return|try|while|with|yield")]
    Keyword,

    // Built-in functions and types
    #[lumen_token(Function)]
    #[regex(r"abs|aiter|all|any|anext|ascii|bin|bool|breakpoint|bytearray|bytes|callable|chr|classmethod|compile|complex|delattr|dict|dir|divmod|enumerate|eval|exec|filter|float|format|frozenset|getattr|globals|hasattr|hash|help|hex|id|input|int|isinstance|issubclass|iter|len|list|locals|map|max|memoryview|min|next|object|oct|open|ord|pow|print|property|range|repr|reversed|round|set|setattr|slice|sorted|staticmethod|str|sum|super|tuple|type|vars|zip|__import__")]
    BuiltinFunction,

    // Magic methods/attributes
    #[lumen_token(Function)]
    #[regex(r"__[a-zA-Z_][a-zA-Z0-9_]*__")]
    MagicMethod,

    // String literals
    #[lumen_token(String)]
    #[regex(r###"[fFrRbBuU]{0,2}"""([^\\]|\\[\s\S])*""""###)]
    #[regex(r"[fFrRbBuU]{0,2}'''([^\\\\]|\\\\[\\s\\S])*'''")]
    #[regex(r#"[fFrRbBuU]{0,2}"([^"\\\n]|\\[\s\S])*""#)]
    #[regex(r"[fFrRbBuU]{0,2}'([^'\\\n]|\\[\s\S])*'")]
    String,

    // Numbers
    #[lumen_token(Number)]
    #[regex(r"0[bB][01_]+[lLjJ]?")]
    BinaryNumber,

    #[lumen_token(Number)]
    #[regex(r"0[oO][0-7_]+[lLjJ]?")]
    OctalNumber,

    #[lumen_token(Number)]
    #[regex(r"0[xX][0-9a-fA-F_]+[lLjJ]?")]
    HexNumber,

    #[lumen_token(Number)]
    #[regex(r"[0-9][0-9_]*[lLjJ]")]
    IntegerWithSuffix,

    #[lumen_token(Number)]
    #[regex(r"[0-9][0-9_]*\.[0-9_]*([eE][+-]?[0-9_]+)?[jJ]?")]
    #[regex(r"[0-9][0-9_]*[eE][+-]?[0-9_]+[jJ]?")]
    #[regex(r"\.[0-9_]+([eE][+-]?[0-9_]+)?[jJ]?")]
    FloatNumber,

    #[lumen_token(Number)]
    #[regex(r"[0-9][0-9_]*")]
    Integer,

    // Comments
    #[lumen_token(Comment)]
    #[regex(r"#[^\n\r]{0,2000}")]
    Comment,

    // Decorators
    #[lumen_token(Attribute)]
    #[regex(r"@[a-zA-Z_][a-zA-Z0-9_]*(\.[a-zA-Z_][a-zA-Z0-9_]*)*")]
    Decorator,

    // Identifiers (variables, functions, classes)
    #[lumen_token(Identifier)]
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    // Operators
    #[lumen_token(Operator)]
    #[token("+=")]
    #[token("-=")]
    #[token("*=")]
    #[token("/=")]
    #[token("//=")]
    #[token("%=")]
    #[token("**=")]
    #[token("&=")]
    #[token("|=")]
    #[token("^=")]
    #[token(">>=")]
    #[token("<<=")]
    #[token("@=")]
    #[token(":=")]
    CompoundOperator,

    #[lumen_token(Operator)]
    #[token("==")]
    #[token("!=")]
    #[token("<=")]
    #[token(">=")]
    #[token("<")]
    #[token(">")]
    ComparisonOperator,

    #[lumen_token(Operator)]
    #[token("**")]
    #[token("//")]
    #[token("<<")]
    #[token(">>")]
    DoubleOperator,

    #[lumen_token(Operator)]
    #[token("+")]
    #[token("-")]
    #[token("*")]
    #[token("/")]
    #[token("%")]
    #[token("@")]
    #[token("&")]
    #[token("|")]
    #[token("^")]
    #[token("~")]
    SingleOperator,

    // Punctuation
    #[lumen_token(Punctuation)]
    #[token("(")]
    #[token(")")]
    #[token("[")]
    #[token("]")]
    #[token("{")]
    #[token("}")]
    #[token(",")]
    #[token(":")]
    #[token(";")]
    #[token(".")]
    #[token("=")]
    #[token("->")]
    #[token("...")]
    Punctuation,
}
