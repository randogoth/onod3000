use statrs::distribution::{ChiSquared, ContinuousCDF};

use crate::Onod;

impl Onod {

    /// Gaps randomness test
    /// Analyzes the gaps between occurrences of a specific byte value and returns a p-value.
    pub fn gaps(samples: &[u8]) -> (f64, f64, f64) {
    
        if samples.is_empty() {
            return (-1.0, 0.0, 1.0); // empty data
        }
    
        // Bin edges and expected frequencies from the Java implementation
        let bin_edges = [0, 26, 56, 90, 130, 176, 233, 307, 410, 587, 1_000_000_000];
        let expected_frequencies = [
            0.10028324483130746,
            0.09967572210885733,
            0.09968386797087492,
            0.10149320063058609,
            0.09867042665174708,
            0.10001818308995936,
            0.10062746966933134,
            0.09938275862858448,
            0.10004382368984566,
            0.1001213027289063,
        ];
    
        let mut all_gaps = Vec::new();
    
        // Measure gaps for all unique values
        for category_pointer in 0..samples.len() - 1 {
            for i in category_pointer + 1..samples.len() {
                if samples[i] == samples[category_pointer] {
                    let gap = i - category_pointer - 1;
                    all_gaps.push(gap as u32);
                    break;
                }
            }
        }
    
        if all_gaps.is_empty() {
            return (-1.0, 0.0, 1.0); // No gaps found
        }
    
        // Create histogram of observed gaps
        let mut observed = vec![0; bin_edges.len() - 1];
        for &gap in &all_gaps {
            for i in 0..bin_edges.len() - 1 {
                if (gap as u32) <= bin_edges[i + 1] {
                    observed[i] += 1;
                    break;
                }
            }
        }
    
        // Calculate expected counts for each bin
        let total_gaps = all_gaps.len() as f64;
        let expected: Vec<f64> = expected_frequencies
            .iter()
            .map(|&freq| freq * total_gaps)
            .collect();
    
        // Perform chi-square test
        let chi_squared_stat: f64 = observed
            .iter()
            .zip(expected.iter())
            .map(|(&o, &e)| if e > 0.0 { (o as f64 - e).powi(2) / e } else { 0.0 })
            .sum();
    
        let degrees_of_freedom = bin_edges.len() as f64 - 2.0; // Number of bins - 1
        let chi_squared_dist = ChiSquared::new(degrees_of_freedom).expect("Failed to create ChiSquared distribution");
        let p_value = 1.0 - chi_squared_dist.cdf(chi_squared_stat);
    
        // Z-score calculation (standardization of the chi-squared statistic)
        let mean = degrees_of_freedom; // Mean of the chi-squared distribution
        let std_dev = (2.0 * degrees_of_freedom).sqrt(); // Standard deviation of the chi-squared distribution
        let z_score = (chi_squared_stat - mean) / std_dev;

        (chi_squared_stat, z_score, p_value)
    }    

}