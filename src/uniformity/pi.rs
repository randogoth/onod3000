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


/// Pi randomness test
/// Uses a Monte Carlo simulation to estimate randomness by calculating the approximation of Pi.

use statrs::distribution::{Normal, ContinuousCDF};

use crate::Onod;

impl Onod {
    /// Pi randomness test using nalgebra for vectorized operations.
    pub fn pi(samples: &[u8]) -> (f64, f64, f64) {
        if samples.len() < 4 {
            return (-1.0, 0.0, 1.0); // Not enough data
        }
    
        let normalized_samples: Vec<f32> = get_floats(samples);
    
        if normalized_samples.is_empty() {
            return (-1.0, 0.0, 1.0);
        }
    
        let mut sum_y = 0.0;
        let count = normalized_samples.len() as f64;
    
        for &x in &normalized_samples {
            let y = (1.0 - x.powi(2)).sqrt();
            sum_y += y as f64;
        }
    
        let mean_y = sum_y / count;
        let test_statistic = 4.0 * mean_y;
        let variance = compute_variance(count);
        let std_dev = variance.sqrt();
        let z_score = (test_statistic - std::f64::consts::PI) / std_dev;
    
        let normal_dist = Normal::new(0.0, 1.0).expect("Failed to create Normal distribution");
        let p_value = 2.0 * (1.0 - normal_dist.cdf(z_score.abs()));
    
        (test_statistic, z_score, p_value)
    }
    
}

fn get_floats(samples: &[u8]) -> Vec<f32> {
    let mut floats = Vec::new();
    for chunk in samples.chunks_exact(4) {
        let int_val = i32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        let unsigned_val = (int_val as u32) >> 1; // Discard sign bit
        let normalized = unsigned_val as f32 / i32::MAX as f32;
        floats.push(normalized);
    }
    floats
}

fn compute_variance(n: f64) -> f64 {
    let term = (2.0 / 3.0) - (std::f64::consts::PI / 4.0).powi(2);
    (16.0 / n) * term
}