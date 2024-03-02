use std::io;

mod token;
mod lexer;
mod repl;

fn main() {
    println!("Monkey v0.1");
    repl::start(io::stdin(), io::stdout());
}
