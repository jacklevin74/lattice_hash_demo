use blake3::{Hasher};
use bs58;
use rand::{seq::SliceRandom, Rng};
use std::time::Instant;

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
        a.iter()
            .zip(b)
            .map(|(x, y)| ((x.wrapping_add(*y)) & 0xFF) as u8)
            .collect()
    }

    fn sub(a: &[u8], b: &[u8]) -> Vec<u8> {
        a.iter()
            .zip(b)
            .map(|(x, y)| ((x.wrapping_sub(*y)) & 0xFF) as u8)
            .collect()
    }

    fn out(hash: &[u8]) -> String {
        let truncated = blake3::hash(hash); // Save hash to extend lifetime
        let truncated_hash = truncated.as_bytes(); // Borrow it safely
        hex::encode(&truncated_hash[0..16]) // Return the first 16 bytes as a hex string
    }
}

#[derive(Clone)] // Deriving `Clone` to enable cloning
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
    let num_accounts = 10_000;
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

    // Take the last 50 accounts to test
    let last_50_accounts: Vec<_> = accounts[(num_accounts - 50)..].to_vec();

    for i in 1..=5 {
        println!("\n--- Iteration {i}: Adding and Removing the Same 50 Accounts ---");

        // Shuffle accounts in a random order
        let mut shuffled_accounts = last_50_accounts.clone();
        shuffled_accounts.shuffle(&mut rand::thread_rng());

        println!("Order of accounts for iteration {i}:");
        for (index, account) in shuffled_accounts.iter().enumerate().take(3) {
            println!("Account {}: pubkey prefix: {}", index + 1, &account.pubkey[..6]);
        }

        // Remove accounts
        let start_remove = Instant::now();
        for account in &shuffled_accounts {
            let hash = compute_lt_hash(account);
            combined_hash = LtHash::sub(&combined_hash, &hash);
        }
        let elapsed_remove = start_remove.elapsed();
        println!("Time for Removing Accounts in Iteration {i}: {:?}", elapsed_remove);

        let after_removal_hash_hex = LtHash::out(&combined_hash);
        println!("Hash after Removing Accounts (first 16 chars): {}", after_removal_hash_hex);

        // Add accounts back
        let start_add_back = Instant::now();
        for account in &shuffled_accounts {
            let hash = compute_lt_hash(account);
            combined_hash = LtHash::add(&combined_hash, &hash);
        }
        let elapsed_add_back = start_add_back.elapsed();
        println!("Time for Adding Accounts Back in Iteration {i}: {:?}", elapsed_add_back);

        let final_combined_hash_hex = LtHash::out(&combined_hash);
        println!("Final Combined Hash after Iteration {i} (first 16 chars): {}", final_combined_hash_hex);

        if initial_combined_hash_hex == final_combined_hash_hex {
            println!("Iteration {i} Verification: PASS ✅");
        } else {
            println!("Iteration {i} Verification: FAIL ❌");
        }
    }
}

