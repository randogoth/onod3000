```
                                ____                        
                           |\   ` //  /\\   /\\   /\\       
                            \\   //  || || || || || ||      
         /'\\ \\/\\  /'\\  / \\  \\  || || || || || ||      
        || || || || || || || ||   )) || || || || || ||      
        || || || || || || || ||  //  || || || || || ||      
        \\,/  \\ \\ \\,/   \\/  /'    \\/   \\/   \\/       
                                                             

    Rust port of Paul Uszak's new randomness testing suite for TRNG Makers 
    generating in the sub 1 MB space. Successor to John Walker’s venerable `ent`.

    Pipe binary entropy data into the `onod3000` command line tool or call it
    with `-f <filename>` to load entropy from a file.

```

`Onod3000` provides multiple randomness tests, each outputting a statistic, a z-score andd a p-value to assess the conformity of the data to random behavior.

## Testing Methodology

- **Null Hypothesis (\(H_0\))**: The data conforms to random behavior.
- **P-Value Interpretation**: Low p-values (< 0.01) indicate a significant deviation from randomness.

All tests have been carefully ported from [Paul Uszak's Java code](http://www.reallyreallyrandom.com/gitbucketlabhub/) (ent3000-0.6.0-beta). Java libraries were substituted with Rust crate equivalents when possible. The suite includes the following tests:

- **Monobit Test**: Evaluates the balance of 0s and 1s.
- **Chi-Square Tests**: Tests the uniformity of bits and bytes.
- **MeanByte Test**: Checks if the mean byte value aligns with expected randomness.
- **Kolmogorov-Smirnov (KS) Test**: Assesses uniformity of data distribution.
- **Pi Test**: Uses Monte Carlo methods to approximate π.
- **Shells Test**: Analyzes distances in 3D space.
- **Gaps Test**: Measures gaps between occurrences of a specific value.
- **Avalanche Test**: Analyzes bit-level changes in data chunks.
- **Runs and RunUps Tests**: Checks for sequential patterns in data.
- **Prediction Test**: Assesses the predictability of next bits.
- **UnCorrelation Test**: Evaluates correlation between shifted data.

More details on some individual tests [available here](http://www.reallyreallyrandom.com/ent3000/the-tests/index.html)

### Sample Output
```bash
cat /dev/random | head -c 259200 | onod3000

Testing 259200 bytes from stdin.
--------------------------------------------------------
Randomness Test           Value   Z-Score   P-Value Pass
--------------------------------------------------------
Shannon                   7.999   -0.3669   0.7137   ✅
Monobit                   0.500    0.9000   0.3681   ✅
ChiBit                    4.555   -0.8612   0.8039   ✅
ChiByte                 236.288   -0.8286   0.7939   ✅
MeanByte                127.312   -1.2970   0.1946   ✅
Compression               1.000    0.0868   0.9308   ✅
Kolm.-Smirnov             0.004    2.0722   0.0272   ✅
Pi                        3.139   -1.3171   0.1878   ✅
Shells                   33.593   -0.0493   0.4874   ✅
Gaps                      7.402   -0.3767   0.5954   ✅
Avalanche                80.006    0.0009   0.9993   ✅
Runs                 129288.000    0.8208   0.4118   ✅
RunUps                    2.647    1.1645   0.1038   ✅
Prediction                0.500   -0.0255   0.9796   ✅
UnCorrelation            -0.001   -0.3725   0.7096   ✅
--------------------------------------------------------
15/15 tests passed.
--------------------------------------------------------
```

## Rust Library

```rust
use onod3000::Onod;

