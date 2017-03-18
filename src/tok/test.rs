
use super::{Error, ErrorCode, Tokenizer, Tok};

fn assert_tokens(expected_tokens: Vec<Tok>, input: &str) {
    let lexer = Tokenizer::new(input, 0);
    let actual_tokens: Vec<Tok> = lexer.into_iter()
        .map(|r| {
                 let (_, t, _) = r.unwrap();
                 t
             })
        .collect();
    assert_eq!(expected_tokens, actual_tokens);
}

fn assert_error(expected_tokens: Vec<Result<Tok, Error>>, input: &str) {
    let lexer = Tokenizer::new(input, 0);
    let mut expected_tokens_it = expected_tokens.into_iter();
    for actual in lexer {
        let expected = expected_tokens_it.next().unwrap();
        match (actual, expected) {
            (Ok((_, actual_token, _)), Ok(expected_token)) => {
                assert_eq!(expected_token, actual_token);
            }
            (Err(Error { code: actual_code, .. }), Err(Error { code: expected_code, .. })) => {
                assert_eq!(expected_code, actual_code);
            }
            (actual, expected) => panic!("expected: {:?}, got: {:?}", expected, actual),
        }
    }
    let missing_token = expected_tokens_it.next();
    assert_eq!(None, missing_token);
}

#[test]
fn test_insert() {
    let expected_tokens = vec![Tok::Insert,
                               Tok::Into,
                               Tok::Id("t3"),
                               Tok::Values,
                               Tok::LeftParen,
                               Tok::StringLiteral("r c"),
                               Tok::Comma,
                               Tok::StringLiteral(""),
                               Tok::RightParen,
                               Tok::Semi];
    assert_tokens(expected_tokens, "INSERT INTO t3 VALUES( 'r c', '');");
    let expected_tokens = vec![Tok::Insert,
                               Tok::Into,
                               Tok::Id("email"),
                               Tok::LeftParen,
                               Tok::Id("from"),
                               Tok::Comma,
                               Tok::Id("to"),
                               Tok::Comma,
                               Tok::Id("subject"),
                               Tok::Comma,
                               Tok::Id("body"),
                               Tok::RightParen,
                               Tok::Values,
                               Tok::LeftParen,
                               Tok::StringLiteral("..."),
                               Tok::Comma,
                               Tok::StringLiteral("..."),
                               Tok::Comma,
                               Tok::StringLiteral("..."),
                               Tok::Comma,
                               Tok::StringLiteral("..."),
                               Tok::RightParen];
    assert_tokens(expected_tokens,
                  "INSERT INTO email([from],[to],subject,body) VALUES('...', '...', '...', '...')");
}

#[test]
fn test_comment() {
    let expected_tokens = vec![];
    assert_tokens(expected_tokens, "-- ......");
    let expected_tokens = vec![];
    assert_tokens(expected_tokens, " -- ......\n");
}

#[test]
fn test_minus() {
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Minus, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a-b");
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Minus, Tok::Minus, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a - -b");
}

#[test]
fn test_plus() {
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Plus, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a+b");
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Plus, Tok::Plus, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a + +b");
}

#[test]
fn test_star() {
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Star, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a*b");
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Star, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a * b");
}

#[test]
fn test_block_comment() {
    let expected_tokens = vec![Tok::Select, Tok::Id("a")];
    assert_tokens(expected_tokens, "SELECT a/*b***/");
}

#[test]
fn test_slash() {
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Slash, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a/b");
}

#[test]
fn test_reminder() {
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Reminder, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a%b");
}

#[test]
fn test_equals() {
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Equals, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a=b");
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Equals, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a==b");
}

#[test]
fn test_less() {
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::LessThan, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a<b");
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::LessEquals, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a<=b");
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::NotEquals, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a<>b");
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::LeftShift, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a<<b");
}

#[test]
fn test_greater() {
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::GreaterThan, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a>b");
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::GreaterEquals, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a>=b");
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::RightShift, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a>>b");
}

