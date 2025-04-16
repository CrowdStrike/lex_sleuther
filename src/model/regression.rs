use std::path::PathBuf;

use ndarray::{Array1, Array2, ArrayView1, ArrayView2};

use crate::model::lex;

/// A regression capable of producing RegressionResults from files or bytes.
pub trait Regression {
    fn weights(&self) -> Array2<f64>;

    /// Given a raw frequency matrix, apply the regression.
    /// See `results_from_files` and `results_from_bytes` for a simpler API.
    fn results(&self, frequency_matrix: ArrayView2<f64>) -> RegressionResults;

    /// Performs regression of the features of the provided file paths in parallel.
    /// The order of the resulting RegressionResults corresponds with the order of the provided files.
    fn results_from_files(&self, sample_files: &Vec<PathBuf>) -> RegressionResults {
        let frequency_matrix = lex::get_frequency_matrix_from_files(sample_files);
        self.results(frequency_matrix.view())
    }

    /// Performs regression of the features of the file represented by the provided bytes.
    /// The RegressionResults will produce one SampleResult.
    fn results_from_bytes(&self, sample_bytes: &Vec<u8>) -> RegressionResults {
        let frequency_matrix = lex::get_frequency_matrix_from_bytes(sample_bytes);
        self.results(frequency_matrix.view())
    }
}

/// Concrete Regression implementation generic over Owned or Borrowed data.
/// By using the Regression trait, we can gloss over the details.
struct RegressionBase<D: ndarray::Data> {
    weight_matrix: ndarray::ArrayBase<D, ndarray::Ix2>,
}

impl<D> Regression for RegressionBase<D>
where
    D: ndarray::Data<Elem = f64>,
{
    fn results(&self, frequency_matrix: ArrayView2<f64>) -> RegressionResults {
        let results_matrix = frequency_matrix.dot(&self.weight_matrix);
        RegressionResults { results_matrix }
    }
    
    fn weights(&self) -> Array2<f64> {
        self.weight_matrix.to_owned()
    }
}

/// The results of a successful regression.
/// Will contain as many SampleResults as there were original samples regressed.
pub struct RegressionResults {
    results_matrix: Array2<f64>,
}

impl RegressionResults {
    /// Iterate over all the SampleResults in order.
    pub fn iter(&self) -> impl Iterator<Item = SampleResult> {
        self.results_matrix
            .outer_iter()
            .map(|row| SampleResult { sample_scores: row })
    }

    /// Get a particular sample's SampleResult
    pub fn get_sample_result(&self, sample_idx: usize) -> SampleResult {
        SampleResult {
            sample_scores: self.results_matrix.row(sample_idx),
        }
    }
}

/// The results of regressing a single sample.
pub struct SampleResult<'a> {
    sample_scores: ArrayView1<'a, f64>,
}

impl<'a> SampleResult<'a> {
    /// Create a SampleResult from raw scores, useful for code-reuse during training.
    pub fn from_adhoc(sample_scores: ArrayView1<'a, f64>) -> Self {
        Self { sample_scores }
    }

    /// Convert the SampleResult into its underlying raw scores.
    pub fn into_scores(self) -> ArrayView1<'a, f64> {
        self.sample_scores
    }

    /// An array where every classification index sorted by most to least likely.
    /// If you are only interested in the winning classification,
    /// simply take the first entry.
    ///
    /// Resolving the actual scores or human-readable labels of these classification indices is left to the caller (usually Model).
    pub fn sorted_classes(&self) -> Array1<usize> {
        let mut row_vec: Vec<_> = self.sample_scores.iter().copied().enumerate().collect();
        // the actual secret sauce here is simple: the highest scores win
        row_vec.sort_unstable_by(|(_, a), (_, b)| b.total_cmp(a));
        row_vec.into_iter().map(|(idx, _)| idx).collect()
    }

    /// An array of probabilities between 0..1 where each value corresponds with the relative
    /// probabilities of each classification rather than the absolute score returned by
    /// the result matrix. Note that the "probabilities" returned do not correspond
    /// to a rigorous mathematical metric, but simply match an intuitive notion
    /// of probability. This is generally only useful for reporting to humans.
    pub fn probabilities(&self) -> Array1<f64> {
        // first pass: map the range of all floats to strictly positive floats
        let positive_scores: Vec<_> = self
            .sample_scores
            .iter()
            // multiplying by 100 here helps the result match our intuitive
            // notion of "percent chance"
            .map(|score| score * 100.0)
            .map(|score| {
                // negatives scores get mapped to 0..1
                if score < 0.0 {
                    score.exp()
                // positive scores get mapped to >1
                } else {
                    score + 1.0
                }
            })
            .collect();

        // second pass: get the total sum of all scores for normalization
        let score_sum: f64 = positive_scores.iter().cloned().sum();

        // third pass: normalize each score so they actually add to 1.0
        positive_scores
            .into_iter()
            .map(move |val| val / score_sum)
            .collect()
    }
}

/// Create a Regression from a raw weight matrix, useful for training.
pub fn from_adhoc<D: ndarray::Data<Elem = f64>>(weight_matrix: ndarray::ArrayBase<D, ndarray::Ix2>) -> impl Regression {
    RegressionBase {
        weight_matrix
    }
}