use flate2::{write::DeflateEncoder, Compression};
use std::io::Write;
use statrs::distribution::{Normal, ContinuousCDF};

use crate::Onod;

impl Onod {

    /// Compression randomness test
    /// Estimates randomness by the compressibility of the data and returns a p-value.
    pub fn compression(samples: &[u8]) -> f64 {
        if samples.is_empty() {
            return 0.0; // Perfect randomness for empty data
        }

        // Compress the data using deflate
        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
        if encoder.write_all(samples).is_err() {
            return 0.0; // Compression failed
        }
        let compressed_data = match encoder.finish() {
            Ok(data) => data,
            Err(_) => return 0.0, // Compression failed
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

        p_value
    }
}