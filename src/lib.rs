pub struct Onod;

mod uniformity;
pub mod ffi;
#[cfg(feature = "python")]
pub mod python;

impl Onod {
    pub fn run(test: &str, samples: &[u8]) -> (f64, f64, f64) {

        match test {
            "avalanche"     => Onod::avalanche(samples),
            "chi_bit"       => Onod::chi_bit(samples),
            "chi_byte"      => Onod::chi_byte(samples),
            "compression"   => Onod::compression(samples),
            "gaps"          => Onod::gaps(samples),
            "ks"            => Onod::ks(samples),
            "mean_byte"     => Onod::mean_byte(samples),
            "monobit"       => Onod::monobit(samples),
            "pi"            => Onod::pi(samples),
            "prediction"    => Onod::prediction(samples),
            "runs"          => Onod::runs(samples),
            "run_ups"       => Onod::run_ups(samples),
            "shannon"       => Onod::shannon(samples),
            "shells"        => Onod::shells(samples),
            "uncorrelation" => Onod::uncorrelation(samples),
            _ => {
                eprintln!("Error: Unknown test '{}'", test);
                (-1.0, 0.0, 0.0)
            }
        }
    }
}
