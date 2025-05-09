use clap::Parser;
use rollup_core::{Transaction, State};
use sp1_sdk::{ProverClient, SP1Stdin, SP1ProofWithPublicValues};
use std::{fs, io::Read, path::PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Input {
    transactions: Vec<Transaction>,
    state: State,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, default_value = "input.json")]
    input: PathBuf,
    #[arg(short, long, default_value = "proof.json")]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut file = fs::File::open(&args.input)
        .map_err(|e| format!("Failed to open input file {}: {}", args.input.display(), e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read input file {}: {}", args.input.display(), e))?;
    let input: Input = serde_json::from_str(&contents)
        .map_err(|e| format!("Invalid JSON format: {}", e))?;
    let mut stdin = SP1Stdin::new();
    stdin.write(&input.transactions);
    stdin.write(&input.state);
    let elf_path = "/home/laolex/projects/sp1-rollup-simulator/target/riscv32im-succinct-zkvm-elf/release/rollup_core_program";
    let elf = fs::read(elf_path)
        .map_err(|e| format!("Failed to read {}: {}", elf_path, e))?;
    let client = ProverClient::from_env();
    let (pk, vk) = client.setup(&elf);
    let proof: SP1ProofWithPublicValues = client
        .prove(&pk, &stdin)
        .run()
        .map_err(|e| format!("Proof generation failed: {}", e))?;
    client
        .verify(&proof, &vk)
        .map_err(|e| format!("Proof verification failed: {}", e))?;
    println!("✅ Proof verified successfully");
    let public_values: State = bincode::deserialize(proof.public_values.as_ref())?;
    println!("Final state: {:?}", public_values);
    fs::write(
        &args.output,
        serde_json::to_string_pretty(&proof).map_err(|e| format!("Failed to serialize proof: {}", e))?,
    )
    .map_err(|e| format!("Failed to write output file {}: {}", args.output.display(), e))?;
    println!("✅ Proof written to {}", args.output.display());
    Ok(())
}