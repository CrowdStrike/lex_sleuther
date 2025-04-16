
use lexgen::lexer;
use strum::{EnumCount, FromRepr};
use BatchTokenKind::*;

/*
 * There is no offical grammar for Windows Batch Scripting since its a Perl 1.0-style back-of-the-envelope shell interpreter.
 * But, the specification here was my best resource: https://en.wikibooks.org/wiki/Windows_Batch_Scripting#Syntax
 */
lexer! {
    pub BatchLexer -> BatchTokenKind;

    let var_name = ($$ascii_alphanumeric | ['_'])+;
    let glob_pattern = ($$ascii_alphanumeric | ['*'])+;

    // my best attempt at a Windows filepath regex...
    // its not perfect but will successfully swallow most of most obvious paths
    let slash = ['\\' '/'];
    let file_path = ($$ascii_alphabetic ':' | '\\' | '.' '.'+) ($$alphanumeric | $slash | ['.' '*' '?'])*;

    rule Init {
        // whitespace
        [' ' '\t']+,
        '\n' = NewLine,

        // comments
        ("rem" | "::") (_ # ['\n' '\r'])* = Comment,

        // variables
        '%' $var_name '%' = NamedVariable,
        '%' $$ascii_digit = ParameterVariable,
        // unclear if this should be double or single escaped so...
        '%' '%'? $var_name = LoopVariable,
        "%~" (['f' 'd' 'p' 'n' 'x' 's' 'a' 't' 'z']* | "$path:") $$ascii_digit = FilePathVariable,
        "%*" = SplatVariable,
        ':' $var_name = Label,

        // escaped characters
        '^' $$ascii_alphabetic = EscapedCharacter,
        // we have to make sure this isn't a loop variable...
        "%%" > (_ # ('~' | $$ascii_alphanumeric)) = EscapedPercent,
        // yes, this is a real thing
        "^^^" = TripleEscape,

        // conjunctions
        '&' = UnconditionalConjunction,
        "&&" = PositiveConditionalConjunction,
        "||" = NegativeConditionalConjunction,

        // symbols
        '(' = OpenParenSymbol,
        ')' = CloseParenSymbol,
        '@' = AtSymbol,
        '\'' = SingleQuoteSymbol,
        '"' = DoubleQuoteSymbol,
        "==" = EqualsSymbol,
        "=" = AssignSymbol,

        // redirections
        '<' = FileToStdioRedirect,
        '>' = StdioToFileOverwriteRedirect,
        ">>" = StdioToFileAppendRedirect,
        ">&" $$ascii_digit = ToHandleRedirect,
        "<&" $$ascii_digit = FromHandleRedirect,

        // switches
        '/' ($$ascii_alphanumeric | '?')+ = WindowsSwitch,
        '-' $$ascii_alphanumeric+ = UnixSwitch,
        "--" $$ascii_alphanumeric+ = GnuSwitch,

        // constants
        "nul" = NullHandle,

        // string processing
        '%' $var_name ":~" ('-'? $$ascii_digit+ ',')? '-'? $$ascii_digit+ '%' = VariableSubstring,
        '%' $var_name ':' '*'? $$ascii_alphanumeric '=' $$ascii_alphanumeric* = VariableStringReplacement,
        
        // wildcards
        '*' = SequenceWildcard,
        '?' = SingleCharacterWildcard,

        // try to at least capture file paths
        $file_path = FilePath,

        // built-in commands
        "assoc" = AssocCommand,
        "break" = BreakCommand,
        "call" = CallCommand,
        "cd" = CdCommand,
        "chdir" = ChdirCommand,
        "chcp" = ChcpCommand,
        "cls" = ClsCommand,
        "color" = ColorCommand,
        "copy" = CopyCommand,
        "date" = DateCommand,
        "del" = DelCommand,
        "erase" = EraseCommand,
        "dir" = DirCommand,
        "echo" = EchoCommand,
        "else" = ElseCommand,
        "endlocal" = EndlocalCommand,
        "exit" = ExitCommand,
        "for" = ForCommand,
        "ftype" = FtypeCommand,
        "goto" = GotoCommand,
        "if" = IfCommand,
        "md" = MdCommand,
        "mkdir" = MkdirCommand,
        "move" = MoveCommand,
        "path" = PathCommand,
        "pause" = PauseCommand,
        "popd" = PopdCommand,
        "prompt" = PromptCommand,
        "pushd" = PushdCommand,
        "rd" = RdCommand,
        "rmdir" = RmdirCommand,
        "rem" = RemCommand,
        "ren" = RenCommand,
        "rename" = RenameCommand,
        "set" = SetCommand,
        "setlocal" = SetlocalCommand,
        "shift" = ShiftCommand,
        "start" = StartCommand,
        "time" = TimeCommand,
        "title" = TitleCommand,
        "type" = TypeCommand,
        "ver" = VerCommand,
        "verify" = VerifyCommand,
        "vol" = VolCommand,

        // external commands
        "arp" = ArpCommand,
        "at" = AtCommand,
        "attrib" = AttribCommand,
        "bcdedit" = BcdeditCommand,
        "cacls" = CaclsCommand,
        "chkdsk" = ChkdskCommand,
        "chkntfs" = ChkntfsCommand,
        "choice" = ChoiceCommand,
        "cipher" = CipherCommand,
        "clip" = ClipCommand,
        "cmd" = CmdCommand,
        "comp" = CompCommand,
        "compact" = CompactCommand,
        "convert" = ConvertCommand,
        "debug" = DebugCommand,
        "diskcomp" = DiskcompCommand,
        "diskcopy" = DiskcopyCommand,
        "diskpart" = DiskpartCommand,
        "doskey" = DoskeyCommand,
        "driverquery" = DriverqueryCommand,
        "expand" = ExpandCommand,
        "fc" = FcCommand,
        "find" = FindCommand,
        "findstr" = FindstrCommand,
        "forfiles" = ForfilesCommand,
        "format" = FormatCommand,
        "fsutil" = FsutilCommand,
        "gpresult" = GpresultCommand,
        "graftabl" = GraftablCommand,
        "help" = HelpCommand,
        "icacls" = IcaclsCommand,
        "ipconfig" = IpconfigCommand,
        "label" = LabelCommand,
        "makecab" = MakecabCommand,
        "mode" = ModeCommand,
        "more" = MoreCommand,
        "net" = NetCommand,
        "openfiles" = OpenfilesCommand,
        "ping" = PingCommand,
        "recover" = RecoverCommand,
        "reg" = RegCommand,
        "replace" = ReplaceCommand,
        "robocopy" = RobocopyCommand,
        "rundll32" = Rundll32Command,
        "sc" = ScCommand,
        "schtasks" = SchtasksCommand,
        "setx" = SetxCommand,
        "shutdown" = ShutdownCommand,
        "sort" = SortCommand,
        "subst" = SubstCommand,
        "systeminfo" = SysteminfoCommand,
        "taskkill" = TaskkillCommand,
        "tasklist" = TasklistCommand,
        "timeout" = TimeoutCommand,
        "tree" = TreeCommand,
        "where" = WhereCommand,
        "wmic" = WmicCommand,
        "xcopy" = XcopyCommand,

        // catch-alls for likely meaningful groups of characters
        $$ascii_digit+ = NumberLiteral,
        $var_name = StringLiteral,

        // unfortunately, because this is a shell interpretted lanuage,
        // this matches as a catch all for basically any other characters that might be passed as command input
        _,
    }
}


#[derive(Debug, EnumCount, PartialEq, Eq, FromRepr)]
pub enum BatchTokenKind {
    // whitespace / comments
    NewLine,
    Comment,

    // variables
    NamedVariable,
    ParameterVariable,
    LoopVariable,
    FilePathVariable,
    SplatVariable,
    Label,

    // TODO: special built-in variables

    // escaped characters
    EscapedCharacter,
    EscapedPercent,
    TripleEscape,

    // conjunctions
    UnconditionalConjunction,
    PositiveConditionalConjunction,
    NegativeConditionalConjunction,

    // symbols
    OpenParenSymbol,
    CloseParenSymbol,
    AtSymbol,
    SingleQuoteSymbol,
    DoubleQuoteSymbol,
    EqualsSymbol,
    AssignSymbol,

    // redirections
    FileToStdioRedirect,
    StdioToFileOverwriteRedirect,
    StdioToFileAppendRedirect,
    ToHandleRedirect,
    FromHandleRedirect,

    // TODO: known-environment variables

    // command switches
    WindowsSwitch,
    UnixSwitch,
    GnuSwitch,

    // constants
    NullHandle,

    // string processing
    VariableSubstring,
    VariableStringReplacement,

    // wildcards
    SequenceWildcard,
    SingleCharacterWildcard,

    // typical-looking windows filepaths
    FilePath,

    // built-in commands
    AssocCommand,
    BreakCommand,
    CallCommand,
    CdCommand,
    ChdirCommand,
    ChcpCommand,
    ClsCommand,
    ColorCommand,
    CopyCommand,
    DateCommand,
    DelCommand,
    EraseCommand,
    DirCommand,
    EchoCommand,
    ElseCommand,
    EndlocalCommand,
    ExitCommand,
    ForCommand,
    FtypeCommand,
    GotoCommand,
    IfCommand,
    MdCommand,
    MkdirCommand,
    MoveCommand,
    PathCommand,
    PauseCommand,
    PopdCommand,
    PromptCommand,
    PushdCommand,
    RdCommand,
    RmdirCommand,
    RemCommand,
    RenCommand,
    RenameCommand,
    SetCommand,
    SetlocalCommand,
    ShiftCommand,
    StartCommand,
    TimeCommand,
    TitleCommand,
    TypeCommand,
    VerCommand,
    VerifyCommand,
    VolCommand,

    // external commands
    ArpCommand,
    AtCommand,
    AttribCommand,
    BcdeditCommand,
    CaclsCommand,
    ChkdskCommand,
    ChkntfsCommand,
    ChoiceCommand,
    CipherCommand,
    ClipCommand,
    CmdCommand,
    CompCommand,
    CompactCommand,
    ConvertCommand,
    DebugCommand,
    DiskcompCommand,
    DiskcopyCommand,
    DiskpartCommand,
    DoskeyCommand,
    DriverqueryCommand,
    ExpandCommand,
    FcCommand,
    FindCommand,
    FindstrCommand,
    ForfilesCommand,
    FormatCommand,
    FsutilCommand,
    GpresultCommand,
    GraftablCommand,
    HelpCommand,
    IcaclsCommand,
    IpconfigCommand,
    LabelCommand,
    MakecabCommand,
    ModeCommand,
    MoreCommand,
    NetCommand,
    OpenfilesCommand,
    PingCommand,
    RecoverCommand,
    RegCommand,
    ReplaceCommand,
    RobocopyCommand,
    Rundll32Command,
    ScCommand,
    SchtasksCommand,
    SetxCommand,
    ShutdownCommand,
    SortCommand,
    SubstCommand,
    SysteminfoCommand,
    TaskkillCommand,
    TasklistCommand,
    TimeoutCommand,
    TreeCommand,
    WhereCommand,
    WmicCommand,
    XcopyCommand,

    // catch-all values
    StringLiteral,
    NumberLiteral,
}

impl From<BatchTokenKind> for usize {
    fn from(token: BatchTokenKind) -> Self {
        token as usize
    }
}