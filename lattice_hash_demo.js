const { blake3 } = require('@noble/hashes/blake3');
const { bytesToHex } = require('@noble/hashes/utils');
const bs58 = require('bs58');

class LtHash {
  constructor() {
    this.hasher = blake3.create({});
  }

  init() {
    this.hasher = blake3.create({});
  }

  append(data) {
    this.hasher.update(data);
  }

  fini() {
    // Simulating 2048-byte XOF by creating multiple 32-byte chunks
    let fullOutput = [];
    let tempHash = this.hasher.digest(); // Get the initial digest (32 bytes)

    while (fullOutput.length < 2048) {
      // Rehash the digest to produce more chunks
      let chunk = blake3(tempHash);
      fullOutput.push(...chunk);
      tempHash = chunk; // Set the next input as the previous output
    }

    return Buffer.from(fullOutput.slice(0, 2048));
  }

  static add(a, b) {
    const result = Buffer.alloc(2048);
    for (let i = 0; i < 2048; i += 2) {
      const valA = a.readUInt16LE(i);
      const valB = b.readUInt16LE(i);
      result.writeUInt16LE((valA + valB) & 0xffff, i);
    }
    return result;
  }

  static sub(a, b) {
    const result = Buffer.alloc(2048);
    for (let i = 0; i < 2048; i += 2) {
      const valA = a.readUInt16LE(i);
      const valB = b.readUInt16LE(i);
      result.writeUInt16LE((valA - valB) & 0xffff, i);
    }
    return result;
  }

  static out(hash) {
    return bytesToHex(blake3(hash)).slice(0, 32); // Return 32-byte hex digest
  }
}

function generateRandomAccount() {
  return {
    lamports: Math.floor(Math.random() * 1e9), // Random lamports value
    data: Buffer.from(blake3(new Uint8Array(128))), // Randomized 128-byte data
    is_executable: Buffer.from([Math.random() < 0.5 ? 1 : 0]), // 1-byte boolean
    owner: Buffer.from(blake3(new Uint8Array(32))), // 32-byte owner
    pubkey: bs58.encode(Buffer.from(blake3(new Uint8Array(32)))), // base58 public key
  };
}

function computeLtHash(account) {
  if (account.lamports === 0) {
    return Buffer.alloc(2048, 0); // All zeros for zero lamports
  }

  const lthash = new LtHash();
  lthash.init();
  lthash.append(Buffer.from(account.lamports.toString()));
  lthash.append(account.data);
  lthash.append(account.is_executable);
  lthash.append(account.owner);
  lthash.append(Buffer.from(account.pubkey));
  return lthash.fini();
}

function main(numAccounts) {
  const accounts = Array.from({ length: numAccounts }, generateRandomAccount);

  console.log(`\n--- Lattice Hash Test for ${numAccounts} Accounts ---`);

  let combinedHash = Buffer.alloc(2048, 0); // Initial zeroed hash

  // Measure addition time
  console.time("Time for Adding All Accounts");
  accounts.forEach((account) => {
    const hash = computeLtHash(account);
    combinedHash = LtHash.add(combinedHash, hash);
  });
  console.timeEnd("Time for Adding All Accounts");

  const initialCombinedHashHex = LtHash.out(combinedHash);
  console.log(`Initial Combined Hash (first 16 chars): ${initialCombinedHashHex.slice(0, 16)}`);

  // Remove the last 50 accounts
  console.time("Time for Removing Last 50 Accounts");
  const last50Accounts = accounts.slice(-50);
  last50Accounts.forEach((account) => {
    const hash = computeLtHash(account);
    combinedHash = LtHash.sub(combinedHash, hash);
  });
  console.timeEnd("Time for Removing Last 50 Accounts");

  const afterRemovalHashHex = LtHash.out(combinedHash);
  console.log(`Hash after Removing Last 50 Accounts (first 16 chars): ${afterRemovalHashHex.slice(0, 16)}`);

  // Add the last 50 accounts back
  console.time("Time for Adding Last 50 Accounts Back");
  last50Accounts.forEach((account) => {
    const hash = computeLtHash(account);
    combinedHash = LtHash.add(combinedHash, hash);
  });
  console.timeEnd("Time for Adding Last 50 Accounts Back");

  const finalCombinedHashHex = LtHash.out(combinedHash);
  console.log(`Final Combined Hash (first 16 chars): ${finalCombinedHashHex.slice(0, 16)}`);

  console.log(`\nVerification: ${initialCombinedHashHex === finalCombinedHashHex ? "PASS ✅" : "FAIL ❌"}`);
}

const numAccounts = process.argv[2] ? parseInt(process.argv[2]) : 10000;
main(numAccounts);

