use crate::Onod;
use crate::chisquaretest::chi_square_test;

impl Onod {
    /// Shells randomness test
    /// Evaluates the uniformity of distances in a 3D sphere and returns the chi-squared statistic, z-score, and p-value.
    pub fn shells(input: &[u8]) -> (f64, f64, f64) {
        
        // Define shell radii (precomputed to ensure equal volumes)
        const SHELL_RADII: [f64; 35] = [
            1., 0.990384019787941, 0.980577593308067,
            0.970571001281035, 0.960353705642329, 0.949914251592996,
            0.939240154232372, 0.928317766722556, 0.91713212619864,
            0.905666772691187, 0.893903535096568, 0.881822276616739,
            0.869400589952457, 0.856613429672063, 0.843432665301749,
            0.829826533366243, 0.815758959214771, 0.801188709029197,
            0.786068317431936, 0.770342714221672, 0.753947441129154,
            0.736806299728077, 0.718828193851318, 0.699902804775202,
            0.67989452969576, 0.65863375600835, 0.635903899768996,
            0.61142141746576, 0.584803547642573, 0.555513224287824,
            0.52275795857471, 0.485285500640517, 0.440911138308369,
            0.385171357110836, 0.30571070873288,
        ];

        let samples = convert_to_3d_points(&input);
        let sphere_radius = SHELL_RADII[0];
        let num_shells = SHELL_RADII.len();

        // Calculate sphere and cube proportions
        let cube_side = 2.0 * sphere_radius;
        let cube_volume = cube_side.powi(3);
        let sphere_volume = (4.0 / 3.0) * std::f64::consts::PI * sphere_radius.powi(3);
        let sphere_proportion = sphere_volume / cube_volume;

        let num_points = samples.len() as f64;
        let num_points_per_shell = sphere_proportion * num_points / num_shells as f64;

        // Initialize observed and expected frequencies
        let mut observed = vec![0u64; num_shells];
        let expected: Vec<f64> = vec![num_points_per_shell; num_shells];

        // Assign points to shells
        for (x, y, z) in samples {
            let radius = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();

            // Skip points outside the sphere
            if radius > sphere_radius {
                continue;
            }

            for j in 1..num_shells {
                if radius > SHELL_RADII[j] {
                    observed[j - 1] += 1;
                    break;
                }
            }
            
            // Assign to the last shell if radius <= SHELL_RADII[num_shells - 1]
            if radius < SHELL_RADII[num_shells - 1] {
                observed[num_shells - 1] += 1;
            }
            
        }

        // Calculate chi-squared statistic
        let chi_squared_stat: f64 = observed
            .iter()
            .zip(expected.iter())
            .map(|(&o, &e)| (o as f64 - e).powi(2) / e)
            .sum();

        // Perform chi-squared test
        let degrees_of_freedom = num_shells as f64 - 1.0;
        let p_value = chi_square_test(&observed, &expected);

        // Calculate z-score
        let mean = degrees_of_freedom;
        let std_dev = (2.0 * degrees_of_freedom).sqrt();
        let z_score = (chi_squared_stat - mean) / std_dev;

        // Return the results
        (chi_squared_stat, z_score, p_value)
    }
}

fn convert_to_3d_points(data: &[u8]) -> Vec<(f64, f64, f64)> {
    let mut points = Vec::new();

    for chunk in data.chunks(12) {
        if chunk.len() == 12 {
            let x = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) >> 1;
            let y = u32::from_be_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]) >> 1;
            let z = u32::from_be_bytes([chunk[8], chunk[9], chunk[10], chunk[11]]) >> 1;

            points.push((
                x as f64 / (i32::MAX as f64),
                y as f64 / (i32::MAX as f64),
                z as f64 / (i32::MAX as f64),
            ));
        }
    }

    points
}