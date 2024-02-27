pub type TokenType = &'static str;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

pub const ILLEGAL: TokenType = "ILLEGAL";
pub const EOF: TokenType = "EOF";
pub const IDENT: TokenType = "IDENT";
pub const INT: TokenType = "INT";
pub const ASSIGN: TokenType = "=";
pub const PLUS: TokenType = "+";
pub const MINUS: TokenType = "-";
pub const BANG: TokenType = "!";
pub const ASTERISK: TokenType = "*";
pub const SLASH: TokenType = "/";
pub const LT: TokenType = "<";
pub const GT: TokenType = ">";
pub const COMMA: TokenType = ",";
pub const SEMICOLON: TokenType = ";";
pub const LPAREN: TokenType = "(";
pub const RPAREN: TokenType = ")";
pub const LBRACE: TokenType = "{";
pub const RBRACE: TokenType = "}";
pub const FUNCTION: TokenType = "FUNCTION";
pub const LET: TokenType = "LET";
pub const TRUE: TokenType = "TRUE";
pub const FALSE: TokenType = "FALSE";
pub const IF: TokenType = "IF";
pub const ELSE: TokenType = "ELSE";
pub const RETURN: TokenType = "RETURN";

pub fn lookup_ident(identifier: &str) -> TokenType {
    match identifier {
        "fn" => FUNCTION,
        "let" => LET,
        "if" => IF,
        "else" => ELSE,
        "true" => TRUE,
        "false" => FALSE,
        "return" => RETURN,
        _ => IDENT,
    }
}
