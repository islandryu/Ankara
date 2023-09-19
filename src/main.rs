mod ast;
mod builtin;
mod interpreter;
mod lexer;
mod parser;
mod precedence;
mod read_file;
mod token;
use std::{cell::RefCell, rc::Rc};

use interpreter::evaluator::{self, EvalOption, Evaluator};
use lexer::Peekable;
use logos::{source, Logos};
use parser::parse;
use token::Token;
extern crate clap;
use builtin::get_builtin_environment::get_builtin_environment;
use clap::{App, Arg};
use read_file::read_file;

fn main() {
    let matches = App::new("ankara")
        .version("1.0")
        .author("Your Name")
        .about("Description about your application")
        .arg(
            Arg::with_name("file")
                .help("The input file to use")
                .required(true)
                .index(1),
        ) // 1つ目のフリーアーギュメントとして受け取る
        .get_matches();

    let file_name = matches.value_of("file").unwrap();

    let source_code = match read_file(file_name) {
        Ok(source_code) => source_code,
        Err(error) => {
            println!("{:?}", error);
            return;
        }
    };

    let mut lexer = Peekable::new(&source_code);
    let program = match parse(&mut lexer) {
        Ok(program) => program,
        Err(error) => {
            println!("{:?}", error);
            return;
        }
    };
    let mut env = get_builtin_environment();
    match program.eval(Rc::new(RefCell::new(env)), &mut EvalOption::new()) {
        Ok(obj) => obj,
        Err(error) => {
            println!("{:?}", error);
            return;
        }
    };
}