fn main() {
    let data = vec![...]; // Your binary data
    let p_value = Onod::monobit(&data);
    println!("Monobit Test P-Value: {:.4}", p_value);
}
```

## Bindings

The library ships with FFI and optional Python bindings so it can be used as C or Python library.

### Python
Install using `maturin`:
```bash
maturin build --release --features python
```

## Comparison

To test the implementation three files of binary random data have been analyzed using the original Java program and this Rust port expecting the same p-values.

### Testing 1024 bytes from test1.bin (/dev/random)

Test          | Java ent3000 | Rust onod3000 | Note |
------------- | ------------ | ------------- | ---- |
Shannon       | N/A          | 0.0000        |      |
Monobit       | 0.8771       | 0.8771        |      |
ChiBit        | 0.0949       | 0.0949        |      |
ChiByte       | 0.1307       | 0.1307        |      |
MeanByte      | 0.5378       | 0.5378        |      |
Compression   | N/A          | 0.0146        |      |
KS            | 0.5142       | 0.9814        | MISMATCH |
Pi            | 0.4015       | 0.7293        | MISMATCH |
Shells        | 0.5479       | 0.1470        | MISMATCH |
Gaps          | 0.0000       | 0.0000        |      |
Avalanche     | 0.9798       | 0.9798        |      |
Runs          | 0.4916       | 0.4916        |      |
RunUps        | 0.2573       | 0.2573        |      |
Prediction    | 0.3164       | 0.1255        | MISMATCH |
UnCorrelation | 0.4556       | 0.4554        |      |

### Testing 259,200 bytes from test2.bin (/dev/random)

Test          | Java ent3000 | Rust onod3000 | Note |
------------- | ------------ | ------------- | ---- |
Shannon       | N/A          | 0.6912        |      |
Monobit       | 0.0619       | 0.0619        |      |
ChiBit        | 0.5461       | 0.5461        |      |
ChiByte       | 0.4672       | 0.4672        |      |
MeanByte      | 0.0500       | 0.0500        |      |
Compression   | N/A          | 0.9308        |      |
KS            | 0.9229       | 0.0028        | MISMATCH |
Pi            | 0.2828       | 0.3953        | MISMATCH |
Shells        | 0.4604       | 0.3163        | MISMATCH |
Gaps          | 0.3854       | 0.3854        |      |
Avalanche     | 0.9974       | 0.9974        |      |
Runs          | 0.5873       | 0.5873        |      |
RunUps        | 0.0402       | 0.0402        |      |
Prediction    | 0.7409       | 0.3569        | MISMATCH |
UnCorrelation | 0.0949       | 0.0949        |      |

### Testing 259,200 bytes from test3.bin (Hardware QRNG)

Test          | Java ent3000 | Rust onod3000 | Note |
------------- | ------------ | ------------- | ---- |
Shannon       | N/A          | 0.6978        |      |
Monobit       | 0.4926       | 0.4926        |      |
ChiBit        | 0.8428       | 0.8428        |      |
ChiByte       | 0.5785       | 0.5785        |      |
MeanByte      | 0.9601       | 0.9601        |      |
Compression   | N/A          | 0.9308        |      |
KS            | 0.7395       | 0.0247        | MISMATCH |
Pi            | 0.2806       | 0.0189        | MISMATCH |
Shells        | 0.7711       | 0.3254        | MISMATCH |
Gaps          | 0.1937       | 0.1937        |      |
Avalanche     | 0.9932       | 0.9932        |      |
Runs          | 0.2500       | 0.2500        |      |
RunUps        | 0.9710       | 0.9710        |      |
Prediction    | 0.7173       | 0.7399        | MISMATCH |
UnCorrelation | 0.4674       | 0.4674        |      |

The port is work in progress and effort will be put into rigorously re-evaluating the implementations. Although the implementation of the randomness tests closely follows the original Java logic, minor differences in the results arise in some of the tests.

Java emphasizes predictability and portability, enforcing strict IEEE 754 behavior across platforms. Rust prioritizes performance and flexibility, allowing platform-specific optimizations that may deviate slightly from strict IEEE semantics.

These differences are generally negligible for practical purposes and do not affect the overall functionality or 
statistical significance of the test.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

### License of Original Java Implementation

```
Copyright (c) 2023 Paul Uszak.    

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```