use statrs::distribution::{Normal, ContinuousCDF};

use crate::Onod;

impl Onod {

    /// UnCorrelation randomness test
    /// Computes the Pearson correlation between the sequence and its shifted version, returning a p-value.
    pub fn uncorrelation(input: &[u8]) -> (f64, f64, f64) {

        let samples = input.iter().map(|&x| x as i32).collect::<Vec<i32>>();

        if samples.len() < 2 {
            return (-1.0, 0.0, 1.0); // Default to perfect randomness for insufficient data
        }

        // Convert samples to f64 for correlation computation
        let samples_a: Vec<f64> = samples.iter().map(|&x| x as f64).collect();

        // Create a shifted version of the sequence
        let mut samples_b = vec![0.0; samples.len()];
        samples_b[0] = samples_a[samples.len() - 1]; // Wrap around
        for i in 1..samples.len() {
            samples_b[i] = samples_a[i - 1];
        }

        // Calculate mean of both sequences
        let mean_a = samples_a.iter().sum::<f64>() / samples_a.len() as f64;
        let mean_b = samples_b.iter().sum::<f64>() / samples_b.len() as f64;

        // Compute Pearson correlation coefficient
        let mut numerator = 0.0;
        let mut denominator_a = 0.0;
        let mut denominator_b = 0.0;

        for i in 0..samples.len() {
            let diff_a = samples_a[i] - mean_a;
            let diff_b = samples_b[i] - mean_b;
            numerator += diff_a * diff_b;
            denominator_a += diff_a.powi(2);
            denominator_b += diff_b.powi(2);
        }

        let correlation = numerator / (denominator_a.sqrt() * denominator_b.sqrt());

        // Calculate p-value for null hypothesis of zero correlation
        let n = samples.len() as f64;
        let t_stat = correlation * ((n - 2.0) / (1.0 - correlation.powi(2))).sqrt();

        // Use t-distribution approximation for large n
        let normal_dist = Normal::new(0.0, 1.0).expect("Failed to create Normal distribution");
        let p_value = 2.0 * (1.0 - normal_dist.cdf(t_stat.abs()));

        (correlation, t_stat, p_value)
    }

}