use ndarray::{s, Array1, Array2, ArrayView2, ArrayViewMut2, Axis, Slice, Zip};
use rand::{rngs::SmallRng, seq::index::sample, SeedableRng};

pub(crate) struct KfoldDataset<'a> {
    pub(crate) fold_idx: usize,
    pub(crate) training_features: ArrayView2<'a, f64>,
    pub(crate) testing_features: ArrayView2<'a, f64>,
    pub(crate) training_ideals: ArrayView2<'a, f64>,
    pub(crate) testing_ideals: ArrayView2<'a, f64>,
}

/// Kfolding that will determine the statistical mean of the metrics your closure produces.
/// Used for cross-validation.
pub(crate) fn kfold_mean<F>(
    feature_matrix: ArrayView2<f64>,
    ideal_matrix: ArrayView2<f64>,
    k: usize,
    num_metrics: usize,
    seed: u64,
    f: F,
) -> Array1<f64> where
    F: Fn(&KfoldDataset) -> Vec<f64>,
{
    let mut metrics = Array2::zeros((k, num_metrics));
    
    kfold(feature_matrix, ideal_matrix, k, seed, |dataset| {
        let elements = f(dataset);
        metrics
            .row_mut(dataset.fold_idx)
            .into_iter()
            .zip(elements)
            .for_each(|(e, metric)| *e = metric);
    });

    // PANIC: mean_axis will only fail if k <= 0
    metrics.mean_axis(Axis(1)).unwrap()
}

/// Executes the provided function `k` times with a different shuffled sampling of the provided dataset.
/// Used for cross-validation.
pub(crate) fn kfold<F>(
    feature_matrix: ArrayView2<f64>,
    ideal_matrix: ArrayView2<f64>,
    k: usize,
    seed: u64,
    mut f: F,
) where
    F: FnMut(&KfoldDataset),
{
    // in general, matrices of the same nrows and the same seed
    // will be shuffled the same way
    let mut rng = SmallRng::seed_from_u64(seed);
    let indices = sample(&mut rng, feature_matrix.nrows(), feature_matrix.nrows()).into_vec();

    // we allocate space for the contiguous shuffled matrices immediately
    let mut shuffled_features = feature_matrix.select(Axis(0), &indices);
    let mut shuffled_ideals = ideal_matrix.select(Axis(0), &indices);

    // slightly lossy here, some features may not be used at all if k does not divide #features evenly
    let chunk_size = shuffled_features.nrows() / k;

    for chunk_idx in 0..k {
        // swap the first chunk's rows with the nth chunks rows.
        // this way, we can split the dataset at the same point and still have different contiguous folds.
        if chunk_idx > 0 {
            // a bit of manual chunking since we need the chunks themselves as well as the remaining chunks for training
            let start_idx = chunk_idx * chunk_size;
            let end_idx = (chunk_idx + 1) * chunk_size;

            swap_rows(
                shuffled_features.view_mut(),
                ..chunk_size,
                start_idx..end_idx,
            );
            swap_rows(shuffled_ideals.view_mut(), ..chunk_size, start_idx..end_idx);
        }

        // due to the swapping happening up above, we can split these views along the same
        // point every time
        let (testing_features, training_features) =
            shuffled_features.view().split_at(Axis(0), chunk_size);
        let (testing_ideals, training_ideals) =
            shuffled_ideals.view().split_at(Axis(0), chunk_size);

        let dataset = KfoldDataset {
            fold_idx: chunk_idx,
            training_features,
            testing_features,
            training_ideals,
            testing_ideals,
        };

        // calling the closure
        f(&dataset);
    }
}

/// Efficiently swaps two (non-overlapping!) ranges of rows in the same matrix.
fn swap_rows<R1: Into<Slice>, R2: Into<Slice>>(
    mut matrix: ArrayViewMut2<f64>,
    from_range: R1,
    to_range: R2,
) {
    let (testing_chunk, training_chunk) =
        matrix.multi_slice_mut((s![from_range.into(), ..], s![to_range.into(), ..]));
    Zip::from(testing_chunk)
        .and(training_chunk)
        .for_each(::std::mem::swap);
}
