use bytesize::ByteSize;
use log::{debug, trace};
use std::{cell::RefCell, collections::VecDeque, io::BufRead, rc::Rc};
use utf8_chars::BufReadCharsExt;

/// This whole file is basically a hack to get proper streaming support with lexgen.
/// We have to create our own scanner that is both iterable, cloneable, and transparently skips non utf-8 characters.
/// We've also thrown in some optimizations to allow a single, small shared buffer to be used between all scanners.
pub(crate) struct StreamScanner<R: BufRead> {
    /// the number of characters this scanner has yielded via next()
    character_position: usize,
    /// shared state between all clones of Scanner
    scanner_state: Rc<RefCell<ScannerState<R>>>,
}

struct ScannerState<R: BufRead> {
    inner_reader: R,
    /// this is a ring buffer of each character and the number of scanners at each position
    char_buffer: VecDeque<(char, usize)>,
    /// number of characters that have been discarded from the ring buffer due to all scanners being done with them
    popped_characters: usize,
}

impl<R: BufRead> StreamScanner<R> {
    pub(crate) fn new(reader: R) -> Self {
        let scanner = Self {
            character_position: 0,
            scanner_state: Rc::new(RefCell::new(ScannerState {
                char_buffer: VecDeque::with_capacity(64),
                inner_reader: reader,
                popped_characters: 0,
            })),
        };

        // unfortunately, we have to poll in the constructor of new() to have consistent state with copies made via clone
        scanner.poll_buffer();
        scanner.increment_scanner_count(1).ok();

        scanner
    }

    /// purges shared ring buffer of characters that have already been scanned by all existing scanners
    fn purge_buffer(&self) {
        let mut state = self.scanner_state.borrow_mut();

        let mut popped_characters = 0;

        for (_, count) in state.char_buffer.iter() {
            if *count > 0 {
                break;
            }
            popped_characters += 1;
        }

        state.popped_characters += popped_characters;
        state.char_buffer.drain(..popped_characters);

        // will be removed as dead code in release mode
        if popped_characters > 0 {
            trace!(
                "purged {} of {} bytes from scanner buffer (capacity {})",
                popped_characters,
                state.char_buffer.len() + popped_characters,
                state.char_buffer.capacity()
            );
        }
    }

    /// ensures that there are characters in the shared buffer and returns the one at the current position
    fn poll_buffer(&self) -> Option<char> {
        let mut state = self.scanner_state.borrow_mut();

        let index = self.character_position - state.popped_characters;
        // poll characters until we have our current position plus the next one if possible
        while state.char_buffer.len() <= index + 1 {
            // discards any invalid unicode character errors as we go
            match state.inner_reader.chars().filter_map(Result::ok).next() {
                Some(value) => state.char_buffer.push_back((value, 0)),
                _ => break,
            }
        }
        state.char_buffer.get(index).map(|entry| entry.0)
    }

    /// add or subtract the scanner count in the shared ring buffer at the current index
    /// has edge cases that we can basically ignore
    fn increment_scanner_count(&self, increment: isize) -> Result<(), IncrementScannerCountError> {
        let mut state = self.scanner_state.borrow_mut();
        let index = self.character_position - state.popped_characters;
        let entry = state
            .char_buffer
            .get_mut(index)
            .ok_or(IncrementScannerCountError::BufferOutOfBounds)?;
        entry.1 = entry
            .1
            .checked_add_signed(increment)
            .ok_or(IncrementScannerCountError::ScannerCountUnderflow)?;
        Ok(())
    }
}

// define errors out of existence
#[derive(Debug)]
enum IncrementScannerCountError {
    BufferOutOfBounds,
    ScannerCountUnderflow,
}

impl<R: BufRead> Iterator for StreamScanner<R> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        // read character at current index for this scanner
        let value = self.poll_buffer()?;

        // scanner count bookkeeping, errors can be ignored
        self.increment_scanner_count(-1).ok();
        self.character_position += 1;
        self.increment_scanner_count(1).ok();

        // discard characters that won't be scanned again
        // we don't have to check this CONSTANTLY, every n bytes or so will do
        if self.character_position % 1024 == 0 {
            self.purge_buffer();
        }

        Some(value)
    }
}

impl<R: BufRead> Clone for StreamScanner<R> {
    fn clone(&self) -> Self {
        // increase the count of scanners at this position
        self.poll_buffer();

        self.increment_scanner_count(1).ok();

        Self {
            character_position: self.character_position,
            scanner_state: self.scanner_state.clone(),
        }
    }
}

impl<R: BufRead> Drop for StreamScanner<R> {
    fn drop(&mut self) {
        // remember to decrement the count of scanners at this position
        self.poll_buffer();

        // decrement errors can be ignored
        self.increment_scanner_count(-1).ok();

        // if we are dropping the final scanner, print how many bytes we used
        if Rc::strong_count(&self.scanner_state) == 1 {
            let capacity = self.scanner_state.borrow().char_buffer.capacity();
            if capacity > self.character_position {
                debug!(
                    "drop() -> scanner overallocated {} (={}-{})",
                    ByteSize((capacity - self.character_position) as u64),
                    capacity,
                    self.character_position
                );
            } else {
                debug!(
                    "drop() -> scanner saved {} (={}-{})",
                    ByteSize((self.character_position - capacity) as u64),
                    self.character_position,
                    capacity
                );
            }
        }
    }
}
