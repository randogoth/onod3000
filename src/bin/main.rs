use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use statrs::distribution::{Normal, ContinuousCDF};

use onod3000::Onod;

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next(); // Skip program name

    let mut input_data = Vec::new();
    let mut source = String::from("stdin");

    if let Some(arg) = args.next() {
        if arg == "-f" {
            if let Some(file_path) = args.next() {
                source = file_path.clone();
                let path = Path::new(&file_path);
                let mut file = File::open(path)?;
                file.read_to_end(&mut input_data)?;
            } else {
                eprintln!("Error: No file path provided after -f.");
                std::process::exit(1);
            }
        } else {
            eprintln!("Usage: {} [-f <file_path>]", std::env::args().next().unwrap());
            std::process::exit(1);
        }
    } else {
        io::stdin().read_to_end(&mut input_data)?;
    }

    if input_data.is_empty() {
        eprintln!("Error: No input data provided.");
        std::process::exit(1);
    }

    println!("\nTesting {}", source);
    println!("Testing {} bytes.", input_data.len());
    println!("--------------------------------------");

    let alpha = 0.01;
    let mut passed_tests = 0;

    // Run each test with the appropriate handling
    let tests = [
        ("Shannon",       Onod::shannon(&input_data)),
        ("Sanity",        Onod::sanity(&input_data)),
        ("Monobit",       Onod::monobit(&input_data)),
        ("ChiBit",        Onod::chi_bit(&input_data)),
        ("ChiByte",       Onod::chi_byte(&input_data)),
        ("MeanByte",      Onod::mean_byte(&input_data)),
        ("Compression",   Onod::compression(&input_data)),
        ("KS",            Onod::ks(&input_data)),
        ("Pi",            Onod::pi(&input_data)),
        ("Shells",        Onod::shells(&input_data)),
        ("Gaps",          Onod::gaps(&input_data)),
        ("Avalanche",     Onod::avalanche(&input_data)),
        ("Runs",          Onod::runs(&input_data)),
        ("RunUps",        Onod::run_ups(&input_data)),
        ("Prediction",    Onod::prediction(&input_data)),
        ("UnCorrelation", Onod::un_correlation(&input_data)),
    ];

    let mut p_values = Vec::new();

    for (test_name, p_value) in tests.iter() {
        let result = if *p_value >= alpha {
            passed_tests += 1;
            "PASS"
        } else {
            "FAIL"
        };
    
        println!("{:<20} p = {:.4},  {}", test_name, p_value, result);
    
        p_values.push(*p_value); // Dereference the value and push it
    }    

    // Calculate combined p-value using Fisher's method
    let (combined_z_score, combined_p_value) = combined_score_stouffer(&p_values);
    let overall_result = if combined_p_value >= alpha { "PASS" } else { "FAIL" };

    println!("--------------------------------------");
    println!("{}/{} tests passed.", passed_tests, tests.len());
    println!("--------------------------------------");
    println!("Combined Z-Score = {:.6}\nCombined P-Value = {:.6}\nOverall Result: {}", combined_z_score.abs(), combined_p_value, overall_result);

    Ok(())
}

pub fn combined_score_stouffer(p_values: &[f64]) -> (f64, f64) {
    let normal_dist = Normal::new(0.0, 1.0).expect("Failed to create Normal distribution");

    // Calculate Z-scores for each p-value
    let z_scores: Vec<f64> = p_values
        .iter()
        .map(|&p| normal_dist.inverse_cdf(1.0 - p))
        .collect();

    // Combine Z-scores
    let combined_z = z_scores.iter().sum::<f64>() / (p_values.len() as f64).sqrt();

    // Convert combined Z-score back to a p-value
    let p_value = 2.0 * (1.0 - normal_dist.cdf(combined_z.abs())); // Two-tailed

    (combined_z, p_value)
}