use lex_sleuther::regression::{from_adhoc, Regression, SampleResult};
use ndarray::{Array1, ArrayView2};

mod crosstab;
mod kfold;
mod ridge_regression;

use crosstab::CrossTabulation;
pub(crate) use ridge_regression::ridge_regression_iter;

use crate::train::stats::kfold::kfold_mean;

/// Given features and ideals, perform kfold cross validation to determine the ideal alpha value for ridge regression.
pub(crate) fn determine_ideal_alpha(
    feature_matrix: ArrayView2<f64>,
    ideal_matrix: ArrayView2<f64>,
    k: usize,
    num_classes: usize,
    seed: u64,
    alphas: &[f64],
) -> f64 {
    let accuracies = kfold_mean(
        feature_matrix.view(),
        ideal_matrix.view(),
        k,
        alphas.len(),
        seed,
        |dataset| {
            ridge_regression_iter(dataset.training_features, dataset.training_ideals, alphas)
                .map(|weight_matrix| {
                    compute_regression_accuracy(
                        weight_matrix.view(),
                        dataset.testing_features,
                        dataset.testing_ideals,
                        num_classes,
                    )
                })
                .collect()
        },
    );

    // now that we have estimate of the the accuracy of various alphas, select the best one
    let mut alpha_scores: Vec<_> = accuracies.into_iter().enumerate().collect();
    alpha_scores.sort_unstable_by(|(_, a), (_, b)| b.total_cmp(a));
    alphas[alpha_scores.first().unwrap().0]
}

/// Given a weight matrix and a testing dataset, how accurate is it?
fn compute_regression_accuracy(
    weight_matrix: ArrayView2<f64>,
    testing_features: ArrayView2<f64>,
    testing_ideals: ArrayView2<f64>,
    num_classes: usize,
) -> f64 {
    // regress, compute cross tabulation, compute accuracy
    let testing_regression = from_adhoc(weight_matrix);
    let predicted_classifications: Array1<usize> = testing_regression
        .results(testing_features)
        .iter()
        .map(|res| res.sorted_classes().get(0).unwrap().to_owned())
        .collect();

    // convert testing ideals into winning classifications
    let actual_classifications: Array1<usize> = testing_ideals
        .rows()
        .into_iter()
        .map(|ideal| {
            SampleResult::from_adhoc(ideal)
                .sorted_classes()
                .get(0)
                .unwrap()
                .to_owned()
        })
        .collect();

    // cross tabulate to compute accuracy
    let metrics = CrossTabulation::new(
        predicted_classifications.view(),
        actual_classifications.view(),
        num_classes,
    );

    metrics.accuracy().mean().unwrap()
}
