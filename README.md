# Lattice Hash Demo

Simple demo benchmarking of a 2048-byte lattice hash using the `@noble/hashes` BLAKE3 implementation.

## Installation

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

