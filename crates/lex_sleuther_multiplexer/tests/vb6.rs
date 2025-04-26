use lex_sleuther_multiplexer::Vb6Lexer;

const COMPREHENSIVE: &str = include_str!("./samples/comprehensive.vb");

#[test]
fn lex_comprehensive_vb() {
    let lexer = Vb6Lexer::new(COMPREHENSIVE);
    for token in lexer {
        if let Err(err) = token {
            panic!("invalid lex: {:?}", err);
        }
        println!("{:?}", token);
    }
}
