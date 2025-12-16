#![no_main]

use ere_platform_sp1::{Platform, SP1Platform, sp1_zkvm};
use fibonacci::fibonacci;

sp1_zkvm::entrypoint!(main);

fn main() {
    let input = SP1Platform::read_whole_input();

    // The private input is 4 bytes in little-endian
    let n = u32::from_le_bytes(input.as_ref().try_into().unwrap());

    // Compute the n-th Fibonacci number
    let fib_n = fibonacci(n);

    // The public output is the concatenate of input and n-th Fibonacci number
    let output = [n.to_le_bytes(), fib_n.to_le_bytes()].concat();

    SP1Platform::write_whole_output(&output);
}
