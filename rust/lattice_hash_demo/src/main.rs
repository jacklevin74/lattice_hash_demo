use blake3::Hasher;
use bs58;
use rand::Rng;
use std::env;
use std::time::Instant;
use hex;

const LTHASH_LEN: usize = 2048; // 2048 bytes for the Lattice Hash

struct LtHash {
    hasher: Hasher,
}

impl LtHash {
    fn new() -> Self {
        Self {
            hasher: Hasher::new(),
        }
    }

    fn init(&mut self) {
        self.hasher = Hasher::new();
    }

    fn append(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }

    fn fini(&mut self) -> Vec<u8> {
        let mut full_output = vec![];
        let mut digest = self.hasher.finalize().as_bytes().to_vec(); // Initial 32-byte digest

        while full_output.len() < LTHASH_LEN {
            full_output.extend(&digest);
            digest = blake3::hash(&digest).as_bytes().to_vec(); // Generate another 32 bytes
        }

        full_output.truncate(LTHASH_LEN);
        full_output
    }

    fn add(a: &[u8], b: &[u8]) -> Vec<u8> {
        a.chunks_exact(2)
            .zip(b.chunks_exact(2))
            .map(|(x, y)| {
                let val_a = u16::from_le_bytes([x[0], x[1]]);
                let val_b = u16::from_le_bytes([y[0], y[1]]);
                (val_a.wrapping_add(val_b)).to_le_bytes()
            })
            .flatten()
            .collect()
    }

    fn sub(a: &[u8], b: &[u8]) -> Vec<u8> {
        a.chunks_exact(2)
            .zip(b.chunks_exact(2))
            .map(|(x, y)| {
                let val_a = u16::from_le_bytes([x[0], x[1]]);
                let val_b = u16::from_le_bytes([y[0], y[1]]);
                (val_a.wrapping_sub(val_b)).to_le_bytes()
            })
            .flatten()
            .collect()
    }

    fn out(hash: &[u8]) -> String {
        let hash_result = blake3::hash(hash);
        hex::encode(&hash_result.as_bytes()[0..16]) // Return the first 16 bytes as a hex string
    }
}

struct Account {
    lamports: u64,
    data: Vec<u8>,
    is_executable: u8,
    owner: Vec<u8>,
    pubkey: String,
}

impl Account {
    fn generate_random() -> Self {
        let mut rng = rand::thread_rng();
        Account {
            lamports: rng.gen_range(1..1_000_000_000),
            data: (0..128).map(|_| rng.gen()).collect(), // Random 128 bytes
            is_executable: if rng.gen_bool(0.5) { 1 } else { 0 },
            owner: (0..32).map(|_| rng.gen()).collect(), // Random 32-byte owner
            pubkey: bs58::encode((0..32).map(|_| rng.gen::<u8>()).collect::<Vec<u8>>()).into_string(),
        }
    }
}

fn compute_lt_hash(account: &Account) -> Vec<u8> {
    if account.lamports == 0 {
        return vec![0; LTHASH_LEN];
    }

    let mut lthash = LtHash::new();
    lthash.init();
    lthash.append(&account.lamports.to_le_bytes());
    lthash.append(&account.data);
    lthash.append(&[account.is_executable]);
    lthash.append(&account.owner);
    lthash.append(account.pubkey.as_bytes());
    lthash.fini()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_accounts: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(10_000) // Default to 10,000 if parsing fails
    } else {
        10_000 // Default number of accounts
    };

    let accounts: Vec<Account> = (0..num_accounts).map(|_| Account::generate_random()).collect();

    println!("\n--- Lattice Hash Test for {num_accounts} Accounts ---");

    let mut combined_hash = vec![0; LTHASH_LEN]; // Initial zeroed hash

    // Measure addition time
    let start_add = Instant::now();
    for account in &accounts {
        let hash = compute_lt_hash(account);
        combined_hash = LtHash::add(&combined_hash, &hash);
    }
    let elapsed_add = start_add.elapsed();
    println!("Time for Adding All Accounts: {:?}", elapsed_add);

    let initial_combined_hash_hex = LtHash::out(&combined_hash);
    println!("Initial Combined Hash (first 16 chars): {}", initial_combined_hash_hex);

    // Remove the last 50 accounts
    let start_remove = Instant::now();
    let last_50_accounts = &accounts[(num_accounts - 50)..];
    for account in last_50_accounts {
        let hash = compute_lt_hash(account);
        combined_hash = LtHash::sub(&combined_hash, &hash);
    }
    let elapsed_remove = start_remove.elapsed();
    println!("Time for Removing Last 50 Accounts: {:?}", elapsed_remove);

    let after_removal_hash_hex = LtHash::out(&combined_hash);
    println!("Hash after Removing Last 50 Accounts (first 16 chars): {}", after_removal_hash_hex);

    // Add the last 50 accounts back
    let start_add_back = Instant::now();
    for account in last_50_accounts {
        let hash = compute_lt_hash(account);
        combined_hash = LtHash::add(&combined_hash, &hash);
    }
    let elapsed_add_back = start_add_back.elapsed();
    println!("Time for Adding Last 50 Accounts Back: {:?}", elapsed_add_back);

    let final_combined_hash_hex = LtHash::out(&combined_hash);
    println!("Final Combined Hash (first 16 chars): {}", final_combined_hash_hex);

    if initial_combined_hash_hex == final_combined_hash_hex {
        println!("\nVerification: PASS ✅");
    } else {
        println!("\nVerification: FAIL ❌");
    }
}

