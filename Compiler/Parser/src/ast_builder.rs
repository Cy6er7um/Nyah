use logos::Lexer;
use hashbrown::HashMap;
use nyah_ast::*;
use nyah_il::function::Function;

use crate::tokenizer::Token;

/// syntax analyzer
pub struct Parser<'a>{
    stream:Lexer<'a,Token>,
    slot:HashMap< String , u32>,
    functions:Vec<Function>
}

impl<'a> From<Lexer<'a,Token>> for Parser<'a>{
    fn from(t: Lexer<'a,Token>) -> Self {
        Self { stream: t, slot: HashMap::new(),functions:Vec::new()}
    }
}

pub trait ParserTrait<'a>:From<Lexer<'a,Token>>{
    fn parse_expression(&mut self) -> Result<Expression,&'static str>;
    fn parse_ifelse(&mut self) -> Result<IfElse,&'static str>;
    fn parse_for(&mut self) -> Result<For,&'static str>;
    fn parse_while(&mut self) -> Result<While,&'static str>;
    fn parse_statement(&mut self) -> Result<Statement,&'static str>;
    fn parse_fn(&mut self) -> Result<(),&'static str>;
    fn build_ast(self) -> Result<(Vec<Statement>,Vec<Function>),&'static str>;
}

pub fn build_ast<'a,T:'a>(stream: Lexer<'a,Token>) -> Result<(Vec<Statement>,Vec<Function>),&'static str>
    where T:ParserTrait<'a>
{
    ParserTrait::build_ast(T::from(stream))
}
