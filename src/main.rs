//#[macro_use] extern crate lalrpop_util;


mod parsing;
mod execution;

use execution::run::{repl, run_file};
use std::env;

fn main() {
    match  env::args().skip(1).next() {
        None => {
            println!("running REPL");
            repl();
        }
        Some(filename) => {
            println!("running file {}", filename);
            run_file(&filename);
        }
    }
}


