use lexgen_util::{LexerError, Loc};

/// The primary role of this accumulator is to keep track of how many occurances of
/// each token we encounter in the lexer for vectorization later.
///
/// The secondary goal is to take in `lexgen` generated lexers and make them homogenous
/// and easy to work with in a Vec.
pub(crate) struct LexerResultAccumulator<'a> {
    result: LexerResult,
    inner_lexer: Box<dyn Iterator<Item = TokenLexResult> + 'a>,
}

impl<'a> LexerResultAccumulator<'a> {
    pub(crate) fn new<I, T>(lexer: I, initial_size: usize) -> Self
    where
        I: Iterator<Item = T> + 'a,
        T: Into<TokenLexResult> + 'a,
    {
        Self {
            result: LexerResult {
                token_counts: vec![0; initial_size],
                error_count: 0,
            },
            // if we don't map the result type, our iterator will have generics and will therefore
            // no longer be a safe trait-object for dynamic dispatch
            inner_lexer: Box::new(lexer.map(Into::into)),
        }
    }

    pub(crate) fn into_result(self) -> LexerResult {
        self.result
    }
}

/// A caller of this iterator is only interested in the position of the lexer in
/// the underlying character stream, so that is what we emit.
/// Interally, the `token_counts` and `error_count` are incremented as we go.
impl Iterator for LexerResultAccumulator<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner_lexer.next() {
            Some(TokenLexResult::Token {
                token_idx,
                end_byte_idx,
            }) => {
                self.result.token_counts[token_idx] += 1;
                Some(end_byte_idx)
            }
            Some(TokenLexResult::Error { byte_idx }) => {
                self.result.error_count += 1;
                Some(byte_idx)
            }
            None => None,
        }
    }
}

/// the result of lexing until a single token or error is emitted
pub(crate) enum TokenLexResult {
    Token {
        /// the index of the encountered token in the `token_counts` array
        token_idx: usize,
        /// the current byte position of the lexer in the underlying character stream
        end_byte_idx: usize,
    },
    Error {
        /// the current byte position of the lexer in the underlying character stream
        byte_idx: usize,
    },
}

/// maps the results from lexgen and homogenizes them into a structure without generics
impl<T: Into<usize>, E> From<Result<(Loc, T, Loc), LexerError<E>>> for TokenLexResult {
    fn from(res: Result<(Loc, T, Loc), LexerError<E>>) -> Self {
        match res {
            Ok((_, token, end)) => TokenLexResult::Token {
                token_idx: token.into(),
                end_byte_idx: end.byte_idx,
            },
            Err(err) => TokenLexResult::Error {
                byte_idx: err.location.byte_idx,
            },
        }
    }
}

pub struct LexerResult {
    pub token_counts: Vec<u64>,
    pub error_count: u64,
}

impl LexerResult {
    /// sum up the total number of tokens lexed
    pub fn total_token_count(&self) -> u64 {
        self.token_counts.iter().sum()
    }

    /// produce values for a token count feature vector, including errors
    pub fn into_count_vector_iter(self) -> impl Iterator<Item = u64> {
        self.token_counts
            .into_iter()
            .chain(std::iter::once(self.error_count))
    }

    /// produce values for a token frequency feature vector, including errors
    pub fn into_frequency_vector_iter(self) -> impl Iterator<Item = f64> {
        let total_count = self.total_token_count();
        // u64 -> f64 is lossy but only for the lower bits of truly huge values
        self.into_count_vector_iter()
            .map(move |count| count as f64 / total_count as f64)
    }
}
