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