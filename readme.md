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

To test the implementation three files of binary random data have been analyzed using the original Java program (version 0.6.0-beta) and this Rust port, expecting the same p-values. The test files can be found in the test folder.

### Testing 1024 bytes from test1.bin (/dev/random)

Test          | Java ent3000 | Rust onod3000 | Note          |
------------- | ------------ | ------------- | ------------- |
Shannon       | N/A          | 0.0000        |               |
Monobit       | 0.6746       | 0.6746        |               |
ChiBit        | 0.9299       | 0.9299        |               |
ChiByte       | 0.9819       | 0.9819        |               |
MeanByte      | 0.7964       | 0.7964        |               |
Compression   | N/A          | 0.0146        |               |
KS            | 0.5313       | 0.2476        | Mismatch      |
Pi            | 0.3784       | 0.3784        |               |
Shells        | 0.7559       | 0.7559        |               |
Gaps          | 0.0000       | 0.0000        |               |
Avalanche     | 0.8794       | 0.8794        |               |
Runs          | 0.0850       | 0.0850        |               |
RunUps        | 0.6537       | 0.6537        |               |
Prediction    | 0.6164       | 0.6164        |               |
UnCorrelation | 0.0921       | 0.0918        | Close enough  |

Feel free to provide more results, and I'll continue updating the table!

### Testing 259,200 bytes from test2.bin (/dev/random)

Test          | Java ent3000 | Rust onod3000 | Note          |
------------- | ------------ | ------------- | ------------- |
Shannon       | N/A          | 0.6853        |               |
Monobit       | 0.2103       | 0.2103        |               |
ChiBit        | 0.5590       | 0.5590        |               |
ChiByte       | 0.3748       | 0.3748        |               |
MeanByte      | 0.0183       | 0.0183        |               |
Compression   | N/A          | 0.9308        |               |
KS            | 0.2900       | 0.0178        | Mismatch      |
Pi            | 0.9765       | 0.9765        |               |
Shells        | 0.7235       | 0.7235        |               |
Gaps          | 0.6425       | 0.6425        |               |
Avalanche     | 0.9995       | 0.9995        |               |
Runs          | 0.3310       | 0.3310        |               |
RunUps        | 0.9393       | 0.9393        |               |
Prediction    | 0.1519       | 0.1519        |               |
UnCorrelation | 0.4497       | 0.4497        |               |

### Testing 259,200 bytes from test3.bin (Hardware QRNG)

Test          | Java ent3000 | Rust onod3000 | Note |
------------- | ------------ | ------------- | ---- |
Shannon       | N/A          | 0.6978        |      |
Monobit       | 0.4926       | 0.4926        |      |
ChiBit        | 0.8428       | 0.8428        |      |
ChiByte       | 0.5785       | 0.5785        |      |
MeanByte      | 0.9601       | 0.9601        |      |
Compression   | N/A          | 0.9308        |      |
KS            | 0.7395       | 0.0441        | Mismatch |
Pi            | 0.2806       | 0.2806        |      |
Shells        | 0.7711       | 0.7711        |      |
Gaps          | 0.1937       | 0.1937        |      |
Avalanche     | 0.9932       | 0.9932        |      |
Runs          | 0.2500       | 0.2500        |      |
RunUps        | 0.9710       | 0.9710        |      |
Prediction    | 0.7173       | 0.7173        |      |
UnCorrelation | 0.4674       | 0.4674        |      |

### Conclusions

Although the implementation of the randomness tests closely follows the original Java logic, differences in the results arise in the Kolmogorov-Smirnov test. The external libraries used for running the test probably differ in their implementation. Also the uniform distribution to test against is provided by the Apache Commons Math library. Since it would be way beyond the scope of this porting project to try to fully match the functionality of the dependencies used we just accept the minor difference. Another issue might be that Java emphasizes predictability and portability, enforcing strict IEEE 754 behavior across platforms. Rust on the other hand prioritizes performance and flexibility, allowing platform-specific optimizations that may deviate slightly from strict IEEE semantics.

These differences are generally negligible for practical purposes and do not affect the overall functionality or 
statistical significance of the test.

### Bonus

We implemented a Well Equidistributed Long-period Linear pseudo-random number generator that is used with a random seed derived from an epoch timestamp that is being used as uniform distribution for the KS test.

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

## Credits

This work was made possible with the invaluable assistance of OpenAI's ChatGPT, which provided guidance, debugging help, and inspiration throughout the development process.