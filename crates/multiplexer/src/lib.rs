use std::fs;
use std::io::Cursor;
use std::{
    io::{self, BufReader},
    path::PathBuf,
};

use log::trace;

use batch_lexer::{BatchLexer, BatchTokenKind};
use html_lexer::{HtmlLexer, HtmlTokenKind};
use js_lexer::{JsLexer, JsTokenKind};
use lexer::LexerResultAccumulator;
use powershell_lexer::{PowerShellLexer, PowerShellTokenKind};
use python_lexer::{PythonLexer, PythonTokenKind};
use strum::EnumCount;
use vb6_lexer::{Vb6Lexer, Vb6TokenKind};

// reexport for ergonomics
pub use lexer::LexerResult;

mod lexer;
mod scanner;

/// Given an Iterator over Unicode `char`'s, computes the feature vector derived from our hoard of lexers.
/// This is the meat and potatoes of this library.
///
/// Note that BufReader does NOT implement `Clone`, which is a restriction placed on us by
/// `lexgen`'s API. If you need to lex a file, use `vector_from_file()` instead.
pub fn lexer_results_from_scanner<I: Iterator<Item = char> + Clone>(
    scanner: I,
) -> Vec<LexerResult> {
    // note lexgen doesn't have support for case insensitivity, which microsoft loves in their langs
    // so if you want to actually match keywords properly, you'll need to automatically lowercase input
    let lowercase_scanner = scanner.clone().flat_map(char::to_lowercase);

    // IMPORTANT: This is the main array where you will need to add new lexers as you create them in order for them to be used.
    // Remember to make sure that the number of Scanner clones MATCHES the number of lexers. Notice the last two lexer entries do
    // NOT clone the lexer because we need to consume EVERY copy of Scanner to efficiently manage memory.
    let mut lexers = [
        LexerResultAccumulator::new(
            HtmlLexer::new_from_iter(lowercase_scanner.clone()),
            HtmlTokenKind::COUNT,
        ),
        LexerResultAccumulator::new(
            Vb6Lexer::new_from_iter(lowercase_scanner.clone()),
            Vb6TokenKind::COUNT,
        ),
        LexerResultAccumulator::new(
            JsLexer::new_from_iter_with_state(scanner.clone(), 0),
            JsTokenKind::COUNT,
        ),
        LexerResultAccumulator::new(
            PowerShellLexer::new_from_iter(lowercase_scanner.clone()),
            PowerShellTokenKind::COUNT,
        ),
        LexerResultAccumulator::new(PythonLexer::new_from_iter(scanner), PythonTokenKind::COUNT),
        LexerResultAccumulator::new(
            BatchLexer::new_from_iter(lowercase_scanner),
            BatchTokenKind::COUNT,
        ),
    ];

    let mut tasks: Vec<_> = lexers.iter_mut().collect();

    // Sort-of unhinged algorithm
    // This basically alternates between each lexer to keep them roughly in sync.
    // Doing this will allow Scanner to discard memory that will no longer be used as we go.
    let mut max_byte_idx = 0;

    while !tasks.is_empty() {
        tasks.retain_mut(|task| {
            for end_byte_idx in task.by_ref() {
                // this lexer is the furthest advanced, so retain it, but move on to the next one
                if end_byte_idx >= max_byte_idx {
                    trace!("lexer switch: {} >= {}", end_byte_idx, max_byte_idx);
                    max_byte_idx = end_byte_idx;
                    return true;
                }
            }
            // this lexer has hit the end of input, so remove it from the task list
            false
        })
    }

    lexers
        .into_iter()
        .map(LexerResultAccumulator::into_result)
        .collect()
}

/// Given a file path, scan over the file and count tokens and errors from our hoard of lexers.
pub fn lexer_results_from_file(filepath: &PathBuf) -> Result<Vec<LexerResult>, io::Error> {
    let file = fs::File::open(filepath)?;
    let reader = BufReader::new(file);
    let scanner = scanner::StreamScanner::new(reader);
    Ok(lexer_results_from_scanner(scanner))
}

/// Given a vector of bytes that is already in memory, compute the feature vector derived from our hoard of lexers.
pub fn lexer_results_from_bytes(bytes: &Vec<u8>) -> Vec<LexerResult> {
    let cursor = Cursor::new(bytes);
    let chars_iter = scanner::SliceScanner(cursor);
    lexer_results_from_scanner(chars_iter)
}
