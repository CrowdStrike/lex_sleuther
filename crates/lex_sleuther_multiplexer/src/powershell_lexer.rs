use lexgen::lexer;
use strum::{EnumCount, FromRepr};
use PowerShellTokenKind::*;

/*
 * There is no updated grammar specification for PowerShell 6, but the published reference for
 * PowerShell 3.0 was referenced heavily for this: https://learn.microsoft.com/en-us/powershell/scripting/lang-spec/chapter-02?view=powershell-7.2
 * 
 * Potentially, we could add certain common automatic / preference variables to get higher fidelity positives.
 *
 * I also referenced the syntax highlighting file for VSCode: https://github.com/zyedidia/micro/blob/master/runtime/syntax/PowerShell.yaml
 */
lexer! {
    pub PowerShellLexer -> PowerShellTokenKind;

    let whitespace = [' ' '\u{0009}' '\u{000B}' '\u{000C}']+ | "`\n";

    // this list will be incomplete and won't catch abbreviated cmdlets, but its still good to have
    let verb = "add" | "approve" | "assert" | "backup" | "block" | "build" | "checkpoint" | "clear" | "close" | "compare" | "complete" | "compress" | "confirm" | "connect" | "convert" | "convertfrom" | "convertto" | "copy" | "debug" | "deny" | "deploy" | "disable" | "disconnect" | "dismount" | "edit" | "enable" | "enter" | "exit" | "expand" | "export" | "find" | "format" | "get" | "grant" | "group" | "hide" | "import" | "initialize" | "install" | "invoke" | "join" | "limit" | "lock" | "measure" | "merge" | "mount" | "move" | "new" | "open" | "optimize" | "out" | "ping" | "pop" | "protect" | "publish" | "push" | "read" | "receive" | "redo" | "register" | "remove" | "rename" | "repair" | "request" | "reset" | "resize" | "resolve" | "restart" | "restore" | "resume" | "revoke" | "save" | "search" | "select" | "send" | "set" | "show" | "skip" | "split" | "start" | "step" | "stop" | "submit" | "suspend" | "switch" | "sync" | "test" | "trace" | "unblock" | "undo" | "uninstall" | "unlock" | "unprotect" | "unpublish" | "unregister" | "update" | "use" | "wait" | "watch" | "write";

    // technically, any character can be escaped, but this will capture the common cases
    let escaped_char = '`' ['a' 'b' 'f' 'n' 'r' 't' 'v' '\'' '"' '`' '0'];

    let variable_chars = $$alphabetic | '_' | '?';
    let variable_scope = $variable_chars+ ":";
    let basic_variable = ['$' '@'] $variable_scope? $variable_chars+;
    let braced_variable = "${" $variable_scope? ($variable_chars # '}')+ "}";

    let type_char = $$alphabetic | '_';

    let generic_token_char = (_ # ['{' '}' '(' ')' ';' ',' '|' '&' '$' '`' '"' '\'' ' ' '\n']);
    let parameter_char = (_ # ['{' '}' '(' ')' ';' ',' '|' '&' '.' '[' ':' ' ' '\n']);
    let first_parameter_char = $$alphabetic | '_' | '?';

    let decimal_type_suffix = ['d' 'l'];
    let decimal_digits = ['0'-'9']+;
    let hexidecimal_digits = ['0'-'9' 'a'-'f']+;
    let numeric_multiplier = "kb" | "mb" | "gb" | "tb" | "pb";
    let dash = ['-' '\u{2013}' '\u{2014}' '\u{2015}'];
    let real_exponent = 'e' ('+' | $dash)? $decimal_digits;

    rule Init {
        // whitespace (ignored)
        $whitespace,
        '\r'? '\n' = NewLine,

        // comments
        "#" (_ # '\n')* = LineComment,
        "<#" => |lexer| lexer.switch_and_return(PowerShellLexerRule::DelimitedComment, DelimitedCommentStart),

        // keywords
        "alias" = AliasKeyword,
        "as" = AsKeyword,
        "begin" = BeginKeyword,
        "break" = BreakKeyword,
        "catch" = CatchKeyword,
        "class" = ClassKeyword,
        "continue" = ContinueKeyword,
        "data" = DataKeyword,
        "default" = DefaultKeyword,
        "define" = DefineKeyword,
        "do" = DoKeyword,
        "dynamicparam" = DynamicParamKeyword,
        "else" = ElseKeyword,
        "elseif" = ElseIfKeyword,
        "end" = EndKeyword,
        "exit" = ExitKeyword,
        "filter" = FilterKeyword,
        "finally" = FinallyKeyword,
        "for" = ForKeyword,
        "foreach" = ForeachKeyword,
        "foreach-object" = ForeachObjectKeyword,
        "from" = FromKeyword,
        "function" = FunctionKeyword,
        "if" = IfKeyword,
        "in" = InKeyword,
        "inlinescript" = InlineScriptKeyword,
        "parallel" = ParallelKeyword,
        "param" = ParamKeyword,
        "process" = ProcessKeyword,
        "return" = ReturnKeyword,
        "switch" = SwitchKeyword,
        "throw" = ThrowKeyword,
        "trap" = TrapKeyword,
        "try" = TryKeyword,
        "until" = UntilKeyword,
        "using" = UsingKeyword,
        "var" = VarKeyword,
        "where" = WhereKeyword,
        "where-object" = WhereObjectKeyword,
        "while" = WhileKeyword,
        "workflow" = WorkflowKeyword,

        // comparison operators
        "-as" = AsComparisonOperator,
        "-ccontains" = CContainsComparisonOperator,
        "-ceq" = CEqComparisonOperator,
        "-cge" = CGeComparisonOperator,
        "-cgt" = CGtComparisonOperator,
        "-cle" = CLeComparisonOperator,
        "-clike" = CLikeComparisonOperator,
        "-clt" = CLtComparisonOperator,
        "-cmatch" = CMatchComparisonOperator,
        "-cne" = CNeComparisonOperator,
        "-cnotcontains" = CNotContainsComparisonOperator,
        "-cnotlike" = CNotLikeComparisonOperator,
        "-cnotmatch" = CNotMatchComparisonOperator,
        "-contains" = ContainsComparisonOperator,
        "-creplace" = CReplaceComparisonOperator,
        "-csplit" = CSplitComparisonOperator,
        "-eq" = EqComparisonOperator,
        "-ge" = GeComparisonOperator,
        "-gt" = GtComparisonOperator,
        "-icontains" = IContainsComparisonOperator,
        "-ieq" = IEqComparisonOperator,
        "-ige" = IGeComparisonOperator,
        "-igt" = IGtComparisonOperator,
        "-ile" = ILeComparisonOperator,
        "-ilike" = ILikeComparisonOperator,
        "-ilt" = ILtComparisonOperator,
        "-imatch" = IMatchComparisonOperator,
        "-in" = InComparisonOperator,
        "-ine" = INeComparisonOperator,
        "-inotcontains" = INotContainsComparisonOperator,
        "-inotlike" = INotLikeComparisonOperator,
        "-inotmatch" = INotMatchComparisonOperator,
        "-ireplace" = IReplaceComparisonOperator,
        "-is" = IsComparisonOperator,
        "-isnot" = IsNotComparisonOperator,
        "-isplit" = ISplitComparisonOperator,
        "-join" = JoinComparisonOperator,
        "-le" = LeComparisonOperator,
        "-like" = LikeComparisonOperator,
        "-lt" = LtComparisonOperator,
        "-match" = MatchComparisonOperator,
        "-ne" = NeComparisonOperator,
        "-notcontains" = NotContainsComparisonOperator,
        "-notin" = NotInComparisonOperator,
        "-notlike" = NotLikeComparisonOperator,
        "-notmatch" = NotMatchComparisonOperator,
        "-replace" = ReplaceComparisonOperator,
        "-shl*" = ShlComparisonOperator,
        "-shr" = ShrComparisonOperator,
        "-split" = SplitComparisonOperator,
        "-f" = FormatOperator,
        "-and" = AndComparisonOperator,
        "-band" = BinaryAndComparisonOperator,
        "-bnot" = BinaryNotComparisonOperator,
        "-bor" = BinaryOrComparisonOperator,
        "-bxor" = BinaryXorComparisonOperator,
        "-not" = NotComparisonOperator,
        "-or" = OrComparisonOperator,
        "-xor" = XorComparisonOperator,

        // constants
        "$true" = TrueConstant,
        "$false" = FalseConstant,
        "$null" = NullConstant,

        // literals
        $decimal_digits $decimal_type_suffix? $numeric_multiplier? = DecimalLiteral,
        "0x" $hexidecimal_digits 'l'? $numeric_multiplier? = HexidecimalLiteral,
        ($decimal_digits? '.' $decimal_digits $real_exponent? | $decimal_digits $real_exponent) $decimal_type_suffix $numeric_multiplier? = RealLiteral,

        // strings
        '\'' => |lexer| lexer.switch_and_return(PowerShellLexerRule::StringVerbatim, StringVerbatimLiteralStart),
        '"' => |lexer| lexer.switch_and_return(PowerShellLexerRule::StringExpandable, StringExpandableLiteralStart),
        "@'" $whitespace* ['\n' '\r']+ =>  |lexer| lexer.switch_and_return(PowerShellLexerRule::StringVerbatimHere, StringVerbatimHereLiteralStart),
        "@\"" $whitespace* ['\n' '\r']+ => |lexer| lexer.switch_and_return(PowerShellLexerRule::StringExpandableHere, StringExpandableHereLiteralStart),

        // variables
        $basic_variable = BasicVariable,
        $braced_variable = BracedVariable,
        "$$" | "$?" | "$^" = ReservedVariable,
        ($type_char+ '.')* $type_char+ = TypeName,

        // commands and parameters
        $verb $dash $type_char+ = RecognizedCommand,
        $dash $first_parameter_char $parameter_char* ':'? = CommandParameter,

        // symbols
        "(" = OpenParenSymbol,
        ")" = CloseParenSymbol,
        "[" = OpenBrackSymbol,
        "]" = CloseBrackSymbol,
        "{" = OpenBraceSymbol,
        "}" = CloseBraceSymbol,
        "@(" = OpenParenExpressionSymbol,
        "@{" = OpenBraceExpressionSymbol,
        "$(" = OpenParenSubstitutionSymbol,
        ";" = SemicolonSymbol,
        "&&" = PipelineAndChainSymbol,
        "||" = PipelineOrChainSymbol,
        "&" = BitAndSymbol,
        "|" = BitOrSymbol,
        "," = CommaSymbol,
        "++" = IncrementSymbol,
        ".." = RangeSymbol,
        "::" = NamespaceSymbol,
        "." = DotSymbol,
        "!" = NotSymbol,
        "*" = MultiplySymbol,
        "/" = DivideSymbol,
        "%" = ModulusSymbol,
        "+" = AddSymbol,
        "-" = SubtractSymbol,
        "--" = DecrementSymbol,

        // assignment operators
        "=" = AssignmentOperator,
        "-=" = SubtractAssignmentOperator,
        "+=" = AddAssignmentOperator,
        "*=" = MultiplyAssignmentOperator,
        "/=" = DivideAssignmentOperator,
        "%=" = ModulusAssignmentOperator,

        // redirection operators
        ['1' '2' '3' '4' '5' '6' '*']? '>' ('>' | '&' ['1' '2'])? | '<' = RedirectionOperator,

        // unfortunately, because this is a shell interpretted lanuage,
        // this matches as a catch all for basically any other characters that might be passed as command input
        _ = GenericToken,
    }

    rule DelimitedComment {
        "#>" => |lexer| lexer.switch_and_return(PowerShellLexerRule::Init, DelimitedCommentEnd),
        _,
    }

    rule StringVerbatim {
        '\'' > (_ # '\'') => |lexer| lexer.switch_and_return(PowerShellLexerRule::Init, StringVerbatimLiteralEnd),
        _,
    }

    rule StringExpandable {
        '"' > (_ # '"') => |lexer| lexer.switch_and_return(PowerShellLexerRule::Init, StringExpandableLiteralEnd),
        $escaped_char = KnownEscapedChar,
        $basic_variable = BasicVariable,
        $braced_variable = BracedVariable,
        _,
    }

    rule StringVerbatimHere {
        ['\n' '\r']+ "'@" => |lexer| lexer.switch_and_return(PowerShellLexerRule::Init, StringVerbatimLiteralEnd),
        _,
    }

    rule StringExpandableHere {
        ['\n' '\r']+ "\"@" => |lexer| lexer.switch_and_return(PowerShellLexerRule::Init, StringVerbatimLiteralEnd),
        $escaped_char = KnownEscapedChar,
        $basic_variable = BasicVariable,
        $braced_variable = BracedVariable,
        _,
    }
}

#[derive(Debug, EnumCount, PartialEq, Eq, FromRepr)]
pub enum PowerShellTokenKind {
    // comments and whitespace
    LineComment,
    DelimitedCommentStart,
    DelimitedCommentEnd,
    NewLine,
    // keywords
    AliasKeyword,
    AsKeyword,
    BeginKeyword,
    BreakKeyword,
    CatchKeyword,
    ClassKeyword,
    ContinueKeyword,
    DataKeyword,
    DefaultKeyword,
    DefineKeyword,
    DoKeyword,
    DynamicParamKeyword,
    ElseKeyword,
    ElseIfKeyword,
    EndKeyword,
    ExitKeyword,
    FilterKeyword,
    FinallyKeyword,
    ForKeyword,
    ForeachKeyword,
    ForeachObjectKeyword,
    FromKeyword,
    FunctionKeyword,
    IfKeyword,
    InKeyword,
    InlineScriptKeyword,
    ParallelKeyword,
    ParamKeyword,
    ProcessKeyword,
    ReturnKeyword,
    SwitchKeyword,
    ThrowKeyword,
    TrapKeyword,
    TryKeyword,
    UntilKeyword,
    UsingKeyword,
    VarKeyword,
    WhereKeyword,
    WhereObjectKeyword,
    WhileKeyword,
    WorkflowKeyword,
    // comparison operators
    AsComparisonOperator,
    CContainsComparisonOperator,
    CEqComparisonOperator,
    CGeComparisonOperator,
    CGtComparisonOperator,
    CLeComparisonOperator,
    CLikeComparisonOperator,
    CLtComparisonOperator,
    CMatchComparisonOperator,
    CNeComparisonOperator,
    CNotContainsComparisonOperator,
    CNotLikeComparisonOperator,
    CNotMatchComparisonOperator,
    ContainsComparisonOperator,
    CReplaceComparisonOperator,
    CSplitComparisonOperator,
    EqComparisonOperator,
    GeComparisonOperator,
    GtComparisonOperator,
    IContainsComparisonOperator,
    IEqComparisonOperator,
    IGeComparisonOperator,
    IGtComparisonOperator,
    ILeComparisonOperator,
    ILikeComparisonOperator,
    ILtComparisonOperator,
    IMatchComparisonOperator,
    InComparisonOperator,
    INeComparisonOperator,
    INotContainsComparisonOperator,
    INotLikeComparisonOperator,
    INotMatchComparisonOperator,
    IReplaceComparisonOperator,
    IsComparisonOperator,
    IsNotComparisonOperator,
    ISplitComparisonOperator,
    JoinComparisonOperator,
    LeComparisonOperator,
    LikeComparisonOperator,
    LtComparisonOperator,
    MatchComparisonOperator,
    NeComparisonOperator,
    NotContainsComparisonOperator,
    NotInComparisonOperator,
    NotLikeComparisonOperator,
    NotMatchComparisonOperator,
    ReplaceComparisonOperator,
    ShlComparisonOperator,
    ShrComparisonOperator,
    SplitComparisonOperator,
    FormatOperator,
    AndComparisonOperator,
    BinaryAndComparisonOperator,
    BinaryNotComparisonOperator,
    BinaryOrComparisonOperator,
    BinaryXorComparisonOperator,
    NotComparisonOperator,
    OrComparisonOperator,
    XorComparisonOperator,

    // constants
    TrueConstant,
    FalseConstant,
    NullConstant,
    // literals
    DecimalLiteral,
    HexidecimalLiteral,
    RealLiteral,
    // strings
    StringVerbatimLiteralStart,
    StringVerbatimLiteralEnd,
    StringVerbatimHereLiteralStart,
    StringVerbatimHereLiteralEnd,
    StringExpandableLiteralStart,
    StringExpandableLiteralEnd,
    StringExpandableHereLiteralStart,
    StringExpandableHereLiteralEnd,
    KnownEscapedChar,

    // variables
    BasicVariable,
    BracedVariable,
    ReservedVariable,
    TypeName,

    // commands and parameters
    RecognizedCommand,
    CommandParameter,
    GenericToken,

    // symbols
    OpenBraceSymbol,
    OpenParenSymbol,
    OpenBrackSymbol,
    CloseBraceSymbol,
    CloseParenSymbol,
    CloseBrackSymbol,
    OpenParenExpressionSymbol,
    OpenBraceExpressionSymbol,
    OpenParenSubstitutionSymbol,
    SemicolonSymbol,
    PipelineAndChainSymbol,
    PipelineOrChainSymbol,
    BitAndSymbol,
    BitOrSymbol,
    CommaSymbol,
    IncrementSymbol,
    RangeSymbol,
    NamespaceSymbol,
    DotSymbol,
    NotSymbol,
    MultiplySymbol,
    DivideSymbol,
    ModulusSymbol,
    AddSymbol,
    SubtractSymbol,
    DecrementSymbol,
    // assignment operators
    AssignmentOperator,
    SubtractAssignmentOperator,
    AddAssignmentOperator,
    MultiplyAssignmentOperator,
    DivideAssignmentOperator,
    ModulusAssignmentOperator,
    // redirection
    RedirectionOperator,
}

impl From<PowerShellTokenKind> for usize {
    fn from(token: PowerShellTokenKind) -> Self {
        token as usize
    }
}