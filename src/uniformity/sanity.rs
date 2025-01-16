use crate::Onod;

impl Onod {

    /// Sanity randomness test
    /// Checks for basic properties of randomness and returns a p-value.
    pub fn sanity(samples: &[u8]) -> f64 {
        if samples.is_empty() {
            return 0.0;
        }

        // Check the proportion of ones and zeros in the byte data
        let mut one_bits = 0;
        let mut total_bits = 0;

        for &byte in samples {
            one_bits += byte.count_ones();
            total_bits += 8;
        }

        let observed_ratio = one_bits as f64 / total_bits as f64;
        let expected_ratio = 0.5; // Expected for a truly random sequence
        let std_dev = (0.5 * 0.5 / total_bits as f64).sqrt(); // Standard deviation for binomial distribution

        // Use normal distribution to calculate p-value
        use statrs::distribution::{Normal, ContinuousCDF};
        let normal_dist = Normal::new(expected_ratio, std_dev).expect("Failed to create Normal distribution");
        let p_value = 1.0 - normal_dist.cdf(observed_ratio);

        p_value
    }

}