use logos::{Lexer, Logos};

#[derive(Debug, Clone, Logos, PartialEq)]
pub enum Token {
    #[token("let")]
    Let,
    #[token("fn")]
    Fn,
    #[token("if")]
    If,
    #[token("elif")]
    Elif,
    #[token("else")]
    Else,
    #[token("loop")]
    Loop,
    #[token("return")]
    Return,
    #[token("continue")]
    Continue,
    #[token("break")]
    Break,

    #[regex(r"[a-zA-Z_?]+", to_string)]
    Identifier(String),

    #[token("true")]
    True,
    #[token("false")]
    False,

    #[regex(r"([0-9]+[.])?[0-9]+", to_float)]
    Number(f64),
    #[regex(r##""(?:[^"\\]|\\.)*""##, to_string)]
    String(String),

    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,

    #[token("=")]
    Assign,
    #[token("==")]
    Equals,
    #[token("!=")]
    NotEquals,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("<=")]
    LessThanOrEquals,
    #[token(">=")]
    GreaterThanOrEquals,

    #[token("&&")]
    And,
    #[token("||")]
    Or,

    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("!")]
    Bang,
    #[token(".")]
    Dot,

    Eof,

    #[error]
    #[regex(r"--[^\n]*", logos::skip)]
    #[regex(r"[ ;\t\n\f]+", logos::skip)]
    Error,
}

impl Into<String> for Token {
    fn into(self) -> String {
        match self {
            Token::Identifier(s) => s,
            _ => unreachable!(),
        }
    }
}

fn to_string(lex: &mut Lexer<Token>) -> Option<String> {
    let mut string: String = lex.slice().to_string();

    if string.starts_with("$") {
        string.remove(0);
    }

    if string.starts_with("\"") {
        string.remove(0);
    }

    if string.ends_with('"') {
        string.remove(string.len() - 1);
    }

    Some(string)
}

fn to_float(lex: &mut Lexer<Token>) -> Option<f64> {
    Some(lex.slice().parse().ok()?)
}
