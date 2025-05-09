use sp1_sdk::{ProverClient, SP1ProofWithPublicValues};
use rollup_core::State;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proof_path = "/home/laolex/projects/sp1-rollup-simulator/proof.json";
    let elf_path = "/home/laolex/projects/sp1-rollup-simulator/target/riscv32im-succinct-zkvm-elf/release/rollup_core_program";

    // Check file existence
    if !std::path::Path::new(proof_path).exists() {
        return Err(format!("Proof file not found: {}", proof_path).into());
    }
    if !std::path::Path::new(elf_path).exists() {
        return Err(format!("ELF binary not found: {}", elf_path).into());
    }

    // Read proof.json
    let proof_json = fs::read_to_string(proof_path)?;
    let proof: SP1ProofWithPublicValues = serde_json::from_str(&proof_json)?;

    // Read ELF binary
    let elf = fs::read(elf_path)?;

    // Set up ProverClient
    let client = ProverClient::from_env();
    let (_, vk) = client.setup(&elf);

    // Verify proof
    client.verify(&proof, &vk)?;
    println!("✅ Proof verified successfully");

    // Deserialize public values as State
    let public_values: State = bincode::deserialize(proof.public_values.as_ref())?;
    println!("Final state: {:?}", public_values);

    Ok(())
}