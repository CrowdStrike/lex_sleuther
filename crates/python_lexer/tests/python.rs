use python_lexer::PythonLexer;

const COMPREHENSIVE: &str = include_str!("./samples/comprehensive.py");

#[test]
fn test_comprehensive_python() {
    let lexer = PythonLexer::new(COMPREHENSIVE);
    for token in lexer {
        if let Err(err) = token {
            panic!("invalid lex: {:?}", err);
        }
        println!("{:?}", token);
    }
}