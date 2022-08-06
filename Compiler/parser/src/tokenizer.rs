use logos::Logos;

#[derive(Logos, Debug, PartialEq, Eq)]
pub enum Token {
    #[regex(r"[_\p{Han}\p{Latin}][_\p{Han}\p{Latin}\p{Number}]*", priority = 1)]
    Identifier,

    #[regex(r"\p{Number}[_\p{Number}]*", priority = 2)]
    Integer,

    #[regex(r#"'((\\')|[^'])'"#)]
    Char,

    #[regex(r#""((\\")|[^"])*""#)]
    String,

    #[regex(r"\.", priority = 3)]
    Dot,

    #[regex(r"\.\.\.", priority = 2)]
    Etc3,

    #[regex(r"\.\.\.\.\.\.", priority = 1)]
    Etc6,

    #[token("&")]
    Reference,

    #[token(",")]
    Comma,

    #[token(":")]
    Colon,

    #[token(";")]
    Semicolon,

    #[token("@")]
    At,

    #[token("#")]
    Hashtag,

    #[token("?")]
    QuestionMark,

    #[token("=")]
    Equal,

    #[token("+")]
    Add,

    #[token("-")]
    Sub,

    #[regex(r"\*", priority = 2)]
    Mul,

    #[token("/")]
    Div,

    #[regex(r"\*\*", priority = 1)]
    Exp,

    #[token(r"%")]
    Mod,

    #[token(r"|")]
    VerticalBar,

    #[token("{")]
    LeftCurlyBrace,

    #[token("}")]
    RightCurlyBrace,

    #[token("[")]
    LeftSquareBracket,

    #[token("]")]
    RightSquareBracket,

    #[token("(")]
    LeftParenthesis,

    #[token(")")]
    RightParenthesis,

    #[regex(r"[ \t\n\f\r]+")]
    Whitespace,

    #[error]
    Error,
}

#[cfg(test)]
mod test {
    use logos::Logos;

    use super::Token;

    #[test]
    fn test_lexer() {
        let mut lex = Token::lexer(include_str!("../../../Example/HttpServer.nyah"));

        while let Some(token) = lex.next() {
            println!("{:?}, {:?}, {:?}", token, lex.span(), lex.slice());
            assert_ne!(token, Token::Error);
        }
    }
}
