use statrs::distribution::{ChiSquared, ContinuousCDF};

use crate::Onod;

impl Onod {

    /// RunUps randomness test
    /// Evaluates the number of four-byte run-ups in the data and returns a p-value.
    pub fn run_ups(input: &[u8]) -> f64 {

        let samples = input.iter().map(|&x| x as u32).collect::<Vec<u32>>();

        if samples.len() < 4 {
            return 0.0; // Not enough data for meaningful calculation
        }

        let mut test_statistic = 0;

        for chunk in samples.chunks(4) {
            if let [first, second, third, fourth] = chunk {
                if first < second && second < third && third < fourth {
                    test_statistic += 1;
                }
            }
        }

        let total_chunks = samples.len() / 4;
        let no_expected = 2_731_135.0 / 67_108_864.0 * total_chunks as f64;

        let observed = [test_statistic as f64, total_chunks as f64 - test_statistic as f64];
        let expected = [no_expected, total_chunks as f64 - no_expected];

        // Use chi-squared test to calculate p-value
        let chi_squared_dist = ChiSquared::new(1.0).expect("Failed to create ChiSquared distribution");
        let chi_squared_stat: f64 = observed.iter()
            .zip(expected.iter())
            .map(|(o, e)| (o - e).powi(2) / e)
            .sum();

        let p_value = 1.0 - chi_squared_dist.cdf(chi_squared_stat);

        p_value
    }
}