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

    pub fn chi_bit(samples: &[u8]) -> (f64, f64, f64) {
    
        if samples.is_empty() {
            return (-1.0, 0.0, 1.0); // empty data
        }
    
        // Lookup table for the number of set bits in each byte (Hamming weight)
        const SET_BITS_PER_BYTE: [usize; 256] = [
            0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 1, 2, 2, 3,
            2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 1, 2, 2, 3, 2, 3, 3, 4,
            2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5,
            4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
            2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5,
            4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6,
            4, 5, 5, 6, 5, 6, 6, 7, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7,
            6, 7, 7, 8,
        ];
    
        // Expected number of occurrences for each Hamming weight
        const EXPECTED_NUMBER: [f64; 9] = [1.0, 8.0, 28.0, 56.0, 70.0, 56.0, 28.0, 8.0, 1.0];
    
        // Frequency count of Hamming weights
        let mut frequency = vec![0; EXPECTED_NUMBER.len()];
        for &byte in samples {
            let hamming_weight = SET_BITS_PER_BYTE[byte as usize];
            frequency[hamming_weight] += 1;
        }
    
        // Calculate observed and expected counts
        let total_samples = samples.len() as f64;
        let expected: Vec<f64> = EXPECTED_NUMBER
            .iter()
            .map(|&e| e / 256.0 * total_samples)
            .collect();
    
        // Chi-squared statistic calculation
        let chi_squared_stat = frequency
            .iter()
            .zip(expected.iter())
            .map(|(&observed, &expected)| {
                if expected > 0.0 {
                    (observed as f64 - expected).powi(2) / expected
                } else {
                    0.0
                }
            })
            .sum();
    
        // Degrees of freedom: 9 categories - 1
        let degrees_of_freedom = (EXPECTED_NUMBER.len() - 1) as f64;
        let chi_squared_dist = ChiSquared::new(degrees_of_freedom).expect("Failed to create ChiSquared distribution");
        let p_value = 1.0 - chi_squared_dist.cdf(chi_squared_stat);
    
        // Z-score calculation (standardization of the chi-squared statistic)
        let mean = degrees_of_freedom; // Mean of the chi-squared distribution
        let std_dev = (2.0 * degrees_of_freedom).sqrt(); // Standard deviation of the chi-squared distribution
        let z_score = (chi_squared_stat - mean) / std_dev;

        (chi_squared_stat, z_score, p_value)

    }   
}