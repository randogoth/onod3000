use statrs::distribution::{ChiSquared, ContinuousCDF};

use crate::Onod;

impl Onod {

    pub fn chi_bit(samples: &[u8]) -> f64 {
    
        if samples.is_empty() {
            return 0.0; // empty data
        }
    
        // Lookup table for the number of set bits in each byte (Hamming weight)
        const SET_BITS_PER_BYTE: [usize; 256] = [
            0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 1, 2, 2, 3,
            2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 1, 2, 2, 3, 2, 3, 3, 4,
            2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5,
            4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
            2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5,
            4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6,
            4, 5, 5, 6, 5, 6, 6, 7, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7,
            6, 7, 7, 8,
        ];
    
        // Expected number of occurrences for each Hamming weight
        const EXPECTED_NUMBER: [f64; 9] = [1.0, 8.0, 28.0, 56.0, 70.0, 56.0, 28.0, 8.0, 1.0];
    
        // Frequency count of Hamming weights
        let mut frequency = vec![0; EXPECTED_NUMBER.len()];
        for &byte in samples {
            let hamming_weight = SET_BITS_PER_BYTE[byte as usize];
            frequency[hamming_weight] += 1;
        }
    
        // Calculate observed and expected counts
        let total_samples = samples.len() as f64;
        let expected: Vec<f64> = EXPECTED_NUMBER
            .iter()
            .map(|&e| e / 256.0 * total_samples)
            .collect();
    
        // Chi-squared statistic calculation
        let chi_squared_stat = frequency
            .iter()
            .zip(expected.iter())
            .map(|(&observed, &expected)| {
                if expected > 0.0 {
                    (observed as f64 - expected).powi(2) / expected
                } else {
                    0.0
                }
            })
            .sum();
    
        // Degrees of freedom: 9 categories - 1
        let degrees_of_freedom = (EXPECTED_NUMBER.len() - 1) as f64;
        let chi_squared_dist = ChiSquared::new(degrees_of_freedom).expect("Failed to create ChiSquared distribution");
        let p_value = 1.0 - chi_squared_dist.cdf(chi_squared_stat);
    
        p_value
    }   
}