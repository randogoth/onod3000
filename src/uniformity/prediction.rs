use statrs::distribution::{Normal, ContinuousCDF};

use crate::Onod;

impl Onod {

    /// Prediction randomness test
    /// Evaluates the predictability of the next bit based on current data and returns a p-value.
    pub fn prediction(samples: &[u8]) -> f64 {

        if samples.is_empty() {
            return 0.0;
        }

        let mut correct_predictions = 0;
        let mut total_predictions = 0;

        for window in samples.windows(2) {
            if let [current, next] = window {
                let predicted = if current & 0x01 == 0 { 0 } else { 1 }; // Predict next bit based on LSB
                let actual = next & 0x01; // Check LSB of the next byte

                if predicted == actual {
                    correct_predictions += 1;
                }

                total_predictions += 1;
            }
        }

        if total_predictions == 0 {
            return 0.0; // No predictions possible
        }

        // Calculate observed proportion of correct predictions
        let observed_proportion = correct_predictions as f64 / total_predictions as f64;

        // Expected proportion for random data
        let expected_proportion = 0.5;
        let std_dev = (0.5 * 0.5 / total_predictions as f64).sqrt(); // Standard deviation for a binomial distribution

        // Calculate the z-score
        let z_score = (observed_proportion - expected_proportion) / std_dev;

        // Use normal distribution to calculate p-value
        let normal_dist = Normal::new(0.0, 1.0).expect("Failed to create Normal distribution");
        let p_value = 2.0 * (1.0 - normal_dist.cdf(z_score.abs()));

        p_value
    }
}