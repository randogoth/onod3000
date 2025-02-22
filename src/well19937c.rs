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


pub struct Well19937c {
    state: [u32; 624],
    index: usize,
}

impl Well19937c {
    /// Creates a new instance of Well19937c with a given seed.
    pub fn new(seed: u32) -> Self {
        let mut state = [0u32; 624];
        state[0] = seed;

        for i in 1..624 {
            state[i] = 1812433253u32
                .wrapping_mul(state[i - 1] ^ (state[i - 1] >> 30))
                .wrapping_add(i as u32);
        }

        Well19937c { state, index: 0 }
    }

    /// Updates the internal state.
    fn twist(&mut self) {
        const M: usize = 397;
        const MATRIX_A: u32 = 0x9908b0df; // Constant matrix A
        const UPPER_MASK: u32 = 0x80000000; // Most significant w-r bits
        const LOWER_MASK: u32 = 0x7fffffff; // Least significant r bits

        for i in 0..624 {
            let x = (self.state[i] & UPPER_MASK) + (self.state[(i + 1) % 624] & LOWER_MASK);
            let mut x_a = x >> 1;

            if x % 2 != 0 {
                x_a ^= MATRIX_A;
            }

            self.state[i] = self.state[(i + M) % 624] ^ x_a;
        }

        self.index = 0;
    }

    /// Generates the next random number in the sequence.
    pub fn next_u32(&mut self) -> u32 {
        if self.index == 0 {
            self.twist();
        }

        let mut y = self.state[self.index];
        self.index = (self.index + 1) % 624;

        // Matsumoto-Kurita tempering
        y ^= (y << 7) & 0xe46e1700;
        y ^= (y << 15) & 0x9b868000;

        y
    }

    /// Generates the next random `f64` in [0, 1).
    pub fn next_f64(&mut self) -> f64 {
        self.next_u32() as f64 / u32::MAX as f64
    }
}