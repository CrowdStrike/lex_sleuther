use lex_sleuther_multiplexer::HtmlLexer;

const COMPREHENSIVE: &str = include_str!("./samples/comprehensive.html");

#[test]
fn lex_comprehensive_html() {
    let lexer = HtmlLexer::new(COMPREHENSIVE);
    for token in lexer {
        if let Err(err) = token {
            panic!("invalid lex: {:?}", err);
        }
        println!("{:?}", token);
    }
}
