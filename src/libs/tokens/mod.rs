use logos::Logos;

#[derive(Debug, Clone, PartialEq, Logos)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
	#[token("and")]
	And,
	#[token("break")]
	Break,
	#[token("do")]
	Do,
	#[token("else")]
	Else,
	#[token("elseif")]
	ElseIf,
	#[token("end")]
	End,
	#[token("true", |_| true)]
	#[token("flase", |_| false)]
	Boolean(bool),
	#[token("for")]
	For,
	#[token("function")]
	Function,
	#[token("if")]
	If,
	#[token("in")]
	In,
	#[token("local")]
	Local,
	#[token("nil")]
	Nil,
	#[token("not")]
	Not,
	#[token("or")]
	Or,
	#[token("repeat")]
	Repeat,
	#[token("return")]
	Return,
	#[token("then")]
	Then,
	#[token("until")]
	Until,
	#[token("while")]
	While,

	#[token("+")]
	Plus,
	#[token("-")]
	Minus,
	#[token("*")]
	Star,
	#[token("/")]
	Slash,
	#[token("%")]
	Percent,
	#[token("^")]
	Caret,
	#[token("#")]
	Hash,
	#[token("==")]
	Equal,
	#[token("~=")]
	NotEqual,
	#[token("<")]
	LessThan,
	#[token(">")]
	GreaterThan,
	#[token("<=")]
	LessThanOrEqual,
	#[token(">=")]
	GreaterThanOrEqual,
	#[token("=")]
	Assign,
	#[token("(")]
	LeftParen,
	#[token(")")]
	RightParen,
	#[token("{")]
	LeftBrace,
	#[token("}")]
	RightBrace,
	#[token("[")]
	LeftBracket,
	#[token("]")]
	RightBracket,
	#[token(";")]
	Semicolon,
	#[token(":")]
	Colon,
	#[token(",")]
	Comma,
	#[token(".")]
	Dot,
	#[token("..")]
	Concat,
	#[token("...")]
	Vararg,

	#[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
	Name(String),

	#[regex(r#""[^"\\]*(?:\\.[^"\\]*)*"|'[^'\\]*(?:\\.[^'\\]*)*'"#, |lex| {
        let slice = lex.slice();
        slice[1..slice.len()-1].to_string()
    })]
	String(String),

	#[regex(r"[0-9]+\.?[0-9]*([eE][+-]?[0-9]+)?", |lex| lex.slice().parse::<f64>().ok())]
	Number(f64),

	#[regex(r"--.*", |lex| lex.slice().to_string(), allow_greedy = true)]
	LineComment(String),
	#[regex(r"--\[\[[\s\S]*?\]\]", |lex| lex.slice().to_string())]
	BlockComment(String),
}

use std::fmt;

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Token::And => write!(f, "and"),
			Token::Break => write!(f, "break"),
			Token::Do => write!(f, "do"),
			Token::Else => write!(f, "else"),
			Token::ElseIf => write!(f, "elseif"),
			Token::End => write!(f, "end"),
			Token::Boolean(b) => write!(f, "{}", b),
			Token::For => write!(f, "for"),
			Token::Function => write!(f, "function"),
			Token::If => write!(f, "if"),
			Token::In => write!(f, "in"),
			Token::Local => write!(f, "local"),
			Token::Nil => write!(f, "nil"),
			Token::Not => write!(f, "not"),
			Token::Or => write!(f, "or"),
			Token::Repeat => write!(f, "repeat"),
			Token::Return => write!(f, "return"),
			Token::Then => write!(f, "then"),
			Token::Until => write!(f, "until"),
			Token::While => write!(f, "while"),
			Token::Plus => write!(f, "+"),
			Token::Minus => write!(f, "-"),
			Token::Star => write!(f, "*"),
			Token::Slash => write!(f, "/"),
			Token::Percent => write!(f, "%"),
			Token::Caret => write!(f, "^"),
			Token::Hash => write!(f, "#"),
			Token::Equal => write!(f, "=="),
			Token::NotEqual => write!(f, "~="),
			Token::LessThan => write!(f, "<"),
			Token::GreaterThan => write!(f, ">"),
			Token::LessThanOrEqual => write!(f, "<="),
			Token::GreaterThanOrEqual => write!(f, ">="),
			Token::Assign => write!(f, "="),
			Token::LeftParen => write!(f, "("),
			Token::RightParen => write!(f, ")"),
			Token::LeftBrace => write!(f, "{{"),
			Token::RightBrace => write!(f, "}}"),
			Token::LeftBracket => write!(f, "["),
			Token::RightBracket => write!(f, "]"),
			Token::Semicolon => write!(f, ";"),
			Token::Colon => write!(f, ":"),
			Token::Comma => write!(f, ","),
			Token::Dot => write!(f, "."),
			Token::Concat => write!(f, ".."),
			Token::Vararg => write!(f, "..."),
			Token::Name(n) => write!(f, "{}", n),
			Token::String(s) => write!(f, "\"{}\"", s),
			Token::Number(n) => write!(f, "{}", n),
			Token::LineComment(c) => write!(f, "-- {}", c),
			Token::BlockComment(c) => write!(f, "--[[ {} ]]", c),
		}
	}
}
