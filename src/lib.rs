// MIT License

// Copyright (c) 2025 Tobias Raayoni Last (@randogoth)

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// ---

// This project is a Rust port of the original Java implementation by Paul Uszak.
// Original work is licensed under the terms specified in the respective file header.


pub struct Onod;

mod uniformity;
pub mod chisquaretest;
pub mod well19937c;
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