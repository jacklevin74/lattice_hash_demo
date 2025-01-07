# Little Lattice Hash Demo

Simple demo benchmarking of a 2048-byte lattice hash using the `@noble/hashes` BLAKE3 implementation.
Reference SIMD 0215: https://github.com/solana-foundation/solana-improvement-documents/blob/main/proposals/0215-accounts-lattice-hash.md

## Installation and Running

```bash
# Clone the repository
git clone https://github.com/jacklevin74/lattice_hash_demo
cd lattice-hash-demo

# Install dependencies
npm install

# Run the script with the default number of accounts (10,000)
node lattice_hash_demo.js

# Or specify a custom number of accounts
node lattice_hash_demo.js 50000

--- Lattice Hash Test for 10000 Accounts ---
Time for Adding All Accounts: 55ms
Initial Combined Hash (first 16 chars): e0a5f6b3e1fa4b1a
Time for Removing Last 50 Accounts: 3ms
Hash after Removing Last 50 Accounts (first 16 chars): d3e9f4b7c8e4a1b2
Time for Adding Last 50 Accounts Back: 2ms
Final Combined Hash (first 16 chars): e0a5f6b3e1fa4b1a

Verification: PASS ✅

# If you want to run demo in RUST
# Note Rust program will also test shuffling of accounts before add/remove test
# verifying final hashes

0. cd rust/lattice_hash_demo

1. Build the project:
   cargo build --release

2. Run the program:
   cargo run --release [num_accounts]
   - Replace [num_accounts] with the number of accounts

Example:
   cargo run --release 5000
