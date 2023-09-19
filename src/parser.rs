use std::error::Error;
use std::fmt::Display;

use crate::ast;
use crate::ast::Identifier;
use crate::ast::Operator;
use crate::lexer::Peekable;
use crate::precedence;
use crate::precedence::Precedence;
use crate::token::Token;
use logos::Lexer;
use logos::Logos;

#[derive(Debug, PartialEq, Clone)]
pub struct ParseError {
    pub message: String,
    child: Option<Box<ParseError>>,
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ParseError: {}", self.message)
    }
}

pub fn parse(lexer: &mut Peekable<'_>) -> Result<ast::Program, ParseError> {
    let mut statements: Vec<ast::Statement> = vec![];
    while lexer.peek().is_some() {
        let statement = match parse_statement(lexer) {
            Ok(statement) => statement,
            Err(error) => return Err(error),
        };
        statements.push(statement);
    }
    return Ok(ast::Program {
        statements: statements,
    });
}

pub fn parse_statement(lexer: &mut Peekable<'_>) -> Result<ast::Statement, ParseError> {
    let token = match lexer.peek() {
        Some(token) => token,
        _ => {
            return Err(ParseError {
                message: "unexpected end of file".to_string(),
                child: None,
            })
        }
    };
    match token {
        Token::Let => match parse_variable_declaration(lexer) {
            Ok(variable_declaration) => {
                match lexer.peek() {
                    Some(Token::Semicolon) => {
                        lexer.next();
                    }
                    _ => {
                        return Err(ParseError {
                            message: "expected semicolon".to_string(),
                            child: None,
                        })
                    }
                };
                return Ok(ast::Statement::VariableDeclaration(variable_declaration));
            }
            Err(error) => return Err(error),
        },
        Token::Return => match parse_return_statement(lexer) {
            Ok(return_statement) => {
                match lexer.peek() {
                    Some(Token::Semicolon) => {
                        lexer.next();
                    }
                    _ => {
                        return Err(ParseError {
                            message: "expected semicolon".to_string(),
                            child: None,
                        })
                    }
                };
                return Ok(ast::Statement::ReturnStatement(return_statement));
            }
            Err(error) => return Err(error),
        },
        Token::Watch => match parse_watch_declaration(lexer) {
            Ok(watch_statement) => {
                match lexer.peek() {
                    Some(Token::Semicolon) => {
                        lexer.next();
                    }
                    _ => {
                        return Err(ParseError {
                            message: "expected semicolon".to_string(),
                            child: None,
                        })
                    }
                };
                return Ok(ast::Statement::WatchDeclaration(watch_statement));
            }
            Err(error) => return Err(error),
        },
        _ => match parse_expression(lexer, Precedence::Lowest) {
            Ok(expression) => {
                let peeked = lexer.peek().cloned();
                if peeked.is_some() && peeked.as_ref().unwrap() == &Token::Semicolon {
                    lexer.next();
                    return Ok(ast::Statement::Expression(expression));
                } else {
                    return Ok(ast::Statement::BlockReturnStatement(
                        ast::BlockReturnStatement { value: expression },
                    ));
                }
            }
            Err(error) => return Err(error),
        },
    }
}

fn parse_variable_declaration(
    lexer: &mut Peekable<'_>,
) -> Result<ast::VariableDeclaration, ParseError> {
    match lexer.next() {
        Some(Token::Let) => {}
        _ => {
            return Err(ParseError {
                message: "expected let".to_string(),
                child: None,
            })
        }
    };
    match lexer.next() {
        Some(Token::Identifier) => {}
        _ => {
            return Err(ParseError {
                message: "expected identifier".to_string(),
                child: None,
            })
        }
    };
    let name = lexer.current_slice.unwrap().to_string();
    match lexer.next() {
        Some(Token::Assign) => {}
        Some(token) => {
            return Err(ParseError {
                message: "expected assign after ".to_string()
                    + &name
                    + " but got "
                    + &token.to_string(),
                child: None,
            })
        }
        _ => {
            return Err(ParseError {
                message: "expected assign".to_string(),
                child: None,
            })
        }
    };
    let value = match parse_expression(lexer, Precedence::Lowest) {
        Ok(expression) => expression,
        Err(error) => return Err(error),
    };
    return Ok({
        ast::VariableDeclaration {
            name: name,
            value: value,
        }
    });
}

