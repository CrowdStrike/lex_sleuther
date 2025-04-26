use lexgen::lexer;
use strum::{EnumCount, FromRepr};

use JsTokenKind::*;

/*
 * Based on the grammar declared here https://github.com/antlr/grammars-v4/blob/master/javascript/javascript/JavaScriptLexer.g4
 * This lexer has an unusual degree of validation that I have decided to translate verbatim.
 * There are some distinctions related to strict-mode vs non-strict mode that we are basically ignoring.
 */
lexer! {
    // state "usize" is the current template string depth
    pub JsLexer(usize) -> JsTokenKind;

    let decimal_integer_literal = '0' | ['1'-'9'] ['0'-'9' '_']*;
    let exponent_part = ['e' 'E'] ['+' '-']? ['0'-'9' '_']+;
    let hex_digit = ['0'-'9' 'a'-'f' 'A'-'F' '_'];

    // escape sequences
    let single_escape_char = ['\'' '"' '\\' 'b' 'f' 'n' 'r' 't' 'v'];
    let non_escape_char = (_ # ['\'' '"' '\\' 'b' 'f' 'n' 'r' 't' 'v' '0'-'9' 'x' 'u' '\r' '\n']);
    let character_escape_sequence = $single_escape_char | $non_escape_char;
    let unicode_escape_sequence = ('u' $hex_digit $hex_digit $hex_digit $hex_digit) | ("u{" $hex_digit+ '}');
    let hex_escape_sequence = 'x' $hex_digit $hex_digit;
    let escape_sequence = $unicode_escape_sequence | $character_escape_sequence | $hex_escape_sequence;
    
    // unfortunately, we lack the sophistication to properly describe the complete set of valid identifier characters.
    // specifically, we need NonSpacingMark, DecimalDigitNumber, and PunctuationConnector.
    let identifier_start = $$alphabetic | '_' | ('\\' $unicode_escape_sequence);
    let identifier_part = $identifier_start | '\u{200C}' | '\u{200D}' | '-' | $$numeric;

    // regex in js is whacky whacky whacky
    let regular_expression_backslash_sequence = '\\' (_ # ['\n' '\r' '\u{2028}' '\u{2029}']);
    let regular_expression_class_char = (_ # ['\n' '\r' '\u{2028}' '\u{2029}' ']' '\\'])
        | $regular_expression_backslash_sequence;
    let regular_expression_char = (_ # ['\n' '\r' '\u{2028}' '\u{2029}' '/' '[' '\\'])
        | $regular_expression_backslash_sequence
        | '[' $regular_expression_class_char* ']';
    let regular_expression_first_char = (_ # ['\n' '\r' '\u{2028}' '\u{2029}' '/' '[' '\\' '*'])
        | $regular_expression_backslash_sequence
        | '[' $regular_expression_class_char* ']';

    let line_continuation = '\\' ['\n' '\r' '\u{2028}' '\u{2029}']+;
    let double_string_char = (_ # ['"' '\\' '\r' '\n']) | '\\' $escape_sequence | $line_continuation;
    let single_string_char = (_ # ['\'' '\\' '\r' '\n']) | '\\' $escape_sequence | $line_continuation;

    rule Init {
        // whitespace (ignored)
        ['\t' '\u{000B}' '\u{000C}' '\u{0020}' '\u{00A0}']+,
        ['\r' '\n' '\u{2028}' '\u{2029}'] = LineTerminator,

        // comments
        "//" (_ # ['\n' '\r' '\u{2028}' '\u{2029}'])* = SingleLineComment,
        "/*" => |lexer| lexer.switch_and_return(JsLexerRule::MultiLineComment, MultiLineCommentStart),

        // skipping declaration of hashbang since we can't validate it happened at the beginning of the file

        // symbols
        '[' = OpenBracketSymbol,
        ']' = CloseBracketSymbol,
        '(' = OpenParenSymbol,
        ')' = CloseParenSymbol,
        '{' = OpenBraceSymbol,
        // this could be just a close brace, or we could be terminating a template string expression
        '}' => |lexer| {
            if *lexer.state() > 0 {
                lexer.switch_and_return(JsLexerRule::Template, TemplateStringExpressionEnd)
            } else {
                lexer.return_(CloseBraceSymbol)
            }
        },
        ';' = SemiColonSymbol,
        ',' = CommaSymbol,
        '=' = AssignSymbol,
        '?' = QuestionMarkSymbol,
        "?." = QuestionMarkDotSymbol,
        ':' = ColonSymbol,
        "..." = EllipsisSymbol,
        '.' = DotSymbol,
        "++" = PlusPlusSymbol,
        "--" = MinusMinusSymbol,
        '+' = PlusSymbol,
        '-' = MinusSymbol,
        '~' = BitNotSymbol,
        '!' = NotSymbol,
        '*' = MultiplySymbol,
        '/' = DivideSymbol,
        '%' = ModulusSymbol,
        "**" = PowerSymbol,
        "??" = NullCoalesceSymbol,
        '#' = HashtagSymbol,
        ">>" = RightShiftArithmeticSymbol,
        "<<" = LeftShiftArithmeticSymbol,
        ">>>" = RightShiftLogicalSymbol,
        '<' = LessThanSymbol,
        '>' = MoreThanSymbol,
        "<=" = LessThanEqualsSymbol,
        ">=" = GreaterThanEqualsSymbol,
        "==" = EqualsSymbol,
        "!=" = NotEqualsSymbol,
        "===" = IdentityEqualsSymbol,
        "!==" = IdentityNotEqualsSymbol,
        "&" = BitAndSymbol,
        "^" = BitXOrSymbol,
        "|" = BitOrSymbol,
        "&&" = AndSymbol,
        "||" = OrSymbol,
        "*=" = MultiplyAssignSymbol,
        "/=" = DivideAssignSymbol,
        "%=" = ModulusAssignSymbol,
        "+=" = PlusAssignSymbol,
        "-=" = MinusAssignSymbol,
        "<<=" = LeftShiftArithmeticAssignSymbol,
        ">>=" = RightShiftArithmeticAssignSymbol,
        ">>>=" = RightShiftLogicalAssignSymbol,
        "&=" = BitAndAssignSymbol,
        "^=" = BitXorAssignSymbol,
        "|=" = BitOrAssignSymbol,
        "**=" = PowerAssignSymbol,
        "??=" = NullishCoalescingAssignSymbol,
        "=>" = ArrowSymbol,

        // literals
        "null" = NullLiteral,
        "true" | "false" = BooleanLiteral,
        ($decimal_integer_literal '.' ['0'-'9'] ['0'-'9' '_']* $exponent_part?)
            | ('.' ['0'-'9'] ['0'-'9' '_']* $exponent_part?)
            | ($decimal_integer_literal $exponent_part?) = DecimalLiteral,
        '0' ['x' 'X'] ['0'-'9' 'a'-'f' 'A'-'F'] $hex_digit* = HexIntegerLiteral,
        '0' ['0'-'7']+ = OctalIntegerLiteral,
        '0' ['o' 'O'] ['0'-'7']  ['0'-'7' '_']* = OctalIntegerStrictLiteral,
        '0' ['b' 'B'] ['0' '1'] ['0' '1' '_']* = BinaryIntegerLiteral,
        
        // what the hell is this, JS?
        // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt
        '0' ['x' 'X'] ['0'-'9' 'a'-'f' 'A'-'F'] $hex_digit* 'n' = BigHexIntegerLiteral,
        '0' ['o' 'O'] ['0'-'7']  ['0'-'7' '_']* 'n' = BigOctalIntegerLiteral,
        '0' ['b' 'B'] ['0' '1'] ['0' '1' '_']* 'b' = BinaryIntegerLiteral,
        $decimal_integer_literal 'n' = BigDecimalIntegerLiteral,

        '/' $regular_expression_first_char $regular_expression_char* '/' $identifier_part* = RegularExpressionLiteral,

        ('"' $double_string_char* '"') | ('\'' $single_string_char* '\'') = StringLiteral,

        // keywords
        "break" = BreakKeyword,
        "do" = DoKeyword,
        "instanceof" = InstanceofKeyword,
        "typeof" = TypeofKeyword,
        "case" = CaseKeyword,
        "else" = ElseKeyword,
        "new" = NewKeyword,
        "var" = VarKeyword,
        "catch" = CatchKeyword,
        "finally" = FinallyKeyword,
        "return" = ReturnKeyword,
        "void" = VoidKeyword,
        "continue" = ContinueKeyword,
        "for" = ForKeyword,
        "switch" = SwitchKeyword,
        "while" = WhileKeyword,
        "debugger" = DebuggerKeyword,
        "function" = FunctionKeyword,
        "this" = ThisKeyword,
        "with" = WithKeyword,
        "default" = DefaultKeyword,
        "if" = IfKeyword,
        "throw" = ThrowKeyword,
        "delete" = DeleteKeyword,
        "in" = InKeyword,
        "try" = TryKeyword,
        "as" = AsKeyword,
        "from" = FromKeyword,
        "of" = OfKeyword,
        "class" = ClassKeyword,
        "enum" = EnumKeyword,
        "extends" = ExtendsKeyword,
        "super" = SuperKeyword,
        "const" = ConstKeyword,
        "export" = ExportKeyword,
        "import" = ImportKeyword,
        "async" = AsyncKeyword,
        "await" = AwaitKeyword,
        "yield" = YieldKeyword,
        "implements" = ImplementsKeyword,
        "let" = LetKeyword,
        "private" = PrivateKeyword,
        "public" = PublicKeyword,
        "interface" = InterfaceKeyword,
        "package" = PackageKeyword,
        "protected" = ProtectedKeyword,
        "static" = StaticKeyword,

        // identifiers
        $identifier_start $identifier_part* = Identifier,

        // template strings (spicy)
        '`' => |lexer| {
            *lexer.state() += 1;
            lexer.switch_and_return(JsLexerRule::Template, TemplateStringStart)
        },
    }

    rule Template {
        '`' => |lexer| {
            *lexer.state() -= 1;
            lexer.switch_and_return(JsLexerRule::Init, TemplateStringEnd)
        },
        "${" => |lexer| lexer.switch_and_return(JsLexerRule::Init, TemplateStringExpressionStart),
        _,
    }

    rule MultiLineComment {
        "*/" => |lexer| lexer.switch_and_return(JsLexerRule::Init, MultiLineCommentEnd),
        _,
    }
}

#[derive(Debug, EnumCount, PartialEq, Eq, FromRepr)]
pub enum JsTokenKind {
    SingleLineComment,
    MultiLineCommentStart,
    MultiLineCommentEnd,
    LineTerminator,

    // symbols
    OpenBracketSymbol,
    CloseBracketSymbol,
    OpenParenSymbol,
    CloseParenSymbol,
    OpenBraceSymbol,
    TemplateCloseBraceSymbol,
    CloseBraceSymbol,
    SemiColonSymbol,
    CommaSymbol,
    AssignSymbol,
    QuestionMarkSymbol,
    QuestionMarkDotSymbol,
    ColonSymbol,
    EllipsisSymbol,
    DotSymbol,
    PlusPlusSymbol,
    MinusMinusSymbol,
    PlusSymbol,
    MinusSymbol,
    BitNotSymbol,
    NotSymbol,
    MultiplySymbol,
    DivideSymbol,
    ModulusSymbol,
    PowerSymbol,
    NullCoalesceSymbol,
    HashtagSymbol,
    RightShiftArithmeticSymbol,
    LeftShiftArithmeticSymbol,
    RightShiftLogicalSymbol,
    LessThanSymbol,
    MoreThanSymbol,
    LessThanEqualsSymbol,
    GreaterThanEqualsSymbol,
    EqualsSymbol,
    NotEqualsSymbol,
    IdentityEqualsSymbol,
    IdentityNotEqualsSymbol,
    BitAndSymbol,
    BitXOrSymbol,
    BitOrSymbol,
    AndSymbol,
    OrSymbol,
    MultiplyAssignSymbol,
    DivideAssignSymbol,
    ModulusAssignSymbol,
    PlusAssignSymbol,
    MinusAssignSymbol,
    LeftShiftArithmeticAssignSymbol,
    RightShiftArithmeticAssignSymbol,
    RightShiftLogicalAssignSymbol,
    BitAndAssignSymbol,
    BitXorAssignSymbol,
    BitOrAssignSymbol,
    PowerAssignSymbol,
    NullishCoalescingAssignSymbol,
    ArrowSymbol,

    // literals
    NullLiteral,
    BooleanLiteral,
    DecimalLiteral,
    HexIntegerLiteral,
    OctalIntegerLiteral,
    OctalIntegerStrictLiteral,
    BinaryIntegerLiteral,

    BigHexIntegerLiteral,
    BigOctalIntegerLiteral,
    BigBinaryIntegerLiteral,
    BigDecimalIntegerLiteral,

    RegularExpressionLiteral,
    StringLiteral,

    // keywords
    BreakKeyword,
    DoKeyword,
    InstanceofKeyword,
    TypeofKeyword,
    CaseKeyword,
    ElseKeyword,
    NewKeyword,
    VarKeyword,
    CatchKeyword,
    FinallyKeyword,
    ReturnKeyword,
    VoidKeyword,
    ContinueKeyword,
    ForKeyword,
    SwitchKeyword,
    WhileKeyword,
    DebuggerKeyword,
    FunctionKeyword,
    ThisKeyword,
    WithKeyword,
    DefaultKeyword,
    IfKeyword,
    ThrowKeyword,
    DeleteKeyword,
    InKeyword,
    TryKeyword,
    AsKeyword,
    FromKeyword,
    OfKeyword,
    ClassKeyword,
    EnumKeyword,
    ExtendsKeyword,
    SuperKeyword,
    ConstKeyword,
    ExportKeyword,
    ImportKeyword,
    AsyncKeyword,
    AwaitKeyword,
    YieldKeyword,
    ImplementsKeyword,
    LetKeyword,
    PrivateKeyword,
    PublicKeyword,
    InterfaceKeyword,
    PackageKeyword,
    ProtectedKeyword,
    StaticKeyword,

    // indentifiers
    Identifier,

    // template strings
    TemplateStringStart,
    TemplateStringExpressionStart,
    TemplateStringExpressionEnd,
    TemplateStringEnd,
}

impl From<JsTokenKind> for usize {
    fn from(token: JsTokenKind) -> Self {
        token as usize
    }
}