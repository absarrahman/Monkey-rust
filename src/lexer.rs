use crate::token::Token;

#[derive(Debug)]
struct Lexer {
    input: &'static str,
    position: i32,
    read_position: i32,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        todo!();
    }

    pub fn next_token(&self) -> Token {
        todo!();
    }
}

#[cfg(test)]

mod tests {
    use crate::token::{Token, self};

    use super::Lexer;
    #[test]
    fn test_next_token() {
        let input: &str = "=+(){},;";

        let expected_res: Vec<Token> = vec![
            Token { token_type: token::ASSIGN, literal: "=".to_string(), },
            Token { token_type: token::PLUS, literal: "+".to_string(), },
            Token { token_type: token::LPAREN, literal: "(".to_string(), },
            Token { token_type: token::RPAREN, literal: ")".to_string(), },
            Token { token_type: token::LBRACE, literal: "{".to_string(), },
            Token { token_type: token::RBRACE, literal: "}".to_string(), },
            Token { token_type: token::COMMA, literal: ",".to_string(), },
            Token { token_type: token::SEMICOLON, literal: ":".to_string(), },
            Token { token_type: token::EOF, literal: "".to_string(), },
        ];

        let lexer = Lexer::new(input);

        for (index, expected_token) in expected_res.into_iter().enumerate() {
            let token = lexer.next_token(); 
            assert_eq!(expected_token.token_type, token.token_type,"FAILED AT test[{index}] - expected {}, got {}",
            expected_token.token_type, token.token_type);
            assert_eq!(expected_token.literal, token.literal,"FAILED AT test[{index}] - expected {}, got {}",
            expected_token.literal, token.literal);
        }


    }
}
