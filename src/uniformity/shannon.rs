use statrs::distribution::{Normal, ContinuousCDF};

use crate::Onod;

impl Onod {
    /// Entropy randomness test
    /// Calculates the Shannon entropy of a byte slice and outputs a p-value.
    pub fn shannon(samples: &[u8]) -> f64 {

        let len = samples.len() as f64;
        if len == 0.0 {
            return 0.0;
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

        p_value
    }
}
