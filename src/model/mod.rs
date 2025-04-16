use std::path::PathBuf;

use ndarray::ArrayView2;

pub mod regression;
pub mod lex;

use regression::{Regression, RegressionResults};

/// A model capable of producing Classifications from files or bytes.
pub struct Model<R: Regression> {
    regression: R,
    class_labels: Vec<String>,
}

impl<R: Regression> Model<R> {
    /// uses the internal RegressionResults API to generate Classification structs
    fn classifications<'a, 'b: 'a>(
        &'b self,
        regression_results: &'a RegressionResults,
    ) -> impl Iterator<Item = Classification<'b>> + 'a {
        regression_results.iter().map(|res| {
            let probabilities = res.probabilities();
            let classes = res.sorted_classes();
            let scores = res.into_scores();
            let verdicts = classes
                .into_iter()
                .map(|class_idx| Verdict {
                    label: &self.class_labels[class_idx],
                    score: scores[class_idx],
                    probability: probabilities[class_idx],
                })
                .collect();

            Classification { verdicts }
        })
    }

    /// Classifies the provided file paths in parallel.
    /// The order of the resulting classifications corresponds with the order of the provided files.
    pub fn classify_files(&self, files: &Vec<PathBuf>) -> Vec<Classification> {
        let results = self.regression.results_from_files(files);
        self.classifications(&results).collect()
    }

    /// Classifies the file containing the provided bytes.
    pub fn classify_bytes(&self, bytes: &Vec<u8>) -> Classification {
        let results = self.regression.results_from_bytes(bytes);
        let classification = self.classifications(&results).next().unwrap();
        classification
    }
}

/// A list of verdicts in descending order of likelyhood.
pub struct Classification<'a> {
    pub verdicts: Vec<Verdict<'a>>,
}

/// The score and probability returned for each class label.
pub struct Verdict<'a> {
    pub label: &'a str,
    pub score: f64,
    pub probability: f64,
}

#[cfg(feature = "baked_model")]
mod baked;

#[cfg(feature = "baked_model")]
/// Returns the model prebaked into the library with the `baked_model` feature
pub fn baked_model() -> Model<impl Regression> {
    use self::regression::from_adhoc;

    let ncols = baked::CLASS_LABELS.len();
    let nrows = baked::WEIGHT_ARRAY.len() / ncols;
    // this doesn't do any extra allocations, it just needs to verify the slice dimensions
    let weight_matrix = ArrayView2::from_shape((nrows, ncols), &baked::WEIGHT_ARRAY).unwrap();
    let regression = from_adhoc(weight_matrix);
    let class_labels = baked::CLASS_LABELS
        .iter()
        .map(ToString::to_string)
        .collect();
    Model {
        regression,
        class_labels,
    }
}
