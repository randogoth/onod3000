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


use statrs::distribution::{Normal, ContinuousCDF};

use crate::Onod;

impl Onod {

    /// Monobit randomness test
    /// Evaluates the balance of 0s and 1s in the binary representation of the data and returns a p-value.
    pub fn monobit(samples: &[u8]) -> (f64, f64, f64) {  

        if samples.is_empty() {
            return (-1.0, 0.0, 1.0); // Default to perfect randomness for empty data
        }

        // Count the total number of 1s in the dataset
        let mut total_ones = 0;
        let mut total_bits = 0;

        for &byte in samples {
            total_ones += byte.count_ones() as usize;
            total_bits += 8;
        }

        // Calculate the observed proportion of 1s
        let observed_proportion = total_ones as f64 / total_bits as f64;

        // Expected proportion for random data
        let expected_proportion = 0.5;
        let std_dev = (0.5 * 0.5 / total_bits as f64).sqrt(); // Standard deviation for a binomial distribution

        // Calculate the z-score
        let z_score = (observed_proportion - expected_proportion) / std_dev;

        // Use normal distribution to calculate p-value
        let normal_dist = Normal::new(0.0, 1.0).expect("Failed to create Normal distribution");
        let p_value = 2.0 * (1.0 - normal_dist.cdf(z_score.abs()));

        (observed_proportion, z_score, p_value)
    }
}