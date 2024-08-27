# Solana Deposit and Withdraw Program

## Overview

This project implements a native Solana program that allows users to initialize an account, deposit SOL into it, and withdraw 10% of the deposited SOL at a given time. It serves as a basic example of how to create and interact with Solana programs.

## Table of Contents

1. [Features](#features)
2. [Prerequisites](#prerequisites)
3. [Installation](#installation)
4. [Usage](#usage)
   - [Deploying the Program](#deploying-the-program)
   - [Interacting with the Program](#interacting-with-the-program)
5. [Program Structure](#program-structure)
6. [Client Application](#client-application)
7. [Testing](#testing)
8. [Security Considerations](#security-considerations)
9. [Contributing](#contributing)
10. [License](#license)

## Features

- Initialize a new account
- Deposit SOL into the account
- Withdraw 10% of the deposited SOL
- Rust-based Solana program
- Client application for easy interaction

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Solana CLI Tools](https://docs.solana.com/cli/install-solana-cli-tools) (version 1.18.4 or compatible)
- [Git](https://git-scm.com/downloads)

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/codewithmide/solana_deposit_withdraw.git
   cd solana_deposit_withdraw
   ```

2. Build the program:

   ```bash
   cargo build-bpf
   ```

## Usage

### Deploying the Program

1. Ensure you have a Solana keypair set up:

   ```bash
   solana-keygen new
   ```

2. Set your Solana configuration to a test network (e.g., devnet):

   ```bash
   solana config set --url https://api.devnet.solana.com
   ```

3. Airdrop some SOL to your account (if on devnet):

   ```bash
   solana airdrop 2
   ```

4. Deploy the program:

   ```bash
   solana program deploy target/deploy/solana_deposit_withdraw.so
   ```

5. Save the program ID output after deployment.

### Interacting with the Program

Use the provided client application to interact with the deployed program. See the [Client Application](#client-application) section for details.

## Program Structure

The Solana program consists of three main functions:

1. `initialize_account`: Sets up a new account with zero balance.
2. `deposit`: Allows depositing SOL into the account.
3. `withdraw`: Enables withdrawing 10% of the account's balance.

The program uses the Borsh serialization format for account data.

## Client Application

A Rust-based client application is provided to interact with the deployed program. To use it:

1. Navigate to the client directory:

   ```bash
   cd client
   ```

2. Update the program ID in `src/main.rs` with your deployed program ID.

3. Run the client:

   ```bash
   cargo run
   ```

The client demonstrates account initialization, SOL deposit, and withdrawal.

## Testing

To run the program tests:

```bash
cargo test-bpf
```

This will execute the test suite and verify the program's functionality.

## Security Considerations

- This program is for educational purposes and not audited for production use.
- Ensure proper access controls and input validation in a real-world scenario.
- Be cautious with handling user funds and implement necessary safety checks.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
