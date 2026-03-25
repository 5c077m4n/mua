use logos::Logos;
use mua::libs::tokens::Token;

#[test]
fn test_keywords() {
	let keywords = [
		("and", Token::And),
		("break", Token::Break),
		("do", Token::Do),
		("else", Token::Else),
		("elseif", Token::ElseIf),
		("end", Token::End),
		("false", Token::Boolean(false)),
		("for", Token::For),
		("function", Token::Function),
		("if", Token::If),
		("in", Token::In),
		("local", Token::Local),
		("nil", Token::Nil),
		("not", Token::Not),
		("or", Token::Or),
		("repeat", Token::Repeat),
		("return", Token::Return),
		("then", Token::Then),
		("true", Token::Boolean(true)),
		("until", Token::Until),
		("while", Token::While),
	];

	for (keyword, expected_token) in keywords {
		let mut lex = Token::lexer(keyword);
		assert_eq!(lex.next(), Some(Ok(expected_token)));
		assert_eq!(lex.slice(), keyword);
		assert_eq!(lex.next(), None);
	}
}

#[test]
fn test_operators() {
	let operators = [
		("+", Token::Plus),
		("-", Token::Minus),
		("*", Token::Star),
		("/", Token::Slash),
		("%", Token::Percent),
		("^", Token::Caret),
		("#", Token::Hash),
		("==", Token::Equal),
		("~=", Token::NotEqual),
		("<", Token::LessThan),
		(">", Token::GreaterThan),
		("<=", Token::LessThanOrEqual),
		(">=", Token::GreaterThanOrEqual),
		("=", Token::Assign),
		("(", Token::LeftParen),
		(")", Token::RightParen),
		("{", Token::LeftBrace),
		("}", Token::RightBrace),
		("[", Token::LeftBracket),
		("]", Token::RightBracket),
		(";", Token::Semicolon),
		(":", Token::Colon),
		(",", Token::Comma),
		(".", Token::Dot),
		("..", Token::Concat),
		("...", Token::Vararg),
	];

	for (str, expected_token) in operators {
		let mut lex = Token::lexer(str);
		assert_eq!(lex.next(), Some(Ok(expected_token)));
		assert_eq!(lex.slice(), str);
	}
}

#[test]
fn test_identifier() {
	let mut lex = Token::lexer("foo bar123 _under");

	assert_eq!(lex.next(), Some(Ok(Token::Name("foo".into()))));
	assert_eq!(lex.slice(), "foo");

	assert_eq!(lex.next(), Some(Ok(Token::Name("bar123".into()))));
	assert_eq!(lex.slice(), "bar123");

	assert_eq!(lex.next(), Some(Ok(Token::Name("_under".into()))));
	assert_eq!(lex.slice(), "_under");
}

#[test]
fn test_numbers() {
	let numbers = ["42", "3.14", "0.5", "1e10", "2.5e-3", "1E+5"];

	for num in numbers {
		let mut lex = Token::lexer(num);
		assert_eq!(lex.next(), Some(Ok(Token::Number(num.parse().unwrap()))));
		assert_eq!(lex.slice(), num);
		assert_eq!(lex.next(), None);
	}
}

#[test]
fn test_strings() {
	let strings = [
		(r#""hello""#, "hello"),
		(r#"'world'"#, "world"),
		(r#""escaped \" quote""#, r#"escaped \" quote"#),
	];

	for (input, expected) in strings {
		let mut lex = Token::lexer(input);
		assert_eq!(lex.next(), Some(Ok(Token::String(expected.to_string()))));
	}
}

#[test]
fn test_comments() {
	let source = r#"
    -- single line comment
    --[[
        block comment
    ]]"#;
	let mut lex = Token::lexer(source);

	assert_eq!(
		lex.next(),
		Some(Ok(Token::LineComment("-- single line comment".to_string())))
	);
	assert_eq!(lex.slice(), "-- single line comment");

	assert_eq!(
		lex.next(),
		Some(Ok(Token::BlockComment("--[[ block comment ]]".to_string())))
	);
	assert_eq!(lex.slice(), "--[[ block comment ]]");
}

#[test]
fn test_skip_whitespace() {
	let source = "  \t\n\n  local   x   =   42  ";
	let lex = Token::lexer(source);

	let tokens: Vec<Result<Token, _>> = lex.collect();
	assert!(tokens.iter().all(|t| t.is_ok()));

	let values: Vec<&Token> = tokens.iter().map(|t| t.as_ref().unwrap()).collect();
	assert_eq!(
		values,
		&[
			&Token::Local,
			&Token::Name("x".into()),
			&Token::Assign,
			&Token::Number(42.0)
		]
	);
}

#[test]
fn test_assignment() {
	let source = "local x = 42";
	let mut lex = Token::lexer(source);

	assert_eq!(lex.next(), Some(Ok(Token::Local)));
	assert_eq!(lex.slice(), "local");

	assert_eq!(lex.next(), Some(Ok(Token::Name("x".into()))));
	assert_eq!(lex.slice(), "x");

	assert_eq!(lex.next(), Some(Ok(Token::Assign)));
	assert_eq!(lex.slice(), "=");

	assert_eq!(lex.next(), Some(Ok(Token::Number(42.0))));
}

#[test]
fn test_function_call() {
	let source = r#"print("hello")"#;
	let mut lex = Token::lexer(source);

	assert_eq!(lex.next(), Some(Ok(Token::Name("print".into()))));
	assert_eq!(lex.slice(), "print");

	assert_eq!(lex.next(), Some(Ok(Token::LeftParen)));
	assert_eq!(lex.next(), Some(Ok(Token::String("hello".to_string()))));
	assert_eq!(lex.next(), Some(Ok(Token::RightParen)));
}

#[test]
fn test_table_access() {
	let source = "t[key]";
	let mut lex = Token::lexer(source);

	assert_eq!(lex.next(), Some(Ok(Token::Name("t".into()))));
	assert_eq!(lex.slice(), "t");

	assert_eq!(lex.next(), Some(Ok(Token::LeftBracket)));
	assert_eq!(lex.next(), Some(Ok(Token::Name("key".into()))));
	assert_eq!(lex.next(), Some(Ok(Token::RightBracket)));
}

#[test]
fn test_priority_concat_vs_dot() {
	let source = "..";
	let mut lex = Token::lexer(source);

	assert_eq!(lex.next(), Some(Ok(Token::Concat)));
}

#[test]
fn test_priority_equal_vs_assign() {
	let source = "==";
	let mut lex = Token::lexer(source);

	assert_eq!(lex.next(), Some(Ok(Token::Equal)));

	let source = "=";
	let mut lex = Token::lexer(source);

	assert_eq!(lex.next(), Some(Ok(Token::Assign)));
}

#[test]
fn test_block_comment_multiline() {
	let source = "--[[\nmulti line\ncomment\n]]";
	let mut lex = Token::lexer(source);

	assert_eq!(
		lex.next(),
		Some(Ok(Token::BlockComment(
			"--[[\nmulti line\ncomment\n]]".to_string()
		)))
	);
}
