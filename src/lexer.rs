#[doc(inline)]
use crate::token;
use crate::token::Token;
use crate::token::TokenType;

#[derive(Debug)]
/// Responsible for recognizing keywords
struct Lexer {
    /// input given to the lexer
    input: Vec<char>,
    /// points to the current position
    position: usize,
    /// points to the next position
    read_position: usize,
    /// points to the character
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: Default::default(),
        };
        lexer.read_char();
        lexer
    }

    /// reads character
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            // asci code for null
            self.ch = '\0';
        } else {
            // store value of next position
            self.ch = self.input[self.read_position];
        }
        // updating current position
        self.position = self.read_position;
        // increment next position
        self.read_position += 1;
    }

    /// Gets the next token
    pub fn next_token(&mut self) -> Token {
        // skip whitespace
        while self.ch.is_whitespace() {
            self.read_char();
        }

        let token = match self.ch {
            '=' => Lexer::new_token(token::ASSIGN, self.ch),
            '+' => Lexer::new_token(token::PLUS, self.ch),
            '-' => Lexer::new_token(token::MINUS, self.ch),
            '{' => Lexer::new_token(token::LBRACE, self.ch),
            '}' => Lexer::new_token(token::RBRACE, self.ch),
            ',' => Lexer::new_token(token::COMMA, self.ch),
            ';' => Lexer::new_token(token::SEMICOLON, self.ch),
            '(' => Lexer::new_token(token::LPAREN, self.ch),
            ')' => Lexer::new_token(token::RPAREN, self.ch),
            '!' => Lexer::new_token(token::BANG, self.ch),
            '*' => Lexer::new_token(token::ASTERISK, self.ch),
            '/' => Lexer::new_token(token::SLASH, self.ch),
            '<' => Lexer::new_token(token::LT, self.ch),
            '>' => Lexer::new_token(token::GT, self.ch),
            '\0' => Token {
                token_type: token::EOF,
                literal: "".to_string(),
            },
            _ => {
                if Lexer::is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let token_type = token::lookup_ident(&literal);

                    return Token {
                        token_type,
                        literal,
                    };
                } else if Lexer::is_digit(self.ch) {
                    let literal = self.read_number();
                    let token_type = token::INT;

                    return Token {
                        token_type,
                        literal,
                    };
                }
                return Lexer::new_token(token::ILLEGAL, self.ch);
            }
        };
        self.read_char();
        token
    }

    /// Generates token using `token_type` and `literal`
    ///
    /// # Arguments
    ///
    /// * `token_type` - Type of the token
    /// * `literal` - character value
    fn new_token(token_type: TokenType, literal: char) -> Token {
        Token {
            token_type,
            literal: literal.to_string(),
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while Lexer::is_letter(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].iter().collect()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while Lexer::is_digit(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].iter().collect()
    }

    fn is_letter(literal: char) -> bool {
        literal.is_alphabetic() || '_'.eq(&literal)
    }

    fn is_digit(literal: char) -> bool {
        literal.is_ascii_digit()
    }
}

#[cfg(test)]

mod tests {
    use crate::token::{self, Token};

    use super::Lexer;
    #[test]
    fn test_next_token() {
        let input: &str = "=+(){},;";

        let expected_res: Vec<Token> = vec![
            Token {
                token_type: token::ASSIGN,
                literal: '='.to_string(),
            },
            Token {
                token_type: token::PLUS,
                literal: '+'.to_string(),
            },
            Token {
                token_type: token::LPAREN,
                literal: '('.to_string(),
            },
            Token {
                token_type: token::RPAREN,
                literal: ')'.to_string(),
            },
            Token {
                token_type: token::LBRACE,
                literal: '{'.to_string(),
            },
            Token {
                token_type: token::RBRACE,
                literal: '}'.to_string(),
            },
            Token {
                token_type: token::COMMA,
                literal: ','.to_string(),
            },
            Token {
                token_type: token::SEMICOLON,
                literal: ';'.to_string(),
            },
            Token {
                token_type: token::EOF,
                literal: "".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);

        for (index, expected_token) in expected_res.into_iter().enumerate() {
            let token = lexer.next_token();
            assert_eq!(
                expected_token.token_type, token.token_type,
                "FAILED AT test[{index}] - expected {}, got {}",
                expected_token.token_type, token.token_type
            );
            assert_eq!(
                expected_token.literal, token.literal,
                "FAILED AT test[{index}] - expected {}, got {}",
                expected_token.literal, token.literal
            );
        }
    }

    #[test]
    fn test_next_token_two() {
        let input: &str = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x,y) {
                x+y;
            }
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }
            "#;

