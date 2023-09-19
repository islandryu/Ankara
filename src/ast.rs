use std::{collections::HashMap, fmt::Display};

use crate::token::{self, Token};

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
    ReturnStatement(ReturnStatement),
    BlockReturnStatement(BlockReturnStatement),
    WatchDeclaration(WatchDeclaration),
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    InfixExpression(Box<InfixExpression>),
    NumberLiteral(NumberLiteral),
    Identifier(Identifier),
    FunctionLiteral(FunctionLiteral),
    CallExpression(Box<CallExpression>),
    IfExpression(Box<IfExpression>),
    BooleanLiteral(BooleanLiteral),
    StringLiteral(StringLiteral),
    ArrayLiteral(ArrayLiteral),
    ElementAccessExpression(Box<ElementAccessExpression>),
    ForExpression(Box<ForExpression>),
    SwitchExpression(Box<SwitchExpression>),
    Assign(Box<Assign>),
    BlockExpression(BlockExpression),
}

#[derive(Debug, PartialEq, Clone)]
pub struct InfixExpression {
    pub left: Expression,
    pub operator: Operator,
    pub right: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NumberLiteral {
    pub value: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,
    Bang,
}

impl Operator {
    pub fn get_operator(token: &Token) -> Operator {
        match token {
            Token::Plus => Operator::Plus,
            Token::Minus => Operator::Minus,
            Token::Asterisk => Operator::Asterisk,
            Token::Slash => Operator::Slash,
            Token::Percent => Operator::Percent,
            Token::Equal => Operator::Equal,
            Token::NotEqual => Operator::NotEqual,
            Token::LessThan => Operator::LessThan,
            Token::LessThanOrEqual => Operator::LessThanOrEqual,
            Token::GreaterThan => Operator::GreaterThan,
            Token::GreaterThanOrEqual => Operator::GreaterThanOrEqual,
            Token::And => Operator::And,
            Token::Or => Operator::Or,
            Token::Bang => Operator::Bang,
            _ => panic!("unexpected token"),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operator = match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Asterisk => "*",
            Operator::Slash => "/",
            Operator::Percent => "%",
            Operator::Equal => "==",
            Operator::NotEqual => "!=",
            Operator::LessThan => "<",
            Operator::LessThanOrEqual => "<=",
            Operator::GreaterThan => ">",
            Operator::GreaterThanOrEqual => ">=",
            Operator::And => "&&",
            Operator::Or => "||",
            Operator::Bang => "!",
        };
        write!(f, "{}", operator)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockExpression {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionLiteral {
    pub parameters: Vec<Identifier>,
    pub body: BlockExpression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub left: Expression,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    pub value: Expression,
}

pub struct BlockReturn {
    pub value: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfExpression {
    pub condition: Expression,
    pub consequence: BlockExpression,
    pub alternative: Option<BlockExpression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BooleanLiteral {
    pub value: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayLiteral {
    pub elements: Vec<ArrayMapValue>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArrayMapValue {
    MapKeyValue(MapKeyValue),
    Value(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub struct MapKeyValue {
    pub key: String,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementAccessExpression {
    pub left: Expression,
    pub index: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockReturnStatement {
    pub value: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrefixExpression {
    pub operator: Operator,
    pub right: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForExpression {
    pub variable: Identifier,
    pub iterable: Expression,
    pub body: BlockExpression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchExpression {
    pub expression: Expression,
    pub cases: Vec<Case>,
    pub default: Option<Default>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Case {
    pub condition: Expression,
    pub body: BlockExpression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Default {
    pub body: BlockExpression,
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::InfixExpression(infix) => write!(
                f,
                "{}",
                infix.left.to_string() + &infix.operator.to_string() + &infix.right.to_string()
            ),
            Expression::NumberLiteral(number) => write!(f, "number Literal {}", number.value),
            Expression::Identifier(identifier) => write!(f, "identifier {}", identifier.value),
            Expression::FunctionLiteral(function) => write!(f, "function",),
            Expression::CallExpression(call) => {
                write!(f, "callExpression {}", call.left.to_string())
            }
            Expression::IfExpression(if_expression) => write!(f, "if"),
            Expression::BooleanLiteral(boolean) => write!(f, "boolean {}", boolean.value),
            Expression::StringLiteral(string) => write!(f, "string {}", string.value),
            Expression::ArrayLiteral(array) => write!(f, "array"),
            Expression::ElementAccessExpression(element_access) => {
                write!(f, "element access {}", element_access.left.to_string())
            }
            Expression::ForExpression(for_expression) => {
                write!(f, "for expression")
            }
            Expression::SwitchExpression(switch_expression) => {
                write!(f, "switch expression")
            }
            Expression::Assign(assign) => {
                write!(f, "assign expression")
            }
            Expression::BlockExpression(block) => {
                write!(f, "block expression")
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Assign {
    pub left: Expression,
    pub right: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WatchDeclaration {
    pub name: String,
    pub block: BlockExpression,
}
