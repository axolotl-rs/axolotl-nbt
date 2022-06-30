use logos::{Lexer, Logos};

fn byte(lex: &mut Lexer<Token>) -> Option<i8> {
    let slice = lex.slice();
    let n: i8 = slice[..slice.len() - 1].parse().ok()?;
    Some(n)
}

fn short(lex: &mut Lexer<Token>) -> Option<i16> {
    let slice = lex.slice();
    let n: i16 = slice[..slice.len() - 1].parse().ok()?;
    Some(n)
}

fn int(lex: &mut Lexer<Token>) -> Option<i32> {
    let slice = lex.slice();
    let n: i32 = slice[..slice.len()].parse().ok()?;
    Some(n)
}

fn long(lex: &mut Lexer<Token>) -> Option<i64> {
    let slice = lex.slice();
    let n: i64 = slice[..slice.len() - 1].parse().ok()?;
    Some(n)
}

fn tag_name(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let n = slice[..slice.len() - 1].to_string();
    Some(n)
}

fn float(lex: &mut Lexer<Token>) -> Option<f32> {
    let slice = lex.slice();
    let n: f32 = slice[..slice.len() - 1].parse().ok()?;
    Some(n)
}

fn to_string(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let n = slice[1..slice.len() - 1].trim().to_string();
    Some(n)
}

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    #[token("[B;")]
    ByteArray,
    #[token("[I;")]
    IntArray,
    #[token("[L;")]
    LongArray,
    #[token("[")]
    StartList,
    #[token("]")]
    EndListOrArray,
    #[token("{")]
    StartCompound,
    #[token("}")]
    EndCompound,
    #[regex(r"\d+B", byte)]
    #[regex(r"\d+b", byte)]
    Byte(i8),
    #[regex(r"\d+S", short)]
    #[regex(r"\d+s", short)]
    Short(i16),
    #[regex(r"\d+", int)]
    Int(i32),
    #[regex(r"\d+L", long)]
    #[regex(r"\d+l", long)]
    Long(i64),
    #[regex(r"\d+F", float)]
    #[regex(r"\d+f", float)]
    Float(f32),
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[regex(r"\w+:", tag_name)]
    TagName(String),
    #[regex(r#""(?:[^"]|\\")*""#, to_string)]
    #[regex(r#"'(?:[^']|\\')*'"#, to_string)]
    String(String),
    #[token(",")]
    ArrayListCompoundSeparator,
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}
