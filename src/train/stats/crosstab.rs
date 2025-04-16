use ndarray::{Array1, Array2, ArrayView1, Axis, Zip};


pub(crate) struct CrossTabulation {
    pub(crate) true_positives: Array1<usize>,
    pub(crate) false_positives: Array1<usize>,
    pub(crate) false_negatives: Array1<usize>,
    pub(crate) true_negatives: Array1<usize>,
}

impl CrossTabulation {
    /// given two predictors, cross tabulate and compute various metrics
    pub(crate) fn new(
        predicted_classifications: ArrayView1<usize>,
        actual_classifications: ArrayView1<usize>,
        num_classes: usize,
    ) -> Self {
        let mut frequency_table = Array2::zeros((num_classes, num_classes));
        Zip::from(actual_classifications)
            .and(predicted_classifications)
            .for_each(|act, pred| frequency_table[[*act, *pred]] += 1);

        // from the frequency table, compute TP, FP, FN, and TN
        let true_positives = frequency_table.diag().to_owned();
        let false_positives = frequency_table.sum_axis(Axis(0)) - &true_positives;
        let false_negatives = frequency_table.sum_axis(Axis(1)) - &true_positives;
        let true_negatives = Zip::from(&true_positives)
            .and(&false_positives)
            .and(&false_negatives)
            .map_collect(|tp, p, n| actual_classifications.len() - (tp + p + n));
        Self {
            true_positives,
            false_positives,
            false_negatives,
            true_negatives,
        }
    }

    /// compute accuracy via (tp + tn)/(tp + fn + fp + tn)
    pub(crate) fn accuracy(&self) -> Array1<f64> {
        Zip::from(&self.true_positives)
            .and(&self.false_positives)
            .and(&self.false_negatives)
            .and(&self.true_negatives)
            .map_collect(|tp, p, n, tn| (tp + tn) as f64 / (tp + p + n + tn) as f64)
    }
}