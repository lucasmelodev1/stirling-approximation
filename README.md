# Stirling's Approximation

Provide 2 functions (to_f64 and to_bigdecimal) that calculates the
factorial of a number using the Stirling's Approximation formula.

This formula let you calculate the factorial with a very good precision
for big numbers, and is not recursive, which saves a lot of computation
time.

The formula is the following:
n! = sqrt(2 * PI * n) * (n / E) ^ n

## Example

```rust
use stirling_approximation;

let factorial = stirling_approximation::to_f64(10);
let high_precision_factorial = stirling_approximation::to_bigdecimal(10).unwrap();

println!("The factorial of 10 is: {}", factorial);
println!("The high precision factorial of 10 is: {}", high_precision_factorial);
```
