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

    /// Avalanche randomness test
    /// Compares the bit-level differences between consecutive chunks of data and returns a p-value.
    pub fn avalanche(samples: &[u8]) -> (f64, f64, f64) {

        const XOR_WINDOW_SIZE: usize = 20; // Bytes. Equivalent to SHA-1 (160 bits).

        if samples.len() < 2 * XOR_WINDOW_SIZE {
            return (-1.0, 0.0, 1.0); // Not enough data for meaningful calculation
        }

        let mut means = Vec::new();

        for i in (0..samples.len() - (2 * XOR_WINDOW_SIZE)).step_by(2 * XOR_WINDOW_SIZE) {
            let a_start = i;
            let a_end = i + XOR_WINDOW_SIZE;
            let b_start = a_end;
            let b_end = b_start + XOR_WINDOW_SIZE;

            let a_bytes = &samples[a_start..a_end];
            let b_bytes = &samples[b_start..b_end];

            // XOR the two chunks and count differing bits
            let mut changed_bits = 0;
            for (a, b) in a_bytes.iter().zip(b_bytes.iter()) {
                changed_bits += (a ^ b).count_ones();
            }

            means.push(changed_bits as f64);
        }

        // Calculate the mean and standard deviation of bit differences
        let mean_observed = means.iter().sum::<f64>() / means.len() as f64;
        let mean_ref = (XOR_WINDOW_SIZE * 8) as f64 / 2.0; // Expected mean bits
        let std_dev_ref = 0.5 * ((XOR_WINDOW_SIZE * 8) as f64).sqrt(); // Expected standard deviation

        // Calculate Z score
        let z_score = (mean_observed - mean_ref) / std_dev_ref;

        // Convert Z score to p-value
        let normal_dist = Normal::new(0.0, 1.0).expect("Failed to create Normal distribution");
        let p_value = 2.0 * (1.0 - normal_dist.cdf(z_score.abs()));

        (mean_observed, z_score, p_value)
    }

}