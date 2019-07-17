use ndarray::{Array1, ArrayBase, Array2, stack, Axis, Array, Ix2, Data};
use ndarray_linalg::{Solve};


/// The simple linear regression model is
///     y = bX + e  where e ~ N(0, sigma^2 * I)
/// In probabilistic terms this corresponds to
///     y - bX ~ N(0, sigma^2 * I)
///     y | X, b ~ N(bX, sigma^2 * I)
/// The loss for the model is simply the squared error between the model
/// predictions and the true values:
///     Loss = ||y - bX||^2
/// The MLE for the model parameters b can be computed in closed form via the
/// normal equation:
///     b = (X^T X)^{-1} X^T y
/// where (X^T X)^{-1} X^T is known as the pseudoinverse / Moore-Penrose
/// inverse.
struct LinearRegression {
    beta: Option<Array1<f32>>,
    fit_intercept: bool,
}

impl LinearRegression {
    fn new(fit_intercept: bool) -> LinearRegression {
        LinearRegression {
            beta: None,
            fit_intercept
        }
    }

    fn fit(&mut self, mut X: Array2<f32>, y: Array1<f32>) {
        let (n_samples, n_features) = X.dim();

        // Check that our inputs have compatible shapes
        assert_eq!(y.dim(), n_samples);

        // If we are fitting the intercept, we need an additional column
        if self.fit_intercept {
            let dummy_column: Array<f32, _> = Array::ones((n_samples, 1));
            X = stack(Axis(1), &[dummy_column.view(), X.view()]).unwrap();
        };

        let rhs = X.t().dot(&y);
        let linear_operator = X.t().dot(&X);
        self.beta = Some(linear_operator.solve_into(rhs).unwrap());
    }

    fn predict<A>(&self, mut X: &mut ArrayBase<A, Ix2>) -> Array1<f32>
    where
        A: Data<Elem=f32>,
    {
        let (n_samples, n_features) = X.dim();

        // If we are fitting the intercept, we need an additional column
        let X = if self.fit_intercept {
            let dummy_column: Array<f32, _> = Array::ones((n_samples, 1));
            stack(Axis(1), &[dummy_column.view(), X.view()]).unwrap()
        } else {
            X.to_owned()
        };

        match &self.beta {
            None => panic!("The linear regression estimator has to be fitted first!"),
            Some(beta) => {
                X.dot(beta)
            }
        }
    }
}
