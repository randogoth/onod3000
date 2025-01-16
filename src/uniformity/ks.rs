use kolmogorov_smirnov::test_f64;

use crate::Onod;

impl Onod {

    /// KS randomness test
    /// Performs the Kolmogorov-Smirnov test to evaluate uniformity of data distribution and returns a p-value.
    // Note: This implementation of the Kolmogorov-Smirnov (KS) test differs slightly from the
    // Java implementation due to differences in library behavior and floating-point handling.
    // The Java implementation (Apache Commons Math) adds random jitter to handle ties in small
    // datasets, uses strict inequality for small sample sizes, and applies specific precision
    // rules based on the IEEE 754 standard for `double`. The Rust implementation, using the
    // `kolmogorov_smirnov` crate, does not add jitter or handle ties in the same way, and
    // adheres to the crate's internal handling of floating-point comparisons. These differences
    // may result in slight variations in p-values or KS statistics between the two versions.
    pub fn ks(samples: &[u8]) -> f64 {

        if samples.is_empty() {
            return 0.0; // empty data
        }

        // Normalize the input samples to [0, 1] range
        let normalized_samples: Vec<f64> = samples.iter().map(|&x| x as f64 / 255.0).collect();

        // Generate a uniform distribution for comparison
        let uniform_distribution: Vec<f64> = (0..normalized_samples.len())
            .map(|i| i as f64 / (normalized_samples.len() as f64 - 1.0))
            .collect();

        // Perform the Kolmogorov-Smirnov test
        let confidence = 0.01; // Significance level
        let result = test_f64(&normalized_samples, &uniform_distribution, confidence);

        // Extract p-value from the result
        let p_value = 1.0 - result.reject_probability;

        p_value
    }
}