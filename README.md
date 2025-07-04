![CrowdStrike Falcon](https://raw.githubusercontent.com/CrowdStrike/falconpy/main/docs/asset/cs-logo.png) [![Twitter URL](https://img.shields.io/twitter/url?label=Follow%20%40CrowdStrike&style=social&url=https%3A%2F%2Ftwitter.com%2FCrowdStrike)](https://twitter.com/CrowdStrike)<br/>

# lex_sleuther

This program can read and understand the source code of many script languages and will tell you which language a text file is most likely to contain. **Lex Sleuther is a novel experiment**. It is provided for the potential benefit of the community but its applications are limited by design. For more information about its origins, how it works, and what problems it solves, please see the [2025 BSidesSF talk about it here](#TODO).

The approach here differs greatly from similar projects like `guesslang` or `magika`.
Rather than using a trained ML model, `lex_sleuther` is actually a *lexer* for each of the 
**supported languages**, and counts the different types of tokens and errors encountered. 
These tokens are given weights according to empirical analysis of real samples,
and new samples are classified probabilistically.

## using

Assuming you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed,
you can quickly install a functioning CLI with this command:

```bash
# TODO: replace with crates.io install command 
cargo install --git https://github.com/CrowdStrike/lex_sleuther.git
lex_sleuther --help
```

## supported languages

Because the scope of this project is much smaller than that of `guesslang` or `magika`,
we can afford to focus on a small number of script languages prevalent in malware campaigns.
This is to our advantage: the fewer languages we support, the faster and more accurate
our disambiguation efforts become.

| Language       | Status    |
| -------------- | --------- |
| HTML           | Supported |
| Visual Basic 6 | Supported |
| Python         | Supported |
| Batch          | Supported |
| PowerShell     | Supported |
| JavaScript     | Supported |

### adding new lexers

To support a new language with a new lexer, follow the instructions in the [`lex_sleuther_multiplexer` README](./crates/lex_sleuther_multiplexer/README.md).

Note that adding a lexer does *not* add a new classification set to the `lex_sleuther` CLI.
The set of classification categories is completely decoupled from the set of lexers built into `lex_sleuther_multiplexer`. 

### adding new recognized languages

To add a new classification category (which may or may not have a corresponding lexer), you'll need to retrain on a new dataset. 
Follow the [instructions in the dataset folder to retrain](./dataset/).

Note that its possible to train on datasets containing languages that do not have a corresponding lexer, but your mileage may vary. 

## erata

### on performance

The root of all evil is premature optimization, and we have avoided doing that here,
but I did evaluate where `lex_sleuther` tends to spend most of its time.

**TL;DR**: its the lexing. 

The state machines currently generated by the [`lexgen` crate](https://github.com/osa1/lexgen) are [not minimized](https://github.com/osa1/lexgen/issues/38), which would lead to the most substantial performance improvements. There are [crates that do this optimization](https://docs.rs/logos/latest/logos/#) and achieve great speedups, but in my usage they tended to break in this use case for unknown reasons (ie stack overflows while expanding macros, etc).

There's also an optimization in the internal crate [`lex_sleuther_multiplexer`](./crates/lex_sleuther_multiplexer/README.md) that attempts to save memory while scanning over large files.
Long story short, it doesn't work and uses more memory than it saves, on average. Good performance improvements would result from fixing that.

### on efficacy 

In general, this project out-performs `guesslang` and is on par with `Magika`, but only for the narrow set of filetypes which `lex_sleuther` recognizes. On a corpus of ~3000 files sourced from CrowdStrike-internal malware feeds that are only the supported filetypes, the following accuracies were observed.

| filetype | lex_sleuther-0.2.8_acc | magika-0.6.1-standard_v3_2_acc |
| -------- | ---------------------- | ------------------------------ |
| bat      | 0.9967                 | 0.9988                         |
| html     | 0.9981                 | 0.9905                         |
| js       | 0.9933                 | 0.9892                         |
| ps1      | 0.9918                 | 0.9968                         |
| python   | 0.9914                 | 0.9870                         |
| vb6      | 0.9969                 | 0.9961                         |

**TODO**: the tool that generated this data will be published on a later date. 

Lex Sleuther currently does the simpliest possible inference (linear regression on token frequencies),
and there are multiple potential improvements that remain unexplored:

1. Account for symbol locality.
2. Trial non-linear inference algorithms. 
3. Improve lexer correctness with unit testing.

#### note on differences from Magika

The original version of `Magika` struggled with "nested" formats, ie scripts that contain other programming languages. For example, an HTML file often contains large sections of JavaScript, CSS, SVG, JSON, and others. This was a contributing factor to `Magika`'s high false negative rate with such files, and was one area where Lex Sleuther outperformed Magika outright. Because Lex Sleuther lexes files according to the rules of that language, it does not suffer from this problem, and usually ignores everything that isn't the outer-most language. That said, since the v3 of `Magika`'s model, this limitation has been seemingly addressed. 

## support

`lex_sleuther` is an open source project, not a CrowdStrike product. As such, it carries no formal support, expressed or implied.

If you encounter any issues while using `lex_sleuther`, you can create an issue on our Github repo for bugs, enhancements, or other requests.

## disclaimer

This project is for research purposes only. This project is not supported by CrowdStrike and CrowdStrike specifically disclaims all warranties as to its quality, merchantability, or fitness for a particular purpose.