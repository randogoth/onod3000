// MIT License

// Copyright (c) 2025 Tobias Raayoni Last (@randogoth)

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// ---

// This project is a Rust port of the original Java implementation by Paul Uszak.
// Original work is licensed under the terms specified in the respective file header.


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