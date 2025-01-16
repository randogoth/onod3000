use statrs::distribution::{Normal, ContinuousCDF};

use crate::Onod;

impl Onod {

    /// Pi randomness test
    /// Uses a Monte Carlo simulation to estimate randomness by calculating the approximation of Pi.
    /// This implementation of the Pi randomness test closely follows the logic of the original Java implementation.
    /// However, minor differences in the results may arise due to the following reasons:
    /// 
    /// 1. **Floating-Point Precision**:
    ///    Rust and Java both use 64-bit floating-point numbers (`double` in Java, `f64` in Rust), but slight differences 
    ///    in their implementations (e.g., rounding modes, intermediate representations) can lead to small deviations.
    ///
    /// 2. **Math Libraries**:
    ///    Java uses Apache Commons Math for statistical computations, which may implement certain calculations 
    ///    (e.g., Z-scores and normal distribution CDFs) differently compared to the `statrs` crate used in Rust.
    ///
    /// 3. **Bit Accuracy**:
    ///    The Java implementation notes the significance of bit accuracy in floating-point computations, 
    ///    as defined in the IEEE 754 standard. Differences in handling edge cases (e.g., subnormal values, 
    ///    precision limits) could lead to slight variations.
    ///
    /// These differences are generally negligible for practical purposes and do not affect the overall functionality or 
    /// statistical significance of the test.
    pub fn pi(samples: &[u8]) -> f64 {
    
        if samples.is_empty() {
            return 0.0;
        }
    
        // Normalize samples to [0.0, 1.0)
        let normalized_samples: Vec<f64> = samples.iter().map(|&x| x as f64 / 255.0).collect();
    
        // Initialize variables for summary statistics
        let mut sum_y = 0.0;
        let mut count = 0.0;
    
        // Compute y-values (sqrt(1 - x^2)) and update summary statistics
        for &x in &normalized_samples {
            let y = (1.0 - x * x).sqrt();
            sum_y += y;
            count += 1.0;
        }
    
        // Calculate mean of y-values
        let mean_y = sum_y / count;
    
        // Calculate the test statistic
        let test_statistic = 4.0 * mean_y;
    
        // Calculate variance and standard deviation
        let variance = (16.0 / count) * ((2.0 / 3.0) - (std::f64::consts::PI / 4.0).powi(2));
        let std_dev = variance.sqrt();
    
        // Calculate Z-score
        let z_score = (test_statistic - std::f64::consts::PI) / std_dev;
    
        // Use normal distribution to calculate p-value
        let normal_dist = Normal::new(0.0, 1.0).expect("Failed to create Normal distribution");
        let p_value = 2.0 * (1.0 - normal_dist.cdf(z_score.abs()));
    
        p_value
    }

}