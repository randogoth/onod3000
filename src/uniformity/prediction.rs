// This file is a Rust port of the original Java implementation by Paul Uszak.
// Original Java code:
// http://www.reallyreallyrandom.com/gitbucketlabhub/
// 
// Copyright (c) 2023 Paul Uszak. Port (C) 2025 by Tobias Raayoni Last (@randogoth)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.


use statrs::distribution::{ChiSquared, ContinuousCDF};

/* 
 * Blatantly copied from David Sexton's battery.
 *
 * An algorithm is used to predict the value of each byte of the sequence from 
 * the beginning of the sequence to the end. In a random sequence the 
 * probability of success of any such algorithm is 1/256. The number of successes 
 * is counted. A chi-squared statistic is calculated. The degrees-of-freedom is 1. 
 * The algorithm is as follows: the next byte is predicted to be equal to all the 
 * previous bytes bitwise XORed together.
 */

use crate::Onod;

impl Onod {
    /// Prediction randomness test
    /// Evaluates the predictability of the next byte based on XORing the previous bytes
    /// and returns the total predictions, z-score, and p-value.
    pub fn prediction(samples: &[u8]) -> (f64, f64, f64) {
        if samples.len() < 3 {
            return (-1.0, 0.0, 1.0); // Not enough data for meaningful calculation
        }

        let mut correct_predictions = 0;
        let mut total_predictions = 0;

        let mut prediction = samples[0]; // Start with the first byte
        for i in 2..samples.len() {
            prediction ^= samples[i - 1]; // XOR all preceding bytes

            if prediction == samples[i] {
                correct_predictions += 1;
            }
            total_predictions += 1;
        }

        // Calculate expected and observed frequencies
        let expected = vec![
            (1.0 / 256.0) * samples.len() as f64, // Probability of correct prediction
            (255.0 / 256.0) * samples.len() as f64, // Probability of incorrect prediction
        ];
        let observed = vec![
            correct_predictions as f64, // Actual correct predictions
            (samples.len() - correct_predictions) as f64, // Actual incorrect predictions
        ];

        // Calculate chi-squared statistic
        let chi_squared_stat: f64 = observed
            .iter()
            .zip(expected.iter())
            .map(|(o, e)| (o - e).powi(2) / e)
            .sum();

        // Use Chi-Squared distribution to calculate p-value
        let chi_squared_dist = ChiSquared::new(1.0).unwrap(); // Degrees of freedom = 1
        let p_value = 1.0 - chi_squared_dist.cdf(chi_squared_stat);

        // Calculate z-score (optional, for diagnostics)
        let mean = 1.0; // Mean of chi-squared distribution
        let std_dev = (2.0 as f64).sqrt(); // Standard deviation of chi-squared distribution
        let z_score = (chi_squared_stat - mean) / std_dev;

        (total_predictions as f64, z_score, p_value)
    }
}
