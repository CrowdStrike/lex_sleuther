use lexgen::lexer;
use strum::{EnumCount, FromRepr};

use PythonTokenKind::*;

/*
 * Based on the grammar declared here https://github.com/antlr/grammars-v4/blob/master/python/python3/Python3Lexer.g4
 * Note that this is specifically for Python3. Python2 should generally match since its sort-of a subset.
 */
lexer! {
    pub PythonLexer -> PythonTokenKind;

    let non_zero_digit = ['1'-'9'];
    let digit = ['0'-'9'];
    let oct_digit = ['0'-'7'];
    let hex_digit = ['0'-'9' 'a'-'f' 'A'-'F'];
    let bin_digit = ['0' '1'];

    let int_part = $digit+;
    let fraction = '.' $digit+;
    let point_float = $int_part? $fraction | $int_part '.';
    let exponent = ['e' 'E'] ['+' '-']? $digit+;
    let exponent_float = ($int_part | $point_float) $exponent;

    let string_escape_sequence = '\\' _;
    let short_string = '\'' ($string_escape_sequence | (_ # ['\\' '\r' '\n' '\'']))* '\'' | '"' ($string_escape_sequence | (_ # ['\\' '\r' '\n' '"']))* '"';
    let string_prefix = (['r' 'R'] | ['u' 'U'] | ['f' 'F'] | (['f' 'F'] ['r' 'R']) | (['r' 'R'] ['f' 'F']));

    let bytes_escape_sequence = '\\' ['\u{0000}'-'\u{007F}'];
    let short_bytes_char_no_single_quote = ['\u{0000}'-'\u{0009}' '\u{000B}'-'\u{000C}' '\u{000E}'-'\u{0026}' '\u{0028}'-'\u{005B}' '\u{005D}'-'\u{007F}'];
    let short_bytes_char_no_double_quote = ['\u{0000}'-'\u{0009}' '\u{000B}'-'\u{000C}' '\u{000E}'-'\u{0021}' '\u{0023}'-'\u{005B}' '\u{005D}'-'\u{007F}'];
    let short_bytes = '\'' ($short_bytes_char_no_single_quote | $bytes_escape_sequence)* '\'' | '"' ($short_bytes_char_no_double_quote | $bytes_escape_sequence)* '"';
    let bytes_prefix = (['b' 'B'] | (['b' 'B'] ['r' 'R']) | (['r' 'R'] ['b' 'B']));


    // once again, we lack the sophistication to describe all of the unicode this matches on
    // technically, this should match on the general categories for Letters, which is over 100,000 code points
    let id_start = '_' | $$alphabetic | $$XID_Start;
    // don't even get me started on whats permitted here
    let id_continue = '-' | $id_start | $$alphanumeric | $$XID_Continue;

    rule Init {
        // comments and whitespace
        [' ' '\t']+,
        '#' (_ # ['\r' '\n'])* = Comment,
        // technically there's some special rules here since newlines have semantic meaning in python but we are ignoring that
        ('\r'? '\n' | '\r') = NewLine,

        // keywords
        "and" = AndKeyword,
        "as" = AsKeyword,
        "assert" = AssertKeyword,
        "async" = AsyncKeyword,
        "await" = AwaitKeyword,
        "break" = BreakKeyword,
        "case" = CaseKeyword,
        "class" = ClassKeyword,
        "continue" = ContinueKeyword,
        "def" = DefKeyword,
        "del" = DelKeyword,
        "elif" = ElifKeyword,
        "else" = ElseKeyword,
        "except" = ExceptKeyword,
        "False" = FalseKeyword,
        "finally" = FinallyKeyword,
        "for" = ForKeyword,
        "from" = FromKeyword,
        "global" = GlobalKeyword,
        "if" = IfKeyword,
        "import" = ImportKeyword,
        "in" = InKeyword,
        "is" = IsKeyword,
        "lambda" = LambdaKeyword,
        "match" = MatchKeyword,
        "None" = NoneKeyword,
        "nonlocal" = NonLocalKeyword,
        "not" = NotKeyword,
        "or" = OrKeyword,
        "pass" = PassKeyword,
        "raise" = RaiseKeyword,
        "return" = ReturnKeyword,
        "True" = TrueKeyword,
        "try" = TryKeyword,
        "_" = UnderscoreKeyword,
        "while" = WhileKeyword,
        "with" = WithKeyword,
        "yield" = YieldKeyword,

        // identifier
        $id_start $id_continue* = Identifier,

        // literals
        ($non_zero_digit $digit*) | ('0'+) = DecimalLiteral,
        '0' ['o' 'O'] $oct_digit+ = OctIntegerLiteral,
        '0' ['x' 'X'] $hex_digit+ = HexIntegerLiteral,
        '0' ['b' 'B'] $bin_digit+ = BinIntegerLiteral,
        $point_float | $exponent_float = FloatLiteral,
        ($point_float | $exponent_float | $int_part) ['j' 'J'] = ImaginaryNumberLiteral,

        // strings
        $string_prefix? $short_string = ShortStringLiteral,
        $string_prefix? "'''" => |lexer| lexer.switch_and_return(PythonLexerRule::LongStringSingleQuote, LongStringLiteralStart),
        $string_prefix? "\"\"\"" => |lexer| lexer.switch_and_return(PythonLexerRule::LongStringDoubleQuote, LongStringLiteralStart),
        $bytes_prefix $short_bytes = ShortBytesLiteral,
        $bytes_prefix "'''" => |lexer| lexer.switch_and_return(PythonLexerRule::LongBytesSingleQuote, LongBytesLiteralStart),
        $bytes_prefix "\"\"\"" => |lexer| lexer.switch_and_return(PythonLexerRule::LongBytesDoubleQuote, LongBytesLiteralStart),


        // symbols
        "." = DotSymbol,
        "..." = EllipsisSymbol,
        "*" = StarSymbol,
        "(" = OpenParenSymbol,
        ")" = CloseParenSymbol,
        "," = CommaSymbol,
        ":" = ColonSymbol,
        "" = SemiColonSymbol,
        "**" = PowerSymbol,
        "=" = AssignSymbol,
        "[" = OpenBrackSymbol,
        "]" = CloseBrackSymbol,
        "|" = OrOpSymbol,
        "^" = XorSymbol,
        "&" = AndOpSymbol,
        "<<" = LeftShiftSymbol,
        ">>" = RightShiftSymbol,
        "+" = AddSymbol,
        "-" = MinuxSymbol,
        "/" = DivSymbol,
        "%" = ModSymbol,
        "//" = IntDivSymbol,
        "~" = NotOpSymbol,
        "{" = OpenBraceSymbol,
        "}" = CloseBraceSymbol,
        "<" = LessThanSymbol,
        ">" = GreaterThanSymbol,
        "==" = EqualsSymbol,
        ">=" = GtEqSymbol,
        "<=" = LtEqSymbol,
        "<>" = NotEqSymbol,
        "!=" = NotEq2Symbol,
        "@" = AtSymbol,
        "->" = ArrowSymbol,
        "+=" = AddAssignSymbol,
        "-=" = SubAssignSymbol,
        "*=" = MultAssignSymbol,
        "@=" = AtAssignSymbol,
        "/=" = DivAssignSymbol,
        "%=" = ModAssignSymbol,
        "&=" = AndAssignSymbol,
        "|=" = OrAssignSymbol,
        "^=" = XorAssignSymbol,
        "<<=" = LeftShiftAssignSymbol,
        ">>=" = RightShiftAssignSymbol,
        "**=" = PowerAssignSymbol,
        "//=" = IntDivAssignSymbol,
    }

    rule LongStringSingleQuote {
        "'''" => |lexer| lexer.switch_and_return(PythonLexerRule::Init, LongStringLiteralEnd),
        _,
    }

    rule LongStringDoubleQuote {
        "\"\"\"" => |lexer| lexer.switch_and_return(PythonLexerRule::Init, LongStringLiteralEnd),
        _,
    }

    rule LongBytesSingleQuote {
        "'''" => |lexer| lexer.switch_and_return(PythonLexerRule::Init, LongBytesLiteralStart),
        _,
    }

    rule LongBytesDoubleQuote {
        "\"\"\"" => |lexer| lexer.switch_and_return(PythonLexerRule::Init, LongBytesLiteralEnd),
        _,
    }
}

#[derive(Debug, EnumCount, FromRepr)]
pub enum PythonTokenKind {
    // comments and whitespace
    Comment,
    NewLine,

    // Identifier
    Identifier,

    // keywords
    AndKeyword,
    AsKeyword,
    AssertKeyword,
    AsyncKeyword,
    AwaitKeyword,
    BreakKeyword,
    CaseKeyword,
    ClassKeyword,
    ContinueKeyword,
    DefKeyword,
    DelKeyword,
    ElifKeyword,
    ElseKeyword,
    ExceptKeyword,
    FalseKeyword,
    FinallyKeyword,
    ForKeyword,
    FromKeyword,
    GlobalKeyword,
    IfKeyword,
    ImportKeyword,
    InKeyword,
    IsKeyword,
    LambdaKeyword,
    MatchKeyword,
    NoneKeyword,
    NonLocalKeyword,
    NotKeyword,
    OrKeyword,
    PassKeyword,
    RaiseKeyword,
    ReturnKeyword,
    TrueKeyword,
    TryKeyword,
    UnderscoreKeyword,
    WhileKeyword,
    WithKeyword,
    YieldKeyword,

    // literals
    DecimalLiteral,
    OctIntegerLiteral,
    HexIntegerLiteral,
    BinIntegerLiteral,
    FloatLiteral,
    ImaginaryNumberLiteral,

    // strings
    ShortBytesLiteral,
    LongBytesLiteralStart,
    LongBytesLiteralEnd,
    ShortStringLiteral,
    LongStringLiteralStart,
    LongStringLiteralEnd,

    // symbols
    DotSymbol,
    EllipsisSymbol,
    StarSymbol,
    OpenParenSymbol,
    CloseParenSymbol,
    CommaSymbol,
    ColonSymbol,
    SemiColonSymbol,
    PowerSymbol,
    AssignSymbol,
    OpenBrackSymbol,
    CloseBrackSymbol,
    OrOpSymbol,
    XorSymbol,
    AndOpSymbol,
    LeftShiftSymbol,
    RightShiftSymbol,
    AddSymbol,
    MinuxSymbol,
    DivSymbol,
    ModSymbol,
    IntDivSymbol,
    NotOpSymbol,
    OpenBraceSymbol,
    CloseBraceSymbol,
    LessThanSymbol,
    GreaterThanSymbol,
    EqualsSymbol,
    GtEqSymbol,
    LtEqSymbol,
    NotEqSymbol,
    NotEq2Symbol,
    AtSymbol,
    ArrowSymbol,
    AddAssignSymbol,
    SubAssignSymbol,
    MultAssignSymbol,
    AtAssignSymbol,
    DivAssignSymbol,
    ModAssignSymbol,
    AndAssignSymbol,
    OrAssignSymbol,
    XorAssignSymbol,
    LeftShiftAssignSymbol,
    RightShiftAssignSymbol,
    PowerAssignSymbol,
    IntDivAssignSymbol,
}

impl From<PythonTokenKind> for usize {
    fn from(token: PythonTokenKind) -> Self {
        token as usize
    }
}