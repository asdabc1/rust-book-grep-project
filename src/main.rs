extern crate rust_book_grep_project;

use std::env;
use std::process;

use rust_book_grep_project::run;

fn main() {
    let args: Vec<_> = env::args().collect();

    if let Err(e) = run(&args) {
        eprintln!("Error running the application: {}", e.to_string());
        process::exit(1);
    }
}

