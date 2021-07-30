//#[macro_use] extern crate lalrpop_util;

use std::io::BufRead;

mod parsing;
mod execution;

use parsing::calculator1::ExprParser;

use execution::tree_walker;

fn main() {

    let parser = ExprParser::new();

    let stdin = std::io::stdin();

    for str in stdin.lock().lines() {
        let str = str.unwrap();

        if str=="exit" {break;}

        let res = parser.parse(&str)
            .map_err(|err| {err.to_string()})
            .and_then(tree_walker::visit_expr);

        match res {
            Ok(number) => {
                println!("result: {}", number);
            }
            Err(err_msg) => {
                println!("error: {}", err_msg);
            }
        }
    }
}


