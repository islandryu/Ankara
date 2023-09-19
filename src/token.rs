use logos::Logos;
use std::fmt;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\f]+")]
pub enum Token {
    #[token("\n")]
    Newline,
    #[token("//")]
    Comment,
    #[regex("[a-zA-Z][a-zA-Z0-9]*")]
    Identifier,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token("||")]
    Or,
    #[token("&&")]
    And,
    #[token("!=")]
    NotEqual,
    #[token("==")]
    Equal,
    #[token("<")]
    LessThan,
    #[token("<=")]
    LessThanOrEqual,
    #[token(">")]
    GreaterThan,
    #[token(">=")]
    GreaterThanOrEqual,
    #[token("!")]
    Bang,
    #[token("%")]
    Percent,
    #[regex("[0-9]+")]
    Number,
    // if
    #[token("if")]
    If,
    // else
    #[token("else")]
    Else,
    // brackets
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    // braces
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    // brackets
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    // semicolon
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    // assignment
    #[token("=")]
    Assign,
    #[token("let")]
    Let,
    #[token(",")]
    Comma,
    #[token("fn")]
    Function,
    #[token("return")]
    Return,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[regex(r#""[^"]*""#)]
    String,
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("switch")]
    Switch,
    #[token("case")]
    Case,
    #[token("default")]
    Default,
    #[token("watch")]
    Watch,
}

impl Token {
    pub fn is_infix_operator(&self) -> bool {
        match self {
            Token::Plus
            | Token::Minus
            | Token::Asterisk
            | Token::Slash
            | Token::Or
            | Token::And
            | Token::NotEqual
            | Token::Equal
            | Token::LessThan
            | Token::LessThanOrEqual
            | Token::GreaterThan
            | Token::GreaterThanOrEqual
            | Token::Percent => true,
            _ => false,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Identifier => write!(f, "Identifier"),
            Token::Plus => write!(f, "Plus"),
            Token::Minus => write!(f, "Minus"),
            Token::Asterisk => write!(f, "Asterisk"),
            Token::Slash => write!(f, "Slash"),
            Token::Or => write!(f, "Or"),
            Token::And => write!(f, "And"),
            Token::NotEqual => write!(f, "NotEqual"),
            Token::Equal => write!(f, "Equal"),
            Token::LessThan => write!(f, "LessThan"),
            Token::LessThanOrEqual => write!(f, "LessThanOrEqual"),
            Token::GreaterThan => write!(f, "GreaterThan"),
            Token::GreaterThanOrEqual => write!(f, "GreaterThanOrEqual"),
            Token::Bang => write!(f, "Bang"),
            Token::Percent => write!(f, "Percent"),
            Token::Number => write!(f, "Number"),
            Token::If => write!(f, "If"),
            Token::Else => write!(f, "Else"),
            Token::LParen => write!(f, "LParen"),
            Token::RParen => write!(f, "RParen"),
            Token::LBrace => write!(f, "LBrace"),
            Token::RBrace => write!(f, "RBrace"),
            Token::LBracket => write!(f, "LBracket"),
            Token::RBracket => write!(f, "RBracket"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::Colon => write!(f, "Colon"),
            Token::Assign => write!(f, "Assign"),
            Token::Let => write!(f, "Let"),
            Token::Comma => write!(f, "Comma"),
            Token::Function => write!(f, "Function"),
            Token::Return => write!(f, "Return"),
            Token::True => write!(f, "True"),
            Token::False => write!(f, "False"),
            Token::String => write!(f, "String"),
            Token::Newline => write!(f, "Newline"),
            Token::For => write!(f, "For"),
            Token::In => write!(f, "In"),
            Token::Switch => write!(f, "Switch"),
            Token::Case => write!(f, "Case"),
            Token::Default => write!(f, "Default"),
            Token::Watch => write!(f, "Watch"),
            Token::Comment => write!(f, "Comment"),
        }
    }
}
