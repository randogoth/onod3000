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

    /// Mean randomness test
    /// Calculates the p-value for the mean of the byte slice compared to expected mean.
    pub fn mean_byte(samples: &[u8]) -> (f64, f64, f64) {
    
        let len = samples.len() as f64;
        if len == 0.0 {
            return (-1.0, 0.0, 1.0);
        }
    
        // Calculate observed mean
        let observed_mean: f64 = samples.iter().map(|&x| x as f64).sum::<f64>() / len;
    
        // Expected mean for uniform distribution
        let expected_mean = 127.5;
    
        // Calculate standard deviation of the mean
        let std_dev_mean = ((256.0 * 256.0 - 1.0) / (12.0 * len)).sqrt();
    
        // Calculate the z-score
        let z_score = (observed_mean - expected_mean) / std_dev_mean;
    
        // Use normal distribution to calculate p-value
        let normal_dist = Normal::new(0.0, 1.0).expect("Failed to create Normal distribution");
        let p_value = 2.0 * (1.0 - normal_dist.cdf(z_score.abs())); // Two-tailed test
    
        (observed_mean, z_score, p_value)
    }
}