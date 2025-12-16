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
