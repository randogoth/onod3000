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


use kolmogorov_smirnov::test_f64;
use std::time::{SystemTime, UNIX_EPOCH};
use statrs::distribution::ContinuousCDF;

use crate::Onod;
use crate::well19937c::Well19937c;

impl Onod {
    /// KS randomness test
    /// Performs the Kolmogorov-Smirnov test to evaluate the uniformity of data distribution
    /// and returns the test statistic (D-statistic), z-score, and p-value.
    pub fn ks(samples: &[u8]) -> (f64, f64, f64) {
        if samples.is_empty() {
            return (-1.0, 0.0, 1.0); // Invalid input
        }
    
        // Normalize the input samples to [0, 1) range
        let mut normalized_samples: Vec<f64> = samples.iter().map(|&x| x as f64 / 255.0).collect();
    
        // Generate a uniform distribution for comparison
        let mut uniform_distribution: Vec<f64> = Self::generate_uniform_distribution(normalized_samples.len(), Self::get_timestamp_seed());
    
        // Sort both distributions
        normalized_samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
        uniform_distribution.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
        let debug = false;
        // Debugging: Optional print sorted values
        if debug {
            println!("Sorted Normalized Samples: {:?}", normalized_samples);
            println!("Sorted Uniform Distribution: {:?}", uniform_distribution);
        }
    
        // Perform the Kolmogorov-Smirnov test
        let confidence = 0.05; // Significance level
        let result = test_f64(&normalized_samples, &uniform_distribution, confidence);
    
        // Extract the KS statistic (D-statistic)
        let ks_statistic = result.statistic;
    
        // Debugging: Print ECDF differences and max difference (D-statistic)
        if debug {
            for (i, (&sample, &uniform)) in normalized_samples.iter().zip(&uniform_distribution).enumerate() {
                let diff = (sample - uniform).abs();
                println!(
                    "Index: {}, Sample: {:.6}, Uniform: {:.6}, Difference: {:.6}",
                    i, sample, uniform, diff
                );
            }
            println!("D-Statistic: {:.6}", ks_statistic);
        }
    
        // Calculate the z-score
        let sample_size = normalized_samples.len() as f64;
        let z_score = ks_statistic * sample_size.sqrt();
    
        // Calculate the p-value
        let p_value = 2.0 * (1.0 - statrs::distribution::Normal::new(0.0, 1.0).unwrap().cdf(z_score.abs()));
    
        (ks_statistic, z_score, p_value)
    }

    /// Generates a uniform distribution of the same length as the input data.
    fn generate_uniform_distribution(len: usize, seed: u32) -> Vec<f64> {
        let mut rng = Well19937c::new(seed);
        (0..len).map(|_| rng.next_f64()).collect()
    }

    fn get_timestamp_seed() -> u32 {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        // Use seconds or nanoseconds as the seed
        (duration.as_secs() as u32) ^ (duration.subsec_nanos())
    }
}