pub fn parse_expression(
    lexer: &mut Peekable,
    precedence: Precedence,
) -> Result<ast::Expression, ParseError> {
    let next = lexer.peek();
    let mut left = match next {
        Some(Token::Number) => {
            lexer.next();
            ast::Expression::NumberLiteral(ast::NumberLiteral {
                value: lexer.current_slice.unwrap().parse::<i32>().unwrap(),
            })
        }
        Some(Token::Identifier) => {
            lexer.next();
            ast::Expression::Identifier(ast::Identifier {
                value: lexer.current_slice.unwrap().to_string(),
            })
        }
        Some(Token::Function) => match parse_function_expression(lexer) {
            Ok(function_declaration) => ast::Expression::FunctionLiteral(function_declaration),
            Err(error) => return Err(error),
        },
        Some(Token::If) => match parse_if_expression(lexer) {
            Ok(if_expression) => ast::Expression::IfExpression(Box::new(if_expression)),
            Err(error) => return Err(error),
        },
        Some(Token::True) => {
            lexer.next();
            ast::Expression::BooleanLiteral(ast::BooleanLiteral { value: true })
        }
        Some(Token::False) => {
            lexer.next();
            ast::Expression::BooleanLiteral(ast::BooleanLiteral { value: false })
        }
        Some(Token::String) => {
            lexer.next();
            let value = lexer.current_slice.unwrap().to_string();
            //  unwrap double quotes
            let value = value[1..value.len() - 1].to_string();
            ast::Expression::StringLiteral(ast::StringLiteral { value: value })
        }
        Some(Token::LBracket) => match parse_array_literal(lexer) {
            Ok(array_literal) => ast::Expression::ArrayLiteral(array_literal),
            Err(error) => return Err(error),
        },
        Some(Token::LParen) => {
            lexer.next();
            let expression = match parse_expression(lexer, Precedence::Lowest) {
                Ok(expression) => expression,
                Err(error) => return Err(error),
            };
            match lexer.next() {
                Some(Token::RParen) => {}
                _ => {
                    return Err(ParseError {
                        message: "expected )".to_string(),
                        child: None,
                    })
                }
            };
            expression
        }
        Some(Token::For) => match parse_for_expression(lexer) {
            Ok(for_expression) => ast::Expression::ForExpression(Box::new(for_expression)),
            Err(error) => return Err(error),
        },
        Some(Token::Switch) => match parse_switch_expression(lexer) {
            Ok(switch_expression) => ast::Expression::SwitchExpression(Box::new(switch_expression)),
            Err(error) => return Err(error),
        },
        Some(Token::LBrace) => match parse_block_statement(lexer) {
            Ok(block_statement) => ast::Expression::BlockExpression(block_statement),
            Err(error) => return Err(error),
        },
        _ => {
            print!("unexpected token: {:?}", lexer.peek());
            return Err(ParseError {
                message: "unexpected token".to_string(),
                child: None,
            });
        }
    };
    let mut peeked = lexer.peek().cloned();

    while peeked.is_some()
        && peeked.as_ref().unwrap() != &Token::Semicolon
        && precedence.is_lower_than(&Precedence::get_precedence(peeked.as_ref().unwrap()))
    {
        let expression = match peeked.as_ref().unwrap() {
            Token::LParen => match parse_call_expression(lexer, left) {
                Ok(call_expression) => ast::Expression::CallExpression(Box::new(call_expression)),
                Err(error) => return Err(error),
            },
            Token::LBracket => match parse_element_access_expression(lexer, left) {
                Ok(element_access_expression) => {
                    ast::Expression::ElementAccessExpression(Box::new(element_access_expression))
                }
                Err(error) => return Err(error),
            },
            Token::Assign => match parse_assign(lexer, left) {
                Ok(assign) => ast::Expression::Assign(Box::new(assign)),
                Err(error) => return Err(error),
            },
            _ => match parse_infix_expression(lexer, left) {
                Ok(infix_expression) => {
                    ast::Expression::InfixExpression(Box::new(infix_expression))
                }
                Err(error) => return Err(error),
            },
        };
        left = expression;
        peeked = lexer.peek().cloned();
    }

    Ok(left)
}

