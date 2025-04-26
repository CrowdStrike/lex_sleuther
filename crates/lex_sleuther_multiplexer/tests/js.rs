use lex_sleuther_multiplexer::JsLexer;

const COMPREHENSIVE: &str = include_str!("./samples/comprehensive.js");

#[test]
fn lex_comprehensive_js() {
    let lexer = JsLexer::new_with_state(COMPREHENSIVE, 0);
    for token in lexer {
        if let Err(err) = token {
            panic!("invalid lex: {:?}", err);
        }
        println!("{:?}", token);
    }
}
