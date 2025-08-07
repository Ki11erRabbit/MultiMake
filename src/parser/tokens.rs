

pub enum Token<'a> {
    // Data
    Int(&'a str),
    Float(&'a str),
    Boolean(bool),
    // Text
    Text(&'a str),
    Whitespace(&'a str),
    // Keywords
    Function,
    Task,
    If,
    Else,
    Elseif,
    For,
    In,
    // Operators
    Assign,
    Add,
    Sub,
    Mul,
    Div,
    FloorDiv,
    Rem,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,
    // Symbols
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Dollar,
    SingleQuote,
    DoubleQuote,
    Comma,
    BackSlash,
    // LineBreak
    LineBreak
}