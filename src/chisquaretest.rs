use statrs::distribution::{ChiSquared, ContinuousCDF};

pub fn chi_square_test(observed: &[u64], expected: &[f64]) -> f64 {
    // Preconditions
    if observed.len() != expected.len() || observed.len() < 2 {
        panic!("Observed and expected arrays must have the same length and length >= 2.");
    }
    if expected.iter().any(|&e| e <= 0.0) {
        panic!("Expected array must contain only strictly positive values.");
    }

    // Rescale expected array if necessary
    let sum_observed: f64 = observed.iter().map(|&o| o as f64).sum();
    let sum_expected: f64 = expected.iter().sum();
    let rescaled_expected: Vec<f64> = expected.iter().map(|&e| e * sum_observed / sum_expected).collect();

    // Calculate chi-squared statistic
    let chi_squared_stat: f64 = observed
        .iter()
        .zip(rescaled_expected.iter())
        .map(|(&o, &e)| (o as f64 - e).powi(2) / e)
        .sum();

    // Perform chi-squared test
    let degrees_of_freedom = observed.len() as f64 - 1.0;
    let chi_squared_dist = ChiSquared::new(degrees_of_freedom).expect("Failed to create ChiSquared distribution");
    let p_value = 1.0 - chi_squared_dist.cdf(chi_squared_stat);

    p_value
}