fn parse_infix_expression(
    lexer: &mut Peekable,
    left: ast::Expression,
) -> Result<ast::InfixExpression, ParseError> {
    let token = match lexer.next() {
        Some(token) => token,
        _ => {
            return Err(ParseError {
                message: "unexpected end of file".to_string(),
                child: None,
            })
        }
    };
    let precedence = Precedence::get_precedence(&token);
    let right = match parse_expression(lexer, precedence) {
        Ok(expression) => expression,
        Err(error) => return Err(error),
    };
    return Ok(ast::InfixExpression {
        left: left,
        operator: Operator::get_operator(&token),
        right: right,
    });
}

fn parse_assign(lexer: &mut Peekable, left: ast::Expression) -> Result<ast::Assign, ParseError> {
    lexer.next();
    let right = match parse_expression(lexer, Precedence::Lowest) {
        Ok(expression) => expression,
        Err(error) => return Err(error),
    };
    return Ok(ast::Assign {
        left: left,
        right: right,
    });
}

fn parse_function_expression(lexer: &mut Peekable) -> Result<ast::FunctionLiteral, ParseError> {
    match lexer.next() {
        Some(Token::Function) => {}
        _ => {
            return Err(ParseError {
                message: "expected function".to_string(),
                child: None,
            })
        }
    };
    match lexer.next() {
        Some(Token::LParen) => {}
        _ => {
            return Err(ParseError {
                message: "expected (".to_string(),
                child: None,
            })
        }
    };
    let mut parameters: Vec<ast::Identifier> = vec![];
    let mut peeked = lexer.peek().cloned();
    while peeked.is_some() && peeked.as_ref().unwrap() != &Token::RParen {
        match lexer.next() {
            Some(Token::Identifier) => {}
            _ => {
                return Err(ParseError {
                    message: "expected identifier".to_string(),
                    child: None,
                })
            }
        };
        parameters.push(ast::Identifier {
            value: lexer.current_slice.unwrap().to_string(),
        });
        peeked = lexer.peek().cloned();
        if peeked.is_some() && peeked.as_ref().unwrap() == &Token::Comma {
            lexer.next();
        }
        peeked = lexer.peek().cloned();
    }
    match lexer.next() {
        Some(Token::RParen) => {}
        _ => {
            return Err(ParseError {
                message: "expected )".to_string(),
                child: None,
            })
        }
    };
    match lexer.next() {
        Some(Token::LBrace) => {}
        _ => {
            return Err(ParseError {
                message: "expected {".to_string(),
                child: None,
            })
        }
    };
    let mut statements: Vec<ast::Statement> = vec![];
    peeked = lexer.peek().cloned();
    while peeked.is_some() && peeked.as_ref().unwrap() != &Token::RBrace {
        let statement = match parse_statement(lexer) {
            Ok(statement) => statement,
            Err(error) => return Err(error),
        };
        statements.push(statement);
        peeked = lexer.peek().cloned();
    }
    match lexer.next() {
        Some(Token::RBrace) => {}
        _ => {
            return Err(ParseError {
                message: "expected }".to_string(),
                child: None,
            })
        }
    };
    return Ok(ast::FunctionLiteral {
        parameters: parameters,
        body: ast::BlockExpression {
            statements: statements,
        },
    });
}

fn parse_call_expression(
    lexer: &mut Peekable,
    left: ast::Expression,
) -> Result<ast::CallExpression, ParseError> {
    match lexer.next() {
        Some(Token::LParen) => {}
        _ => {
            return Err(ParseError {
                message: "expected (".to_string(),
                child: None,
            })
        }
    };
    let mut arguments: Vec<ast::Expression> = vec![];
    let mut peeked = lexer.peek().cloned();
    while peeked.is_some() && peeked.as_ref().unwrap() != &Token::RParen {
        let expression = match parse_expression(lexer, Precedence::Lowest) {
            Ok(expression) => expression,
            Err(error) => return Err(error),
        };
        arguments.push(expression);
        peeked = lexer.peek().cloned();
        if peeked.is_some() && peeked.as_ref().unwrap() == &Token::Comma {
            lexer.next();
        }
        peeked = lexer.peek().cloned();
    }
    match lexer.next() {
        Some(Token::RParen) => {}
        _ => {
            return Err(ParseError {
                message: "expected )".to_string(),
                child: None,
            })
        }
    };
    return Ok(ast::CallExpression { left, arguments });
}

