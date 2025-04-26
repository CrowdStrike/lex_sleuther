use lex_sleuther_multiplexer::{BatchLexer, BatchTokenKind};

const SYNTAX_TEST: &str = include_str!("./samples/syntax_test.bat");

#[test]
fn lex_syntax_test() {
    let lowercased = SYNTAX_TEST.to_lowercase().to_owned();
    let lexer = BatchLexer::new(lowercased.as_str());
    for token in lexer {
        match token {
            Err(err) => panic!("invalid lex: {:?}", err),
            Ok((start, BatchTokenKind::NewLine, _)) => print!("\n[{}]: ", start.line + 2),
            Ok((_, token, _)) => print!("{:?},", token),
        }
    }
}
