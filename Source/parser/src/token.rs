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

    #[regex(r",")]
    Comma,

    #[regex(r":")]
    Colon,

    #[regex(r";")]
    Semicolon,

    #[regex(r"@")]
    At,

    #[regex(r"#")]
    Hashtag,

    #[regex(r"=")]
    Equal,

    #[regex(r"\+")]
    Add,

    #[regex(r"\-")]
    Sub,

    #[regex(r"\*", priority = 2)]
    Mul,

    #[regex(r"/")]
    Div,

    #[regex(r"\*\*", priority = 1)]
    Exp,

    #[regex(r"%")]
    Mod,

    #[regex(r"|")]
    VerticalBar,

    #[regex(r"\{")]
    LeftCurlyBrace,

    #[regex(r"\}")]
    RightCurlyBrace,

    #[regex(r"\[")]
    LeftSquareBracket,

    #[regex(r"\]")]
    RightSquareBracket,

    #[regex(r"\(")]
    LeftParenthesis,

    #[regex(r"\)")]
    RightParenthesis,

    #[regex(r"[ \t\n\f]+")]
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
        let mut lex = Token::lexer(
            r#"
#[exampleMacro("ExampleString")]
@exampleDecorator("ExampleString")
func exampleFunction(
    exampleArg: exampleType,
    ...exampleArgs: [exampleType2],
    ......exampleKwargs: {string: exampleType3}
): exampleType4 {
    let exampleVariable = 123_456;
    examplePackage.exampleModule.exampleType4
        .exampleMethod(exampleVariable, "This is a String.")
}
"#
        );

        while let Some(token) = lex.next() {
            println!("{:?}, {:?}, {:?}", token, lex.span(), lex.slice());
            assert_ne!(token, Token::Error);
        }
    }
}
