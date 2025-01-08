
# Little Lattice Hash Demo

Simple demo benchmarking a 2048-byte lattice hash using the `@noble/hashes` BLAKE3 implementation.  
Reference SIMD 0215: [Solana Accounts Lattice Hash Proposal](https://github.com/solana-foundation/solana-improvement-documents/blob/main/proposals/0215-accounts-lattice-hash.md)

---

## Installation and Running (JavaScript)

### 1. Clone the Repository:
```bash
git clone https://github.com/jacklevin74/lattice_hash_demo
cd lattice-hash-demo
```

### 2. Install Dependencies:
```bash
npm install
```

### 3. Run the Script:
To run with the default number of accounts (10,000):
```bash
node lattice_hash_demo.js
```

To specify a custom number of accounts:
```bash
node lattice_hash_demo.js 50000
```

### Example Output:
```plaintext
--- Lattice Hash Test for 10000 Accounts ---
Time for Adding All Accounts: 55ms
Initial Combined Hash (first 16 chars): e0a5f6b3e1fa4b1a
Time for Removing Last 50 Accounts: 3ms
Hash after Removing Last 50 Accounts (first 16 chars): d3e9f4b7c8e4a1b2
Time for Adding Last 50 Accounts Back: 2ms
Final Combined Hash (first 16 chars): e0a5f6b3e1fa4b1a

Verification: PASS ✅
```

---

## Running the Demo in Rust

This Rust version also tests adding and removing the same 50 accounts multiple times in random order.

### 1. Navigate to the Rust Project:
```bash
cd rust/lattice_hash_demo
```

### 2. Build the Project:
```bash
cargo build --release
```

### 3. Run the Program:
```bash
cargo run --release [num_accounts]
```
- Replace `[num_accounts]` with the number of accounts you want to test.

### Example:
```bash
cargo run --release 5000
```

## Running the Demo in Python:

### 1. In project directory install needed modules:
```bash
pip3 install base58 blake3 numba
```

### 2. Run the program
```bash
python3 lattice_hash_demo.py
```

---
