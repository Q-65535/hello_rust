 #![allow(warnings)]
use std::{io::{self, Write}, process::exit, env};
use colored::*;
use crate::lex::*;
use crate::parse::*;
use crate::TokenKind::*;
use crate::CompareToken::*;
use crate::ExprType::*;
use crate::codegen::*;
pub mod parse;
pub mod lex;
pub mod codegen;

// static global_src: &str = "-10-1234*(2-3423)/11112;1+2;";
static global_src: &str = "c = 1; b = 10;";



fn main() {
    let args: Vec<String> = env::args().collect();
    let input: &String;
    if args.len() > 1 {
        input = &args[1];
    } else {
        println!("No arguments provided.");
        return;
    }

    // println!("compiling source code: '{}'", input);
    // lex
    let mut lexer = Lexer::new(input);
    let mut tokens: Vec<Token> = Vec::new();
    let tokens = lexer.lex();
    let src = input.clone();
    // parse
    let mut parser = Parser::new(&src, tokens, 0);
    let res = parser.parse();
    // codegen
    let mut gen = Generator::new();
    match res {
        Ok(program) => gen.gen_code(&program),
        Err(err_msg) => println!("{}", err_msg),
    }
}