        let expected_res: Vec<Token> = vec![
            Token {
                token_type: token::LET,
                literal: "let".to_string(),
            },
            Token {
                token_type: token::IDENT,
                literal: "five".to_string(),
            },
            Token {
                token_type: token::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                token_type: token::INT,
                literal: "5".to_string(),
            },
            Token {
                token_type: token::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: token::LET,
                literal: "let".to_string(),
            },
            Token {
                token_type: token::IDENT,
                literal: "ten".to_string(),
            },
            Token {
                token_type: token::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                token_type: token::INT,
                literal: "10".to_string(),
            },
            Token {
                token_type: token::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: token::LET,
                literal: "let".to_string(),
            },
            Token {
                token_type: token::IDENT,
                literal: "add".to_string(),
            },
            Token {
                token_type: token::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                token_type: token::FUNCTION,
                literal: "fn".to_string(),
            },
            Token {
                token_type: token::LPAREN,
                literal: "(".to_string(),
            },
            Token {
                token_type: token::IDENT,
                literal: "x".to_string(),
            },
            Token {
                token_type: token::COMMA,
                literal: ",".to_string(),
            },
            Token {
                token_type: token::IDENT,
                literal: "y".to_string(),
            },
            Token {
                token_type: token::RPAREN,
                literal: ")".to_string(),
            },
            Token {
                token_type: token::LBRACE,
                literal: "{".to_string(),
            },
            Token {
                token_type: token::IDENT,
                literal: "x".to_string(),
            },
            Token {
                token_type: token::PLUS,
                literal: "+".to_string(),
            },
            Token {
                token_type: token::IDENT,
                literal: "y".to_string(),
            },
            Token {
                token_type: token::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: token::RBRACE,
                literal: "}".to_string(),
            },
            Token {
                token_type: token::LET,
                literal: "let".to_string(),
            },
            Token {
                token_type: token::IDENT,
                literal: "result".to_string(),
            },
            Token {
                token_type: token::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                token_type: token::IDENT,
                literal: "add".to_string(),
            },
            Token {
                token_type: token::LPAREN,
                literal: "(".to_string(),
            },
            Token {
                token_type: token::IDENT,
                literal: "five".to_string(),
            },
            Token {
                token_type: token::COMMA,
                literal: ",".to_string(),
            },
            Token {
                token_type: token::IDENT,
                literal: "ten".to_string(),
            },
            Token {
                token_type: token::RPAREN,
                literal: ")".to_string(),
            },
            Token {
                token_type: token::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: token::BANG,
                literal: "!".to_string(),
            },
            Token {
                token_type: token::MINUS,
                literal: "-".to_string(),
            },
            Token {
                token_type: token::SLASH,
                literal: "/".to_string(),
            },
            Token {
                token_type: token::ASTERISK,
                literal: "*".to_string(),
            },
            Token {
                token_type: token::INT,
                literal: "5".to_string(),
            },
            Token {
                token_type: token::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: token::INT,
                literal: "5".to_string(),
            },
            Token {
                token_type: token::LT,
                literal: "<".to_string(),
            },
            Token {
                token_type: token::INT,
                literal: "10".to_string(),
            },
            Token {
                token_type: token::GT,
                literal: ">".to_string(),
            },
            Token {
                token_type: token::INT,
                literal: "5".to_string(),
            },
            Token {
                token_type: token::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: token::IF,
                literal: "if".to_string(),
            },
            Token {
                token_type: token::LPAREN,
                literal: "(".to_string(),
            },
            Token {
                token_type: token::INT,
                literal: "5".to_string(),
            },
            Token {
                token_type: token::LT,
                literal: "<".to_string(),
            },
            Token {
                token_type: token::INT,
                literal: "10".to_string(),
            },
            Token {
                token_type: token::RPAREN,
                literal: ")".to_string(),
            },
            Token {
                token_type: token::LBRACE,
                literal: "{".to_string(),
            },
            Token {
                token_type: token::RETURN,
                literal: "return".to_string(),
            },
            Token {
                token_type: token::TRUE,
                literal: "true".to_string(),
            },
            Token {
                token_type: token::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: token::RBRACE,
                literal: "}".to_string(),
            },
            Token {
                token_type: token::ELSE,
                literal: "else".to_string(),
            },
            Token {
                token_type: token::LBRACE,
                literal: "{".to_string(),
            },
            Token {
                token_type: token::RETURN,
                literal: "return".to_string(),
            },
            Token {
                token_type: token::FALSE,
                literal: "false".to_string(),
            },
            Token {
                token_type: token::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: token::RBRACE,
                literal: "}".to_string(),
            },
            Token {
                token_type: token::EOF,
                literal: "".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);

        for (index, expected_token) in expected_res.into_iter().enumerate() {
            let token = lexer.next_token();
            println!("{} {}", token.token_type, token.literal);
            assert_eq!(
                expected_token.token_type, token.token_type,
                "FAILED AT test[{index}] - expected {}, got {}",
                expected_token.token_type, token.token_type
            );
            assert_eq!(
                expected_token.literal, token.literal,
                "FAILED AT test[{index}] - expected {}, got {}",
                expected_token.literal, token.literal
            );
        }
    }
}
