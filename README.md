# SP1 Rollup Simulator

🚀 A zero-knowledge rollup simulator built with [Succinct Labs' SP1 zkVM](https://github.com/succinctlabs/sp1). This project demonstrates a simple blockchain rollup that processes transactions and generates verifiable proofs using SP1's RISC-V zkVM.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.85.0-orange)](https://www.rust-lang.org/)
[![SP1](https://img.shields.io/badge/SP1-v4.2.0-blue)](https://github.com/succinctlabs/sp1)

## Features
- 💸 Processes transactions (e.g., transfers between accounts).
- 🔒 Generates zero-knowledge proofs for transaction validity.
- ✅ Verifies proofs to ensure correct state transitions.
- 🛠️ Modular CLI tools for proof generation and verification.

## Project Structure
- **`rollup-core`**: Shared logic for transactions and state.
- **`rollup-core-program`**: RISC-V program for SP1 zkVM to process transactions.
- **`rollup-cli`**: CLI to generate proofs from JSON inputs.
- **`verify-proof`**: CLI to verify generated proofs.

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (1.85.0-dev or later)
- [SP1 Toolchain](https://sp1.succinct.xyz/) (v4.2.0)
- Ubuntu 24.04.1 (or WSL2 on Windows)

## Installation
1. **Install Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env

2. **Install SP1 Toolchain**:
   ```bash
    curl -L https://sp1.succinct.xyz | bash
    source ~/.sp1/env
    sp1up

3. **Clone the Repository**:
    ```bash
    git clone https://github.com/Laolex/sp1-rollup-simulator.git
    cd sp1-rollup-simulator

### Usage
1. **Build the Project**:
    ```bash
    cd rollup-core-program
    cargo prove build
    cd ..

2. **Generate a Proof**:
    ```bash
    cargo run --release --bin rollup-cli -- --input input.json --output proof.json

## Example input.json:
    json

    {
    "transactions": [
        {"from": 1, "to": 2, "amount": 50},
        {"from": 2, "to": 3, "amount": 20}
    ],
    "state": {
        "balances": {
        "1": 100,
        "2": 30
        }
    }
    }

    Output: Final state {1: 50, 2: 10, 3: 20}.
3. **Verify the Proof**:
    ```bash
    cd verify-proof
    cargo run --release

#### Contributing
    Contributions are welcome! Please read CONTRIBUTING.md for guidelines on how to submit pull requests, report bugs, or suggest features.

##### Acknowledgments
    Built with  using Succinct Labs' SP1 zkVM.
    Inspired by the need for scalable, verifiable blockchain rollups.

##### Contact
    GitHub: Laolex
    Discord: !e_man#1444






