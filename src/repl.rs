use std::io::{Stdin, Stdout, Write};

use crate::{lexer::Lexer, token};

pub fn start(stdin: Stdin, mut out: Stdout) -> ! {
    loop {
        write!(out, ">>").expect("Expected to start");
        out.flush().expect("Failed to flush Stdout");
        let mut line_text = "".to_string();
        match stdin.read_line(&mut line_text) {
            Ok(_) => {
                let mut lexer = Lexer::new(&line_text);
                loop {
                    let token_value = lexer.next_token();
                    if token_value.token_type == token::EOF {
                        break;
                    }
                    writeln!(out, "TOKEN VALUE {:?}", token_value).expect("Failed to write token value");
                }
            }
            Err(e) => write!(out, "Error occurred {}", e).expect("Failed to write error"),
        }
    }
}