#[test]
fn test_exclamation_mark() {
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::NotEquals, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a!=b");
}

#[test]
fn test_exclamation_mark_alone() {
    let expected_tokens = vec![Ok(Tok::Select),
                               Ok(Tok::Id("a")),
                               super::error(ErrorCode::ExpectedEqualsSign, 0, ""),
                               Ok(Tok::Id("b"))];
    assert_error(expected_tokens, "SELECT a!b");
}

#[test]
fn test_pipe() {
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::BitOr, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a|b");
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Concat, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a||b");
}

#[test]
fn test_comma() {
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Comma, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a,b");
}

#[test]
fn test_ampersand() {
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::BitAnd, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a&b");
}

#[test]
fn test_tilde() {
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::BitNot, Tok::Id("b")];
    assert_tokens(expected_tokens, "SELECT a~b");
}

#[test]
fn test_literal() {
    let expected_tokens = vec![Tok::Select, Tok::StringLiteral("")];
    assert_tokens(expected_tokens, "SELECT ''");
    let expected_tokens = vec![Tok::Select, Tok::Id("")];
    assert_tokens(expected_tokens, "SELECT \"\"");
    let expected_tokens = vec![Tok::Select, Tok::Id("")];
    assert_tokens(expected_tokens, "SELECT ``");

    let expected_tokens = vec![Tok::Select, Tok::StringLiteral("hel''lo")];
    assert_tokens(expected_tokens, "SELECT 'hel''lo'");
    let expected_tokens = vec![Tok::Select, Tok::Id("hel\"\"lo")];
    assert_tokens(expected_tokens, "SELECT \"hel\"\"lo\"");
    let expected_tokens = vec![Tok::Select, Tok::Id("hel``lo")];
    assert_tokens(expected_tokens, "SELECT `hel``lo`");

    let expected_tokens = vec![Tok::Select, Tok::StringLiteral("hel''lo''")];
    assert_tokens(expected_tokens, "SELECT 'hel''lo'''");

    let expected_tokens = vec![Tok::Select, Tok::StringLiteral("''hel''lo''")];
    assert_tokens(expected_tokens, "SELECT '''hel''lo'''");

    let expected_tokens = vec![Ok(Tok::Select),
                               super::error(ErrorCode::UnterminatedLiteral, 0, "")];
    assert_error(expected_tokens, "SELECT '");
    let expected_tokens = vec![Ok(Tok::Select),
                               super::error(ErrorCode::UnterminatedLiteral, 0, "")];
    assert_error(expected_tokens, "SELECT '''");
    let expected_tokens = vec![Ok(Tok::Select),
                               super::error(ErrorCode::UnterminatedLiteral, 0, "")];
    assert_error(expected_tokens, "SELECT 'hel''");
    let expected_tokens = vec![Ok(Tok::Select),
                               super::error(ErrorCode::UnterminatedLiteral, 0, "")];
    assert_error(expected_tokens, "SELECT 'hel''lo''");
}

#[test]
fn test_dot() {
    let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Dot, Tok::Star];
    assert_tokens(expected_tokens, "SELECT a.*");
    let expected_tokens = vec![Tok::Select, Tok::Dot, Tok::Star];
    assert_tokens(expected_tokens, "SELECT .*");

    let expected_tokens = vec![Tok::Select, Tok::Float(".1234")];
    assert_tokens(expected_tokens, "SELECT .1234");
}

