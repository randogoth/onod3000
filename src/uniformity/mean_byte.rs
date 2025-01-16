use statrs::distribution::{Normal, ContinuousCDF};

use crate::Onod;

impl Onod {

    /// Mean randomness test
    /// Calculates the p-value for the mean of the byte slice compared to expected mean.
    pub fn mean_byte(samples: &[u8]) -> f64 {
    
        let len = samples.len() as f64;
        if len == 0.0 {
            return 0.0;
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
    
        p_value
    }
}