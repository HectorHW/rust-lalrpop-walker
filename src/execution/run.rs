use std::fs;
use std::io::BufRead;
use std::collections::HashMap;
use super::tree_walker::{clear_run, run_keep_state};
use crate::parsing::calculator1::ProgramParser;


pub fn repl() -> () {
    let stdin = std::io::stdin();
    let parser = ProgramParser::new();

    let mut state = HashMap::new();

    for str in stdin.lock().lines() {
        let str = str.unwrap();

        if str=="exit" {break;}
        let _ = parser.parse(&str)
            .map_err(|err| {err.to_string()})
            .and_then(|program| {run_keep_state(program, &mut state)})
            .map_err(|err| {println!("error: {}", err)});
    }
}

pub fn run_file(filename: &str) -> () {

    let file_content = fs::read_to_string(filename).expect("failed to read file");
    let parser = ProgramParser::new();

    let _ = parser.parse(&file_content)
        .map_err(|e| {e.to_string()})
        .and_then(|program| {clear_run(program)})
        .map_err(|err| {println!("error: {}", err)});
}