fn parse_return_statement(lexer: &mut Peekable) -> Result<ast::ReturnStatement, ParseError> {
    match lexer.next() {
        Some(Token::Return) => {}
        _ => {
            return Err(ParseError {
                message: "expected return".to_string(),
                child: None,
            })
        }
    };
    let expression = match parse_expression(lexer, Precedence::Lowest) {
        Ok(expression) => expression,
        Err(error) => return Err(error),
    };
    return Ok(ast::ReturnStatement { value: expression });
}

fn parse_if_expression(lexer: &mut Peekable) -> Result<ast::IfExpression, ParseError> {
    match lexer.next() {
        Some(Token::If) => {}
        _ => {
            return Err(ParseError {
                message: "expected if".to_string(),
                child: None,
            })
        }
    };
    match lexer.next() {
        Some(Token::LParen) => {}
        _ => {
            return Err(ParseError {
                message: "expected (".to_string(),
                child: None,
            })
        }
    };
    let condition = match parse_expression(lexer, Precedence::Lowest) {
        Ok(expression) => expression,
        Err(error) => return Err(error),
    };
    match lexer.next() {
        Some(Token::RParen) => {}
        _ => {
            return Err(ParseError {
                message: "expected )".to_string(),
                child: None,
            })
        }
    };
    match lexer.peek() {
        Some(Token::LBrace) => {}
        _ => {
            return Err(ParseError {
                message: "expected {".to_string(),
                child: None,
            })
        }
    };
    let consequence = parse_block_statement(lexer);
    match lexer.peek() {
        Some(Token::Else) => {
            lexer.next();
            match lexer.peek() {
                Some(Token::LBrace) => {}
                _ => {
                    return Err(ParseError {
                        message: "expected {".to_string(),
                        child: None,
                    })
                }
            };
            let alternative = parse_block_statement(lexer);
            return Ok(ast::IfExpression {
                condition: condition,
                consequence: consequence.unwrap(),
                alternative: Some(alternative.unwrap()),
            });
        }
        Some(_) => {
            return Ok(ast::IfExpression {
                condition: condition,
                consequence: consequence.unwrap(),
                alternative: None,
            });
        }
        _ => {
            return Err(ParseError {
                message: "expected {".to_string(),
                child: None,
            });
        }
    };
}

fn parse_block_statement(lexer: &mut Peekable) -> Result<ast::BlockExpression, ParseError> {
    match lexer.next() {
        Some(Token::LBrace) => {}
        _ => {
            return Err(ParseError {
                message: "expected {".to_string(),
                child: None,
            })
        }
    };
    let mut statements: Vec<ast::Statement> = vec![];
    let mut peeked = lexer.peek().cloned();
    while peeked.is_some() && peeked.as_ref().unwrap() != &Token::RBrace {
        let statement = match parse_statement(lexer) {
            Ok(statement) => statement,
            Err(error) => return Err(error),
        };
        statements.push(statement);
        peeked = lexer.peek().cloned();
    }
    match lexer.next() {
        Some(Token::RBrace) => {}
        _ => {
            return Err(ParseError {
                message: "expected }".to_string(),
                child: None,
            })
        }
    };
    return Ok(ast::BlockExpression {
        statements: statements,
    });
}

fn parse_array_literal(lexer: &mut Peekable) -> Result<ast::ArrayLiteral, ParseError> {
    match lexer.next() {
        Some(Token::LBracket) => {}
        _ => {
            return Err(ParseError {
                message: "expected [".to_string(),
                child: None,
            })
        }
    };
    let elements = match parse_comma_separated(lexer) {
        Ok(elements) => elements,
        Err(error) => return Err(error),
    };
    match lexer.next() {
        Some(Token::RBracket) => {}
        _ => {
            return Err(ParseError {
                message: "expected ]".to_string(),
                child: None,
            })
        }
    };
    return Ok(ast::ArrayLiteral { elements });
}

