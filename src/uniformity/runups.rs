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

use crate::Onod;

impl Onod {

    /// RunUps randomness test
    /// Evaluates the number of four-byte run-ups in the data and returns a p-value.
    pub fn run_ups(input: &[u8]) -> (f64, f64, f64) {

        let samples = input.iter().map(|&x| x as u32).collect::<Vec<u32>>();

        if samples.len() < 4 {
            return (-1.0, 0.0, 1.0); // Not enough data for meaningful calculation
        }

        let mut test_statistic = 0;

        for chunk in samples.chunks(4) {
            if let [first, second, third, fourth] = chunk {
                if first < second && second < third && third < fourth {
                    test_statistic += 1;
                }
            }
        }

        let total_chunks = samples.len() / 4;
        let no_expected = 2_731_135.0 / 67_108_864.0 * total_chunks as f64;

        let observed = [test_statistic as f64, total_chunks as f64 - test_statistic as f64];
        let expected = [no_expected, total_chunks as f64 - no_expected];

        // Use chi-squared test to calculate p-value
        let chi_squared_dist = ChiSquared::new(1.0).expect("Failed to create ChiSquared distribution");
        let chi_squared_stat: f64 = observed.iter()
            .zip(expected.iter())
            .map(|(o, e)| (o - e).powi(2) / e)
            .sum();

        let p_value = 1.0 - chi_squared_dist.cdf(chi_squared_stat);

        // Z-score calculation (standardization of the chi-squared statistic)
        let mean = 1.0; // Mean of the chi-squared distribution
        let std_dev = (2.0 as f64).sqrt(); // Standard deviation of the chi-squared distribution
        let z_score = (chi_squared_stat - mean) / std_dev;

        (chi_squared_stat, z_score, p_value)
    }
}