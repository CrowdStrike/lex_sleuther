# multiplexer

A **multiplexer** over multiple lexers. 

This crate is only responsible for actually lexing source code and producing a feature vector, a glorified count of each token occurrence.

We leverage the [`lexgen`](https://github.com/osa1/lexgen) lexer library to build optimized lexer state machines quickly. In many cases, the resulting tokenization is not totally correct, but it is good enough and fast enough to provide a meaningful guess.

### adding new lexers

There are three general steps to adding new lexers:

1. Create a new crate using the `lexgen::lexer!` macro to generate a lexer based on a PEG-like syntax. Use existing lexers as a guide.
2. Add your lexer to the array `lexers` in `multiplexer/src/lib.rs`. 
3. Create a simple test to sanity check the stability of your lexer. Use existing tests as an example.

Note that adding a lexer here does *not* add a new classification set to [`lex_sleuther`](../../README.md) upstream.
The set of classification categories is completely decoupled from the set of lexers this library uses internally. 