fn parse_comma_separated(lexer: &mut Peekable<'_>) -> Result<Vec<ast::ArrayMapValue>, ParseError> {
    let mut elements: Vec<ast::ArrayMapValue> = vec![];
    let mut peeked = lexer.peek().cloned();
    while peeked.is_some() && peeked.as_ref().unwrap() != &Token::RBracket {
        let expression = match parse_expression(lexer, Precedence::Lowest) {
            Ok(expression) => expression,
            Err(error) => return Err(error),
        };
        peeked = lexer.peek().cloned();
        if peeked.is_some() && peeked.as_ref().unwrap() == &Token::Colon {
            let key = match expression {
                ast::Expression::Identifier(identifier) => identifier.value,
                _ => {
                    return Err(ParseError {
                        message: "expected string literal".to_string(),
                        child: None,
                    })
                }
            };
            lexer.next();
            let value = match parse_expression(lexer, Precedence::Lowest) {
                Ok(expression) => expression,
                Err(error) => return Err(error),
            };
            elements.push(ast::ArrayMapValue::MapKeyValue(ast::MapKeyValue {
                key: key,
                value: value,
            }));
            peeked = lexer.peek().cloned();
        } else {
            elements.push(ast::ArrayMapValue::Value(expression));
        }

        if peeked.is_some() && peeked.as_ref().unwrap() == &Token::Comma {
            lexer.next();
        }
        peeked = lexer.peek().cloned();
    }
    return Ok(elements);
}

fn parse_element_access_expression(
    lexer: &mut Peekable,
    left: ast::Expression,
) -> Result<ast::ElementAccessExpression, ParseError> {
    match lexer.next() {
        Some(Token::LBracket) => {}
        _ => {
            return Err(ParseError {
                message: "expected [".to_string(),
                child: None,
            })
        }
    };
    let index = match parse_expression(lexer, Precedence::Lowest) {
        Ok(expression) => expression,
        Err(error) => return Err(error),
    };
    match lexer.next() {
        Some(Token::RBracket) => {}
        _ => {
            return Err(ParseError {
                message: "expected ]".to_string(),
                child: None,
            })
        }
    };
    return Ok(ast::ElementAccessExpression { left, index });
}

fn parse_for_expression(lexer: &mut Peekable) -> Result<ast::ForExpression, ParseError> {
    match lexer.next() {
        Some(Token::For) => {}
        _ => {
            return Err(ParseError {
                message: "expected for".to_string(),
                child: None,
            })
        }
    };
    match lexer.next() {
        Some(Token::LParen) => {}
        _ => {
            return Err(ParseError {
                message: "expected (".to_string(),
                child: None,
            })
        }
    };
    match lexer.next() {
        Some(Token::Identifier) => {}
        _ => {
            return Err(ParseError {
                message: "expected identifier".to_string(),
                child: None,
            })
        }
    };
    let name = lexer.current_slice.unwrap().to_string();
    match lexer.next() {
        Some(Token::In) => {}
        _ => {
            return Err(ParseError {
                message: "expected in".to_string(),
                child: None,
            })
        }
    };
    let array = match parse_expression(lexer, Precedence::Lowest) {
        Ok(expression) => expression,
        Err(error) => return Err(error),
    };
    match lexer.next() {
        Some(Token::RParen) => {}
        _ => {
            return Err(ParseError {
                message: "expected )".to_string(),
                child: None,
            })
        }
    };
    let block_statement = match parse_block_statement(lexer) {
        Ok(block_statement) => block_statement,
        Err(error) => return Err(error),
    };
    return Ok(ast::ForExpression {
        variable: ast::Identifier { value: name },
        iterable: array,
        body: block_statement,
    });
}

