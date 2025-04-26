use std::io::BufRead;
use utf8_chars::BufReadCharsExt;

/// comparatively simple scanner over bytes that are presumedly already in memory
#[derive(Clone)]
pub(crate) struct SliceScanner<T: BufRead>(pub(crate) T);

impl<T: BufRead> Iterator for SliceScanner<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        // discard utf-8 errors as we encounter them
        self.0.chars().filter_map(Result::ok).next()
    }
}
