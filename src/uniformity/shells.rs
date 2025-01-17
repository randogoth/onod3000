use statrs::distribution::{ChiSquared, ContinuousCDF};

use crate::Onod;

impl Onod {

    /// Shells randomness test
    /// Evaluates the uniformity of distances between identical byte values and returns a p-value.
    pub fn shells(input: &[u8]) -> (f64, f64, f64) {
    
        // Define shell radii (precomputed to ensure equal volumes)
        const SHELL_RADII: [f64; 35] = [
            1., 0.990384019787941, 0.980577593308067, 0.970571001281035, 0.960353705642329, 
            0.949914251592996, 0.939240154232372, 0.928317766722556, 0.91713212619864, 
            0.905666772691187, 0.893903535096568, 0.881822276616739, 0.869400589952457, 
            0.856613429672063, 0.843432665301749, 0.829826533366243, 0.815758959214771, 
            0.801188709029197, 0.786068317431936, 0.770342714221672, 0.753947441129154, 
            0.736806299728077, 0.718828193851318, 0.699902804775202, 0.67989452969576, 
            0.65863375600835, 0.635903899768996, 0.61142141746576, 0.584803547642573, 
            0.555513224287824, 0.52275795857471, 0.485285500640517, 0.440911138308369, 
            0.385171357110836, 0.30571070873288
        ];

        let samples = convert_to_3d_points(input);
    
        if samples.len() < 25000 {
            // eprintln!("---------------------------------------------------------------");
            // eprintln!("ERROR: Shells test requires at least 25,000 points for statistical validity. Skipping.");
            // eprintln!("---------------------------------------------------------------");
            return (-1.0, 0.0, 1.0); // Skip the test for small datasets
        }
    
        let sphere_radius = SHELL_RADII[0];
        let no_shells = SHELL_RADII.len();
    
        // Calculate sphere and cube volume proportions
        let cube_side = 2.0 * sphere_radius;
        let cube_volume = cube_side.powi(3);
        let sphere_volume = (4.0 / 3.0) * std::f64::consts::PI * sphere_radius.powi(3);
        let sphere_proportion = sphere_volume / cube_volume; // Theoretical value: π/6
    
        let no_points = samples.len() as f64;
        let no_points_per_shell = sphere_proportion * no_points / no_shells as f64;
    
        let mut observed = vec![0u64; no_shells];
        let expected: Vec<f64> = vec![no_points_per_shell; no_shells];
    
        for (x, y, z) in samples {
            // Compute radius from origin
            let radius = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
    
            // Ignore points outside the sphere
            if radius > sphere_radius {
                continue;
            }
    
            // Assign to the correct shell
            for j in 1..SHELL_RADII.len() {
                if radius > SHELL_RADII[j] {
                    observed[j - 1] += 1;
                    break;
                }
            }
    
            if radius < SHELL_RADII[no_shells - 1] {
                observed[no_shells - 1] += 1;
            }
        }
    
        // Perform Chi-Square Test
        let chi_squared_stat: f64 = observed.iter()
            .zip(expected.iter())
            .map(|(&o, &e)| (o as f64 - e).powi(2) / e)
            .sum();
    
        let degrees_of_freedom = no_shells as f64 - 1.0;
        let chi_squared_dist = ChiSquared::new(degrees_of_freedom).expect("Failed to create ChiSquared distribution");
        let p_value = 1.0 - chi_squared_dist.cdf(chi_squared_stat);
    
        // Z-score calculation (standardization of the chi-squared statistic)
        let mean = degrees_of_freedom; // Mean of the chi-squared distribution
        let std_dev = (2.0 * degrees_of_freedom).sqrt(); // Standard deviation of the chi-squared distribution
        let z_score = (chi_squared_stat - mean) / std_dev;

        (chi_squared_stat, z_score, p_value)
    }    
}

fn convert_to_3d_points(data: &[u8]) -> Vec<(f64, f64, f64)> {
    let mut points = Vec::new();

    for chunk in data.chunks(3) {
        if chunk.len() == 3 {
            // Normalize the bytes to [0.0, 1.0) range
            let x = chunk[0] as f64 / 255.0;
            let y = chunk[1] as f64 / 255.0;
            let z = chunk[2] as f64 / 255.0;
            points.push((x, y, z));
        }
    }

    points
}