fn parse_switch_expression(lexer: &mut Peekable) -> Result<ast::SwitchExpression, ParseError> {
    match lexer.next() {
        Some(Token::Switch) => {}
        _ => {
            return Err(ParseError {
                message: "expected switch".to_string(),
                child: None,
            })
        }
    };
    match lexer.next() {
        Some(Token::LParen) => {}
        _ => {
            return Err(ParseError {
                message: "expected (".to_string(),
                child: None,
            })
        }
    };
    let expression = match parse_expression(lexer, Precedence::Lowest) {
        Ok(expression) => expression,
        Err(error) => return Err(error),
    };
    match lexer.next() {
        Some(Token::RParen) => {}
        _ => {
            return Err(ParseError {
                message: "expected )".to_string(),
                child: None,
            })
        }
    };
    match lexer.next() {
        Some(Token::LBrace) => {}
        _ => {
            return Err(ParseError {
                message: "expected {".to_string(),
                child: None,
            })
        }
    };
    let mut cases: Vec<ast::Case> = vec![];
    let mut peeked = lexer.peek().cloned();
    while peeked.is_some()
        && peeked.as_ref().unwrap() != &Token::RBrace
        && peeked.as_ref().unwrap() != &Token::Default
    {
        let case = match parse_case(lexer) {
            Ok(case) => case,
            Err(error) => return Err(error),
        };
        cases.push(case);
        peeked = lexer.peek().cloned();
    }
    peeked = lexer.peek().cloned();
    let default = match peeked {
        Some(Token::Default) => match parse_default(lexer) {
            Ok(default) => Some(default),
            Err(error) => return Err(error),
        },
        _ => None,
    };

    match lexer.next() {
        Some(Token::RBrace) => {}
        _ => {
            return Err(ParseError {
                message: "expected }".to_string(),
                child: None,
            })
        }
    };
    return Ok(ast::SwitchExpression {
        expression: expression,
        cases: cases,
        default: default,
    });
}

fn parse_case(lexer: &mut Peekable) -> Result<ast::Case, ParseError> {
    match lexer.next() {
        Some(Token::Case) => {}
        _ => {
            return Err(ParseError {
                message: "expected case".to_string(),
                child: None,
            })
        }
    };
    let expression = match parse_expression(lexer, Precedence::Lowest) {
        Ok(expression) => expression,
        Err(error) => return Err(error),
    };
    match lexer.next() {
        Some(Token::Colon) => {}
        _ => {
            return Err(ParseError {
                message: "expected :".to_string(),
                child: None,
            })
        }
    };
    let block_statement = match parse_block_statement(lexer) {
        Ok(block_statement) => block_statement,
        Err(error) => return Err(error),
    };
    return Ok(ast::Case {
        condition: expression,
        body: block_statement,
    });
}

fn parse_default(lexer: &mut Peekable) -> Result<ast::Default, ParseError> {
    match lexer.next() {
        Some(Token::Default) => {}
        _ => {
            return Err(ParseError {
                message: "expected default".to_string(),
                child: None,
            })
        }
    };
    match lexer.next() {
        Some(Token::Colon) => {}
        _ => {
            return Err(ParseError {
                message: "expected :".to_string(),
                child: None,
            })
        }
    };
    let block_statement = match parse_block_statement(lexer) {
        Ok(block_statement) => block_statement,
        Err(error) => return Err(error),
    };
    return Ok(ast::Default {
        body: block_statement,
    });
}

fn parse_watch_declaration(lexer: &mut Peekable) -> Result<ast::WatchDeclaration, ParseError> {
    match lexer.next() {
        Some(Token::Watch) => {}
        _ => {
            return Err(ParseError {
                message: "expected watch".to_string(),
                child: None,
            })
        }
    };
    match lexer.next() {
        Some(Token::Identifier) => {}
        _ => {
            return Err(ParseError {
                message: "expected identifier".to_string(),
                child: None,
            })
        }
    };
    let name = lexer.current_slice.unwrap().to_string();
    match lexer.next() {
        Some(Token::Assign) => {}
        _ => {
            return Err(ParseError {
                message: "expected assign".to_string(),
                child: None,
            })
        }
    };
    let value = match parse_block_statement(lexer) {
        Ok(expression) => expression,
        Err(error) => return Err(error),
    };
    return Ok(ast::WatchDeclaration {
        name: name,
        block: value,
    });
}

// test parser
#[cfg(test)]
mod tests {
    use crate::ast::{Expression, VariableDeclaration};

    use super::*;

