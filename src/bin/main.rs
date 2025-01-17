use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

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

    println!("\nTesting {} bytes from {}.", input_data.len(), source);
    println!("-----------------------------------------------------");
    println!("Randomness Test     Observation  Z-Score P-Value Pass");
    println!("-----------------------------------------------------");

    let mut passed_tests = 0;
    let alpha = 0.01;

    // Create vectors to store results
    let (mut observations, mut z_scores, mut p_values) = (Vec::new(), Vec::new(), Vec::new());

    // Run each test with the appropriate handling
    let tests = [
        ("Shannon",       Onod::shannon(&input_data)),
        ("Sanity",        Onod::sanity(&input_data)),
        ("Monobit",       Onod::monobit(&input_data)),
        ("ChiBit",        Onod::chi_bit(&input_data)),
        ("ChiByte",       Onod::chi_byte(&input_data)),
        ("MeanByte",      Onod::mean_byte(&input_data)),
        ("Compression",   Onod::compression(&input_data)),
        ("Kolm.-Smirnov", Onod::ks(&input_data)),
        ("Pi",            Onod::pi(&input_data)),
        ("Shells",        Onod::shells(&input_data)),
        ("Gaps",          Onod::gaps(&input_data)),
        ("Avalanche",     Onod::avalanche(&input_data)),
        ("Runs",          Onod::runs(&input_data)),
        ("RunUps",        Onod::run_ups(&input_data)),
        ("Prediction",    Onod::prediction(&input_data)),
        ("UnCorrelation", Onod::un_correlation(&input_data)),
    ];

    for (test_name, (observation, z_score, p_value)) in &tests {
        let result = if *p_value >= alpha && *observation != -1.0 {
            passed_tests += 1;
            "✅"
        } else {
            if *observation == -1.0 {
                "SKIP"
            } else {
                "❌"
            } 
        };

        println!(
            "{:<15} {:>15.3}  {:>8.4}  {:.4}  {}",
            test_name, observation, z_score, p_value, result
        );

        observations.push(*observation);
        z_scores.push(*z_score);
        p_values.push(*p_value);
    }

    println!("-----------------------------------------------------");
    println!("{}/{} tests passed.", passed_tests, tests.len());
    println!("-----------------------------------------------------");

    Ok(())
}