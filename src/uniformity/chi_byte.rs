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


use std::collections::HashMap;
use statrs::distribution::{ChiSquared, ContinuousCDF};

use crate::Onod;

impl Onod {

    /// ChiByte randomness test
    /// Evaluates the uniformity of byte values across the data and returns a p-value.
    pub fn chi_byte(samples: &[u8]) -> (f64, f64, f64) {

        if samples.is_empty() {
            return (-1.0, 0.0, 1.0); // Default to perfect randomness for empty data
        }

        // Count occurrences of each byte value (0-255)
        let mut counts = HashMap::new();
        for &byte in samples {
            *counts.entry(byte).or_insert(0) += 1;
        }

        // Calculate expected count assuming uniform distribution
        let expected_count = samples.len() as f64 / 256.0;

        // Calculate chi-squared statistic
        let mut chi_squared_stat = 0.0;
        for i in 0..256 {
            let observed = *counts.get(&(i as u8)).unwrap_or(&0) as f64;
            let diff = observed - expected_count;
            chi_squared_stat += (diff * diff) / expected_count;
        }

        // Use chi-squared distribution to calculate p-value
        let degrees_of_freedom = 256.0 - 1.0; // 256 possible byte values - 1
        let chi_squared_dist = ChiSquared::new(degrees_of_freedom).expect("Failed to create ChiSquared distribution");
        let p_value = 1.0 - chi_squared_dist.cdf(chi_squared_stat);

        // Z-score calculation (standardization of the chi-squared statistic)
        let mean = degrees_of_freedom; // Mean of the chi-squared distribution
        let std_dev = (2.0 * degrees_of_freedom).sqrt(); // Standard deviation of the chi-squared distribution
        let z_score = (chi_squared_stat - mean) / std_dev;

        (chi_squared_stat, z_score, p_value)
    }

}