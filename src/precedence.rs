use crate::token::Token;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
pub enum Precedence {
    Lowest,      // 最低優先度
    Assign,      // =
    LogicalOr,   // ||
    LogicalAnd,  // &&
    Equals,      // ==, !=
    LessGreater, // <, >, <=, >=
    Sum,         // +, -
    Product,     // *, /, %
    Prefix,      // -x, !x
    Call,        // 関数呼び出し
    Index,       // 配列やマップのインデックス
    Factorial,   // 階乗（例：n!）
}

impl Precedence {
    pub fn is_lower_than(&self, other: &Precedence) -> bool {
        *self < *other
    }
    pub fn is_higher_than(&self, other: &Precedence) -> bool {
        *self > *other
    }
    pub fn get_precedence(token: &Token) -> Precedence {
        match token {
            Token::Assign => Precedence::Assign,
            Token::Or => Precedence::LogicalOr,
            Token::And => Precedence::LogicalAnd,
            Token::Equal | Token::NotEqual => Precedence::Equals,
            Token::LessThan
            | Token::LessThanOrEqual
            | Token::GreaterThan
            | Token::GreaterThanOrEqual => Precedence::LessGreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Asterisk | Token::Slash | Token::Percent => Precedence::Product,
            Token::Bang | Token::Minus => Precedence::Prefix,
            Token::LParen => Precedence::Call,
            Token::LBracket => Precedence::Index,
            _ => Precedence::Lowest,
        }
    }
}