#[test]
fn test_number() {
    let expected_tokens = vec![Tok::Select, Tok::Float("3.14")];
    assert_tokens(expected_tokens, "SELECT 3.14");

    let expected_tokens = vec![Tok::Select, Tok::Float("2.5E10")];
    assert_tokens(expected_tokens, "SELECT 2.5E10");
    let expected_tokens = vec![Tok::Select, Tok::Float("2.5E10")];
    assert_tokens(expected_tokens, "SELECT 2.5E10 ");
    let expected_tokens = vec![Tok::Select, Tok::Float("2.5E10"), Tok::Semi];
    assert_tokens(expected_tokens, "SELECT 2.5E10;");

    let expected_tokens = vec![Tok::Select, Tok::Float("2.5e10")];
    assert_tokens(expected_tokens, "SELECT 2.5e10");
    let expected_tokens = vec![Tok::Select, Tok::Float("2.5e1")];
    assert_tokens(expected_tokens, "SELECT 2.5e1");

    let expected_tokens = vec![Tok::Select, Tok::Float("2.5E-10")];
    assert_tokens(expected_tokens, "SELECT 2.5E-10");
    let expected_tokens = vec![Tok::Select, Tok::Float("2.5E+1")];
    assert_tokens(expected_tokens, "SELECT 2.5E+1");

    let expected_tokens = vec![Tok::Select, Tok::Float("5.")];
    assert_tokens(expected_tokens, "SELECT 5.");

    let expected_tokens = vec![Tok::Select, Tok::Float(".5")];
    assert_tokens(expected_tokens, "SELECT .5");

    let expected_tokens = vec![Tok::Select, Tok::Float("0.5")];
    assert_tokens(expected_tokens, "SELECT 0.5");

    let expected_tokens = vec![Tok::Select, Tok::Integer("5")];
    assert_tokens(expected_tokens, "SELECT 5");
    let expected_tokens = vec![Tok::Select, Tok::Integer("2501")];
    assert_tokens(expected_tokens, "SELECT 2501");

    let expected_tokens = vec![Ok(Tok::Select), super::error(ErrorCode::BadNumber, 0, "")];
    assert_error(expected_tokens, "SELECT 1e");

    let expected_tokens = vec![Ok(Tok::Select), super::error(ErrorCode::BadNumber, 0, "")];
    assert_error(expected_tokens, "SELECT 1e-");
    let expected_tokens = vec![Ok(Tok::Select), super::error(ErrorCode::BadNumber, 0, "")];
    assert_error(expected_tokens, "SELECT 1e+");

    let expected_tokens = vec![Ok(Tok::Select), super::error(ErrorCode::BadNumber, 0, "")];
    assert_error(expected_tokens, "SELECT 1_");
    let expected_tokens = vec![Ok(Tok::Select), super::error(ErrorCode::BadNumber, 0, "")];
    assert_error(expected_tokens, "SELECT 1.0_");
    let expected_tokens = vec![Ok(Tok::Select), super::error(ErrorCode::BadNumber, 0, "")];
    assert_error(expected_tokens, "SELECT 1.0e5_");
}

#[test]
fn test_hex_integer() {
    let expected_tokens = vec![Tok::Select, Tok::Integer("0x5")];
    assert_tokens(expected_tokens, "SELECT 0x5");
    let expected_tokens = vec![Tok::Select, Tok::Integer("0X52")];
    assert_tokens(expected_tokens, "SELECT 0X52");
    let expected_tokens = vec![Tok::Select, Tok::Integer("0Xff")];
    assert_tokens(expected_tokens, "SELECT 0Xff");

    let expected_tokens = vec![Ok(Tok::Select),
                               super::error(ErrorCode::MalformedHexInteger, 0, "")];
    assert_error(expected_tokens, "SELECT 0Xfg");

    let expected_tokens = vec![Ok(Tok::Select),
                               super::error(ErrorCode::MalformedHexInteger, 0, "")];
    assert_error(expected_tokens, "SELECT 0Xg");
}

#[test]
fn test_bracket() {
    let expected_tokens = vec![Tok::Select, Tok::Id("a")];
    assert_tokens(expected_tokens, "SELECT [a]");
    let expected_tokens = vec![Tok::Select, Tok::Id("")];
    assert_tokens(expected_tokens, "SELECT []");

    let expected_tokens = vec![Ok(Tok::Select),
                               super::error(ErrorCode::UnterminatedBracket, 0, "")];
    assert_error(expected_tokens, "SELECT [");
    let expected_tokens = vec![Ok(Tok::Select),
                               super::error(ErrorCode::UnterminatedBracket, 0, "")];
    assert_error(expected_tokens, "SELECT [abc");
}

