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
    /// Entropy randomness test
    /// Calculates the Shannon entropy of a byte slice and outputs a p-value.
    pub fn shannon(samples: &[u8]) -> (f64, f64, f64) {

        let len = samples.len() as f64;
        if len == 0.0 {
            return (-1.0, 0.0, 1.0);
        }

        // Count occurrences of each byte
        let mut counts = [0usize; 256];
        for &byte in samples {
            counts[byte as usize] += 1;
        }

        // Calculate Shannon entropy
        let entropy: f64 = counts.iter()
            .filter(|&&count| count > 0)
            .map(|&count| {
                let p = count as f64 / len;
                -p * p.log2()
            })
            .sum();

        // Expected entropy for a uniform distribution
        let expected_entropy = 8.0;

        // Calculate Z statistic
        let std_dev = (0.833_f64).sqrt();
        let z_score = (entropy - expected_entropy) * len.sqrt() / std_dev;

        // Calculate p-value from Z score
        let normal_dist = Normal::new(0.0, 1.0).expect("Failed to create Normal distribution");
        let p_value = 2.0 * (1.0 - normal_dist.cdf(z_score.abs())); // Two-tailed test

        (entropy, z_score, p_value)
    }
}
