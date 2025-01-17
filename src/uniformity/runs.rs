use statrs::distribution::{Normal, ContinuousCDF};

use crate::Onod;

impl Onod {

    /// Runs randomness test
    /// Evaluates the randomness by counting both increasing and decreasing runs and returns a p-value.
    pub fn runs(samples: &[u8]) -> (f64, f64, f64) {

        if samples.is_empty() {
            return (-1.0, 0.0, 1.0);
        }

        // Clone the samples to avoid modifying the original input
        let samples = samples.to_vec();
        let median = calculate_median(&samples);

        let mut above = 0;
        let mut below = 0;

        // Transform the data into a dichotomous vector and count above/below values
        let mut purged_samples = Vec::new();
        for &sample in &samples {
            if (sample as f64) > median {
                purged_samples.push(1); // Mark as above
                above += 1;
            } else if (sample as f64) < median {
                purged_samples.push(0); // Mark as below
                below += 1;
            }
        }

        // Count runs
        let mut runs_observed = 1; // At least one run exists
        for window in purged_samples.windows(2) {
            if window[0] != window[1] {
                runs_observed += 1;
            }
        }

        // Calculate expected runs and standard deviation
        let runs_expected = ((2.0 * above as f64 * below as f64) / (above + below) as f64) + 1.0;
        let std_dev = ((2.0 * above as f64 * below as f64 * (2.0 * above as f64 * below as f64 - above as f64 - below as f64))
            / (((above + below) as f64).powi(2) * (above + below - 1) as f64))
            .sqrt();

        // Calculate Z-score
        let z_score = (runs_observed as f64 - runs_expected) / std_dev;

        // Use normal distribution to calculate p-value
        let normal_dist = Normal::new(0.0, 1.0).expect("Failed to create Normal distribution");
        let p_value = 2.0 * (1.0 - normal_dist.cdf(z_score.abs()));

        if p_value.is_nan() {
            return (-1.0, 0.0, 1.0); // Return 0 if p-value is NaN
        }

        (runs_observed as f64, z_score, p_value)
    }
}

/// Helper function to calculate the median of a dataset
fn calculate_median(samples: &[u8]) -> f64 {
    let mut sorted_samples = samples.to_vec();
    sorted_samples.sort_unstable();

    let len = sorted_samples.len();
    if len % 2 == 0 {
        (sorted_samples[len / 2 - 1] as f64 + sorted_samples[len / 2] as f64) / 2.0
    } else {
        sorted_samples[len / 2] as f64
    }
}