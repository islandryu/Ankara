use crate::Token;
use logos::Lexer;
use logos::Logos;

pub struct Peekable<'source> {
    lexer: Lexer<'source, Token>,
    pub peeked: Option<Token>,
    pub peeked_slice: Option<&'source str>,
    pub current: Option<Token>,
    pub current_slice: Option<&'source str>,
    pub is_newline: bool,
}

impl<'source> Peekable<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            lexer: Token::lexer(source),
            peeked: None,
            peeked_slice: None,
            current: None,
            current_slice: None,
            is_newline: false,
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        if self.peeked.is_none() {
            let mut next = self.lexer.next();

            //skip newline
            while let Some(token) = next.clone() {
                match token {
                    Ok(Token::Newline) => {
                        self.is_newline = true;
                        next = self.lexer.next();
                    }
                    Ok(Token::Comment) => {
                        next = self.lexer.next();
                        while let Some(token) = next {
                            match token {
                                Ok(Token::Newline) => {
                                    self.is_newline = true;
                                    next = self.lexer.next();
                                    break;
                                }
                                _ => {
                                    next = self.lexer.next();
                                }
                            }
                        }
                    }
                    _ => {
                        break;
                    }
                }
            }

            self.peeked = match next {
                Some(token) => match token {
                    Ok(token) => Some(token.clone()),
                    _ => None,
                },
                _ => None,
            };
            self.peeked_slice = match &self.peeked {
                Some(_) => Some(self.lexer.slice()),
                _ => None,
            };
        }
        self.peeked.as_ref()
    }
}

impl<'source> Iterator for Peekable<'source> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.peeked.is_none() {
            self.peek();
        }
        match self.peeked.take() {
            Some(token) => {
                self.current = Some(token);
                self.current_slice = self.peeked_slice.take();
                self.current.clone()
            }
            _ => None,
        }
    }
}

// test peekable
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peekable() {
        let mut peekable = Peekable::new(
            "\
        let x = 1;\
        return x;\
        ",
        );
        assert_eq!(peekable.peek(), Some(&Token::Let));
        assert_eq!(peekable.peek(), Some(&Token::Let));
        assert_eq!(peekable.next(), Some(Token::Let));
        assert_eq!(peekable.peek(), Some(&Token::Identifier));
        assert_eq!(peekable.next(), Some(Token::Identifier));
        assert_eq!(peekable.next(), Some(Token::Assign));
        assert_eq!(peekable.peek(), Some(&Token::Number));
        assert_eq!(peekable.next(), Some(Token::Number));
        assert_eq!(peekable.peek(), Some(&Token::Semicolon));
        assert_eq!(peekable.next(), Some(Token::Semicolon));
        assert_eq!(peekable.peek(), Some(&Token::Return));
        assert_eq!(peekable.next(), Some(Token::Return));
        assert_eq!(peekable.peek(), Some(&Token::Identifier));
        assert_eq!(peekable.next(), Some(Token::Identifier));
        assert_eq!(peekable.peek(), Some(&Token::Semicolon));
        assert_eq!(peekable.next(), Some(Token::Semicolon));
        assert_eq!(peekable.peek(), None);
        assert_eq!(peekable.next(), None);
    }
}
