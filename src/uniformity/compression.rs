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


use flate2::{write::DeflateEncoder, Compression};
use std::io::Write;
use statrs::distribution::{Normal, ContinuousCDF};

use crate::Onod;

impl Onod {

    /// Compression randomness test
    /// Estimates randomness by the compressibility of the data and returns a p-value.
    pub fn compression(samples: &[u8]) -> (f64, f64, f64) {
        if samples.is_empty() {
            return (-1.0, 0.0, 1.0); // Perfect randomness for empty data
        }

        // Compress the data using deflate
        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
        if encoder.write_all(samples).is_err() {
            return (-1.0, 0.0, 1.0); // Compression failed
        }
        let compressed_data = match encoder.finish() {
            Ok(data) => data,
            Err(_) => return (-1.0, 0.0, 1.0), // Compression failed
        };

        // Calculate compression ratio
        let original_size = samples.len() as f64;
        let compressed_size = compressed_data.len() as f64;
        let compression_ratio = compressed_size / original_size;

        // Z-score calculation
        let expected_mean = 1.0;
        let std_dev = 0.002;
        let z_score = (compression_ratio - expected_mean) / std_dev;

        // Calculate p-value using the normal distribution
        let normal_dist = Normal::new(0.0, 1.0).expect("Failed to create Normal distribution");
        let p_value = 2.0 * (1.0 - normal_dist.cdf(z_score.abs())); // Two-tailed test

        (compression_ratio, z_score, p_value)
    }
}