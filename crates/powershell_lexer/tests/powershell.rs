use powershell_lexer::PowerShellLexer;

const COMPREHENSIVE: &str = include_str!("./samples/comprehensive.ps1");

#[test]
fn lex_comprehensive_ps1() {
    let lexer = PowerShellLexer::new(COMPREHENSIVE);
    for token in lexer {
        if let Err(err) = token {
            panic!("invalid lex: {:?}", err);
        }
        println!("{:?}", token);
    }
}
