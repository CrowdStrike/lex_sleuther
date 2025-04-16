use std::path::PathBuf;

use multiplexer::{lexer_results_from_bytes, lexer_results_from_file, LexerResult};
use ndarray::Array2;
use rayon::prelude::*;

/// Get the frequency matrix where each row is the frequency of each token in a sample.
/// Frequency is used in place of raw counts to account for files of drastically different sizes.
pub fn get_frequency_matrix_from_files(sample_files: &Vec<PathBuf>) -> Array2<f64> {
    let token_frequencies: Vec<f64> = sample_files
        // we have to use flat_map_iter() instead of flat_map() + par_bridge()
        // to prevent non-determinism in the order things are returned.
        // par_bridge() is NOT guaranteed to keep the order of the original iterator!
        .par_iter()
        .flat_map_iter(|path| {
            let lexer_results = lexer_results_from_file(path).unwrap();
            lexer_results
                .into_iter()
                .flat_map(LexerResult::into_frequency_vector_iter)
        })
        .collect();

    Array2::from_shape_vec(
        (
            sample_files.len(),
            token_frequencies.len() / sample_files.len(),
        ),
        token_frequencies,
    )
    .unwrap()
}

/// Get the frequency matrix where each row is the frequency of each token in a sample.
/// Frequency is used in place of raw counts to account for files of drastically different sizes.
pub fn get_frequency_matrix_from_bytes(sample_bytes: &Vec<u8>) -> Array2<f64> {
    let lexer_results = lexer_results_from_bytes(sample_bytes);
    let token_frequencies: Vec<f64> = lexer_results
        .into_iter()
        .flat_map(LexerResult::into_frequency_vector_iter)
        .collect();

    Array2::from_shape_vec((1, token_frequencies.len()), token_frequencies).unwrap()
}
