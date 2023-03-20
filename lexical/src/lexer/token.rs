#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token {
    EOF,
    ILLEGAL,
    KEYWORD,
    OPERATOR(char),
    INT(Vec<char>),
    IDENTIFIER(Vec<char>)
}

pub fn get_keyword_token(ident: &Vec<char>) -> Result<Token, String> {
    let identifier: String = ident.into_iter().collect();
    match &identifier[..] {
        "fn" => Ok(Token::KEYWORD),
        "let" => Ok(Token::KEYWORD),
        "true" => Ok(Token::KEYWORD),
        "false" => Ok(Token::KEYWORD),
        "if" => Ok(Token::KEYWORD),
        "else" => Ok(Token::KEYWORD),
        "return" => Ok(Token::KEYWORD),
        _ => Err(String::from("Not a keyword"))
    }
}