    #[test]
    fn test_parse() {
        let mut lexer = Peekable::new("let x = 1;");
        let program = parse(&mut lexer).unwrap();
        assert_eq!(
            program,
            ast::Program {
                statements: vec![ast::Statement::VariableDeclaration(
                    ast::VariableDeclaration {
                        name: "x".to_string(),
                        value: ast::Expression::NumberLiteral(ast::NumberLiteral { value: 1 }),
                    }
                )],
            }
        );
    }
    #[test]
    fn test_infix_expression() {
        let mut lexer = Peekable::new("1 + 2;");
        let expression = parse_expression(&mut lexer, Precedence::Lowest).unwrap();
        assert_eq!(
            expression,
            Expression::InfixExpression(Box::new(ast::InfixExpression {
                left: Expression::NumberLiteral(ast::NumberLiteral { value: 1 }),
                operator: Operator::Plus,
                right: Expression::NumberLiteral(ast::NumberLiteral { value: 2 }),
            }))
        );

        let mut lexer = Peekable::new("1 + 2 * 3;");
        let expression = parse_expression(&mut lexer, Precedence::Lowest).unwrap();
        assert_eq!(
            expression,
            Expression::InfixExpression(Box::new(ast::InfixExpression {
                left: Expression::NumberLiteral(ast::NumberLiteral { value: 1 }),
                operator: Operator::Plus,
                right: Expression::InfixExpression(Box::new(ast::InfixExpression {
                    left: Expression::NumberLiteral(ast::NumberLiteral { value: 2 }),
                    operator: Operator::Asterisk,
                    right: Expression::NumberLiteral(ast::NumberLiteral { value: 3 }),
                })),
            }))
        );

        let mut lexer = Peekable::new("1 * 2 + 3;");
        let expression = parse_expression(&mut lexer, Precedence::Lowest).unwrap();
        assert_eq!(
            expression,
            Expression::InfixExpression(Box::new(ast::InfixExpression {
                left: Expression::InfixExpression(Box::new(ast::InfixExpression {
                    left: Expression::NumberLiteral(ast::NumberLiteral { value: 1 }),
                    operator: Operator::Asterisk,
                    right: Expression::NumberLiteral(ast::NumberLiteral { value: 2 }),
                })),
                operator: Operator::Plus,
                right: Expression::NumberLiteral(ast::NumberLiteral { value: 3 }),
            }))
        );
    }

    #[test]
    // identifier
    fn test_parse_identifier() {
        let mut lexer = Peekable::new("x;");
        let expression = parse_expression(&mut lexer, Precedence::Lowest).unwrap();
        assert_eq!(
            expression,
            Expression::Identifier(ast::Identifier {
                value: "x".to_string(),
            })
        );
    }

