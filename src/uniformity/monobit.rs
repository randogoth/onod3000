use statrs::distribution::{Normal, ContinuousCDF};

use crate::Onod;

impl Onod {

    /// Monobit randomness test
    /// Evaluates the balance of 0s and 1s in the binary representation of the data and returns a p-value.
    pub fn monobit(samples: &[u8]) -> f64 {  

        if samples.is_empty() {
            return 0.0; // Default to perfect randomness for empty data
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

        p_value
    }
}