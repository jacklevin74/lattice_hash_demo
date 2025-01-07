const crypto = require("crypto");
const blake3 = require("blake3");
const bs58 = require("bs58");

class LtHash {
  constructor() {
    this.hasher = blake3.createHash();
  }

  init() {
    this.hasher = blake3.createHash();
  }

  append(data) {
    this.hasher.update(data);
  }

  fini() {
    return this.hasher.digest({ length: 2048 }); // 2048-byte hash
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
    const finalHash = blake3.createHash();
    finalHash.update(hash);
    return finalHash.digest({ length: 32 }).toString("hex"); // 32-byte final hash
  }
}

function generateRandomAccount() {
  return {
    lamports: Math.floor(Math.random() * 1e9), // Random lamports value
    data: crypto.randomBytes(128), // 128 bytes of random data
    is_executable: Buffer.from([Math.random() < 0.5 ? 1 : 0]), // 1 byte boolean
    owner: crypto.randomBytes(32), // 32-byte random owner
    pubkey: bs58.encode(crypto.randomBytes(32)), // base58-encoded public key
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