#[test]
fn test_question_mark() {
    let expected_tokens = vec![Tok::Select, Tok::Variable("")];
    assert_tokens(expected_tokens, "SELECT ?");
    let expected_tokens = vec![Tok::Select, Tok::Variable("?1")];
    assert_tokens(expected_tokens, "SELECT ?1");
    let expected_tokens = vec![Tok::Select, Tok::Variable("?12")];
    assert_tokens(expected_tokens, "SELECT ?12");
}

#[test]
fn test_dollar() {
    let expected_tokens = vec![Tok::Select, Tok::Variable("$a")];
    assert_tokens(expected_tokens, "SELECT $a");

    let expected_tokens = vec![Ok(Tok::Select), super::error(ErrorCode::BadVariableName, 0, "")];
    assert_error(expected_tokens, "SELECT $");
}

#[test]
fn test_at() {
    let expected_tokens = vec![Tok::Select, Tok::Variable("@a")];
    assert_tokens(expected_tokens, "SELECT @a");

    let expected_tokens = vec![Ok(Tok::Select), super::error(ErrorCode::BadVariableName, 0, "")];
    assert_error(expected_tokens, "SELECT @");
}

#[test]
fn test_hash() {
    let expected_tokens = vec![Tok::Select, Tok::Variable("#a")];
    assert_tokens(expected_tokens, "SELECT #a");

    let expected_tokens = vec![Ok(Tok::Select), super::error(ErrorCode::BadVariableName, 0, "")];
    assert_error(expected_tokens, "SELECT #");
}

#[test]
fn test_colon() {
    let expected_tokens = vec![Tok::Select, Tok::Variable(":a")];
    assert_tokens(expected_tokens, "SELECT :a");

    let expected_tokens = vec![Ok(Tok::Select), super::error(ErrorCode::BadVariableName, 0, "")];
    assert_error(expected_tokens, "SELECT :");

    let expected_tokens =
        vec![Ok(Tok::Select), super::error(ErrorCode::BadVariableName, 0, ""), Ok(Tok::Comma)];
    assert_error(expected_tokens, "SELECT :,");
}

#[test]
fn test_blob_literal() {
    let expected_tokens = vec![Tok::Select, Tok::Id("x0")];
    assert_tokens(expected_tokens, "SELECT x0");

    let expected_tokens = vec![Tok::Select, Tok::Blob("abcde123")];
    assert_tokens(expected_tokens, "SELECT x'abcde123'");
    let expected_tokens = vec![Tok::Select, Tok::Blob("abcde123")];
    assert_tokens(expected_tokens, "SELECT X'abcde123'");

    let expected_tokens =
        vec![Ok(Tok::Select), super::error(ErrorCode::MalformedBlobLiteral, 0, ""), Ok(Tok::Comma)];
    assert_error(expected_tokens, "SELECT x'adcef',");

    let expected_tokens =
        vec![Ok(Tok::Select), super::error(ErrorCode::MalformedBlobLiteral, 0, ""), Ok(Tok::Comma)];
    assert_error(expected_tokens, "SELECT x'adcefg',");

    let expected_tokens = vec![Ok(Tok::Select),
                               super::error(ErrorCode::MalformedBlobLiteral, 0, "")];
    assert_error(expected_tokens, "SELECT x'adcef");
}

#[test]
fn test_illegal_char() {
    let expected_tokens = vec![super::error(ErrorCode::UnrecognizedToken, 0, "")];
    assert_error(expected_tokens, "\\");
    let expected_tokens = vec![super::error(ErrorCode::UnrecognizedToken, 0, "")];
    assert_error(expected_tokens, "{");
    let expected_tokens = vec![super::error(ErrorCode::UnrecognizedToken, 0, "")];
    assert_error(expected_tokens, "}");
}
