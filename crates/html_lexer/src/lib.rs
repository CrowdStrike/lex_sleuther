use lexgen::lexer;
use HtmlTokenKind::*;
use strum::{EnumCount, FromRepr};

/*
 * Based on the grammar declared here https://github.com/antlr/grammars-v4/blob/master/html/HTMLLexer.g4
 * Note that browsers are ridiculously tolerant in terms of errors in HTML syntax, missing quotes,
 * unclosed tags, and the like. This rule may run into edge cases that browsers handle fine.
 *
 * We're basically parsing HTML with Regex.
 */
lexer! {
    pub HtmlLexer -> HtmlTokenKind;

    let double_quote_string = '"' (_ # '"')* '"';
    let single_quote_string = '\'' (_ # '\'')* '\'';
    let attribute_characters = ['-' '_' '.' '/' '+' ',' '?' '=' ':' ';' '#' '0'-'9' 'a'-'z' 'A'-'Z']+ ' '?;
    let hex_attribute = '#' $$ascii_hexdigit+;
    let dec_attribute = $$ascii_digit+ '%'?;
    
    rule Init {
        // whitespace
        ' ' | '\t' | ('\r'? '\n')+,

        // alternative content directives we'll mostly swallow and ignore
        "<!--" => |lexer| lexer.switch(HtmlLexerRule::Comment),
        "<![" => |lexer| lexer.switch(HtmlLexerRule::ConditionalComment),
        "<?" => |lexer| lexer.switch(HtmlLexerRule::QuestionMarkScriptlet),
        "<%" => |lexer| lexer.switch(HtmlLexerRule::PercentScriptlet),
        "<?xml" (_ # '>')* ">" = Xml,
        "<!" (_ # '>')* ">" = Dtd,

        "<script" (_ # '>')* '>' => |lexer| lexer.switch_and_return(HtmlLexerRule::Script, ScriptOpen),
        "<style" (_ # '>')* '>' => |lexer| lexer.switch_and_return(HtmlLexerRule::Style, StyleOpen),

        "<" => |lexer| lexer.switch_and_return(HtmlLexerRule::Tag, TagOpen),

        // catch-all for everything thats not markup
        (_ # '<')+ = HtmlText,
    }

    rule Comment {
        "-->" => |lexer| lexer.switch_and_return(HtmlLexerRule::Init, Comment),
        _,
    }

    rule ConditionalComment {
        "]>" => |lexer| lexer.switch_and_return(HtmlLexerRule::Init, ConditionalComment),
        _,
    }

    rule QuestionMarkScriptlet {
        "?>" => |lexer| lexer.switch_and_return(HtmlLexerRule::Init, Scriptlet),
        _,
    }

    rule PercentScriptlet {
        "%>" => |lexer| lexer.switch_and_return(HtmlLexerRule::Init, Scriptlet),
        _,
    }

    rule Script {
        "</script>" | "</>" => |lexer| lexer.switch_and_return(HtmlLexerRule::Init, ScriptClose),
        _,
    }

    rule Style {
        "</style>" | "</>" => |lexer| lexer.switch_and_return(HtmlLexerRule::Init, StyleClose),
        _,
    }

    rule Tag {
        // whitespace
        [' ' '\t' '\r' '\n']+,
        ">" => |lexer| lexer.switch_and_return(HtmlLexerRule::Init, TagClose),
        "=" => |lexer| lexer.switch_and_return(HtmlLexerRule::Attribute, TagEquals),
        "/" = TagSlash,
        // this specification of valid HTML tag names is actually a slight oversimplification...
        $$ascii_alphabetic ($$ascii_alphanumeric | ['-' '_' '.'])* = TagName,

    }

    rule Attribute {
        // whitespace
        [' ' '\t' '\r' '\n']+,
        // many different valid forms for attribute values
        ' '* ($double_quote_string | $single_quote_string | $attribute_characters | $hex_attribute | $dec_attribute) =>
            |lexer| lexer.switch_and_return(HtmlLexerRule::Tag, AttributeValue),
    }
}

#[derive(Debug, EnumCount, FromRepr)]
pub enum HtmlTokenKind {
     Comment,
     ConditionalComment,
     Xml,
     // we never construct Cdata in this lexer because conditional comment will capture it
     Cdata,
     Dtd,
     Scriptlet,
     WhiteSpace,
     TagOpen,
     TagEquals,
     TagSlash,
     TagName,
     TagClose,
     AttributeValue,
     ScriptOpen,
     ScriptClose,
     StyleOpen,
     StyleClose,
     HtmlText
}

impl From<HtmlTokenKind> for usize {
    fn from(token: HtmlTokenKind) -> Self {
        token as usize
    }
}