    #[test]
    fn test_parse_multiple_statements() {
        let mut lexer = Peekable::new(
            "\
            let x = 1;\
            let y = 2;\
            x + y;\
        ",
        );
        let program = parse(&mut lexer).unwrap();
        assert_eq!(
            program,
            ast::Program {
                statements: vec![
                    ast::Statement::VariableDeclaration(ast::VariableDeclaration {
                        name: "x".to_string(),
                        value: ast::Expression::NumberLiteral(ast::NumberLiteral { value: 1 }),
                    }),
                    ast::Statement::VariableDeclaration(ast::VariableDeclaration {
                        name: "y".to_string(),
                        value: ast::Expression::NumberLiteral(ast::NumberLiteral { value: 2 }),
                    }),
                    ast::Statement::Expression(ast::Expression::InfixExpression(Box::new(
                        ast::InfixExpression {
                            left: ast::Expression::Identifier(ast::Identifier {
                                value: "x".to_string(),
                            }),
                            operator: Operator::Plus,
                            right: ast::Expression::Identifier(ast::Identifier {
                                value: "y".to_string(),
                            }),
                        }
                    )))
                ],
            }
        );
    }
    #[test]
    fn test_parse_function_expression() {
        let mut lexer = Peekable::new(
            "\
            let a = fn(x, y) {\
                x + y;\
            };\
        ",
        );
        let variableDeclaration = parse_variable_declaration(&mut lexer).unwrap();
        assert_eq!(
            variableDeclaration,
            VariableDeclaration {
                name: "a".to_string(),
                value: Expression::FunctionLiteral(ast::FunctionLiteral {
                    parameters: vec![
                        ast::Identifier {
                            value: "x".to_string(),
                        },
                        ast::Identifier {
                            value: "y".to_string(),
                        }
                    ],
                    body: ast::BlockExpression {
                        statements: vec![ast::Statement::Expression(
                            ast::Expression::InfixExpression(Box::new(ast::InfixExpression {
                                left: ast::Expression::Identifier(ast::Identifier {
                                    value: "x".to_string(),
                                }),
                                operator: Operator::Plus,
                                right: ast::Expression::Identifier(ast::Identifier {
                                    value: "y".to_string(),
                                }),
                            }))
                        )],
                    },
                }),
            }
        )
    }
    #[test]
    fn test_parse_call_expression() {
        let mut lexer = Peekable::new(
            "\
            add(1, 2);\
        ",
        );
        let expression = parse_expression(&mut lexer, Precedence::Lowest).unwrap();
        assert_eq!(
            expression,
            Expression::CallExpression(Box::new(ast::CallExpression {
                left: ast::Expression::Identifier(ast::Identifier {
                    value: "add".to_string(),
                }),
                arguments: vec![
                    ast::Expression::NumberLiteral(ast::NumberLiteral { value: 1 }),
                    ast::Expression::NumberLiteral(ast::NumberLiteral { value: 2 }),
                ],
            }))
        );
    }
    #[test]
    fn test_parse_if_expression() {
        let mut lexer = Peekable::new(
            "\
            if (x < y) {\
                x;\
            } else {\
                y;\
            }\
            ",
        );
        let expression = parse_expression(&mut lexer, Precedence::Lowest).unwrap();
        assert_eq!(
            expression,
            Expression::IfExpression(Box::new(ast::IfExpression {
                condition: ast::Expression::InfixExpression(Box::new(ast::InfixExpression {
                    left: ast::Expression::Identifier(ast::Identifier {
                        value: "x".to_string(),
                    }),
                    operator: Operator::LessThan,
                    right: ast::Expression::Identifier(ast::Identifier {
                        value: "y".to_string(),
                    }),
                })),
                consequence: ast::BlockExpression {
                    statements: vec![ast::Statement::Expression(ast::Expression::Identifier(
                        ast::Identifier {
                            value: "x".to_string(),
                        }
                    ))],
                },
                alternative: Some(ast::BlockExpression {
                    statements: vec![ast::Statement::Expression(ast::Expression::Identifier(
                        ast::Identifier {
                            value: "y".to_string(),
                        }
                    ))],
                }),
            }))
        );
    }
    #[test]
    fn test_parse_array() {
        let mut lexer = Peekable::new(
            "\
            [1, 2, 3, myKey: 4];\
        ",
        );
        let expression = parse_expression(&mut lexer, Precedence::Lowest).unwrap();
        assert_eq!(
            expression,
            Expression::ArrayLiteral(ast::ArrayLiteral {
                elements: vec![
                    ast::ArrayMapValue::Value(ast::Expression::NumberLiteral(ast::NumberLiteral {
                        value: 1
                    })),
                    ast::ArrayMapValue::Value(ast::Expression::NumberLiteral(ast::NumberLiteral {
                        value: 2
                    })),
                    ast::ArrayMapValue::Value(ast::Expression::NumberLiteral(ast::NumberLiteral {
                        value: 3
                    })),
                    ast::ArrayMapValue::MapKeyValue(ast::MapKeyValue {
                        key: "myKey".to_string(),
                        value: ast::Expression::NumberLiteral(ast::NumberLiteral { value: 4 }),
                    }),
                ],
            })
        );
    }

    #[test]
    fn test_comment() {
        let mut lexer = Peekable::new(
            "\
            // comment
            let x = 1;
            ",
        );
        let program = parse(&mut lexer).unwrap();
        assert_eq!(
            program,
            ast::Program {
                statements: vec![ast::Statement::VariableDeclaration(
                    ast::VariableDeclaration {
                        name: "x".to_string(),
                        value: ast::Expression::NumberLiteral(ast::NumberLiteral { value: 1 }),
                    }
                )],
            }
        );
    }
}
