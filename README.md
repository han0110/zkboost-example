# zkboost Example

## Prerequisite

- Rust 1.88
- Docker

## Tutorial

In thie example, we use SP1 as the zkVM, and fibonacci as the guest.

### Step 1 - Create Guest

Create a new Rust project (use fibonacci as example):

```shell
cargo new ./fibonacci
```

Add guest dependency:

```
cd fibonacci
cargo add ere-platform-sp1 --git https://github.com/eth-act/ere --rev d15d36aa0a5d552a5159087884e7bea9ea74c745
```

In `./fibonacci/src/lib.rs`, write:

```rust
pub fn fibonacci(n: u32) -> u32 {
    let mut a = 0u32;
    let mut b = 1u32;
    for _ in 0..n {
        let c = a.wrapping_add(b);
        a = b;
        b = c;
    }
    a
}
```

In `./fibonacci/src/main.rs`, write:

```rust
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
```

### Step 2 - Compile Guest

Compile guest (in repo root):

```
docker run \
    -v ./fibonacci:/fibonacci \
    -e RUST_LOG=info \
    ghcr.io/eth-act/ere/ere-compiler-sp1:0.0.15-d15d36a \
    --compiler-kind \
    rust-customized \
    --guest-path \
    /fibonacci \
    --output-path \
    /fibonacci/fibonacci-sp1
```

### Step 3 - Create config and start `zkboost-server`

In `./config.toml`, write:

```toml
[[zkvm]]
kind = "sp1"
resource = "cpu"
program-id = "fibonacci-sp1"
program = "./fibonacci/fibonacci-sp1"
```

Install the `zkboost-server`:

```shell
cargo install --git https://github.com/eth-act/zkboost --rev 2ccd0d30babeab37b733d889e7ae4a6f4d0e4b6f zkboost-server
```

Start the server `zkboost-server`:

```shell
RUST_LOG=info \
  ERE_IMAGE_REGISTRY=ghcr.io/eth-act/ere \
  zkboost-server \
  --config ./config.toml \
  --port 3001
```

It will download the zkVM server image and then be ready to serve.

### Step 4 - Create client to ask `zkboost-server` for proof

In another terminal, create a new Rust project (in repo root):

```shell
cargo new ./fibonacci-client
```

Add `zkboost-client` dependency:

```
cd fibonacci-client
cargo add tokio -F full
cargo add fibonacci --path ../fibonacci
cargo add zkboost-client --git https://github.com/eth-act/zkboost --rev 2ccd0d30babeab37b733d889e7ae4a6f4d0e4b6f
cargo add ere-zkvm-interface --git https://github.com/eth-act/ere --rev d15d36aa0a5d552a5159087884e7bea9ea74c745
```

In `./fibonacci-client/src/main.rs`, write:

```rust
use ere_zkvm_interface::Input;
use fibonacci::fibonacci;
use std::time::Duration;
use zkboost_client::zkboostClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = zkboostClient::new("http://localhost:3001")?;
    let program_id = "fibonacci-sp1";

    // Prepare input and expected output.

    let n = 10u32;
    let fib_n = fibonacci(n);

    let input = Input::new()
        .with_prefixed_stdin(n.to_le_bytes().to_vec())
        .stdin;
    let output = [n.to_le_bytes(), fib_n.to_le_bytes()].concat();

    // Execute

    println!("Requesting execute...");
    let res = client.execute(program_id, input.clone()).await?;
    assert_eq!(res.public_values, output);
    println!(
        "Execution time: {:?}",
        Duration::from_millis(res.execution_time_ms as u64)
    );

    // Prove

    println!("Requesting prove...");
    let res = client.prove(program_id, input.clone()).await?;
    assert_eq!(res.public_values, output);
    println!(
        "Proving time: {:?}, proof size: {} KiB",
        Duration::from_millis(res.proving_time_ms as u64),
        res.proof.len() as f64 / 1024f64,
    );

    // Verify

    println!("Requesting verify...");
    let proof = res.proof;
    let res = client.verify(program_id, proof.clone()).await?;
    assert!(res.verified);
    assert_eq!(res.public_values, output);
    println!("Successfully verified");

    Ok(())
}
```

And finally run:

```shell
cargo run
```

And it should output something like:

```shell
Requesting execute...
Execution time: 3ms
Requesting prove...
Proving time: 12.547s, proof size: 1284.705078125 KiB
Requesting verify...
Successfully verified
```
