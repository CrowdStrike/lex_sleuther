use ndarray::{Array2, ArrayView2};
use ndarray_linalg::LeastSquaresSvdInPlace;


/// Does repeated ridge regressions with multiple alpha values slightly efficiently
pub(crate) fn ridge_regression_iter<'a>(
    a: ArrayView2<f64>,
    b: ArrayView2<f64>,
    alphas: &'a [f64],
) -> impl Iterator<Item = Array2<f64>> + 'a + use<'a> {
    let x_t = a.t();
    let x_t_y = x_t.dot(&b);
    let x_t_x = x_t.dot(&a);

    let mut lhs = Array2::zeros(x_t_x.raw_dim());
    let mut rhs = Array2::zeros(x_t_y.raw_dim());

    alphas.iter().map(move |&alpha| {
        // we have to clone these on each call because lapack writes the solution into the original matrices as we solve
        lhs.clone_from(&x_t_x);
        rhs.clone_from(&x_t_y);
        // offsetting the so-called "ridge"
        let mut diagonal = lhs.diag_mut();
        diagonal += alpha;
        // least squares via Singular Value Decomposition
        let svd = lhs.least_squares_in_place(&mut rhs).unwrap();
        svd.solution
    })
}