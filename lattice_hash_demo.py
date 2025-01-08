import sys
import time
import random
import base58
import numpy as np
import multiprocessing as mp
from blake3 import blake3
from numba import njit, prange

class LtHash:
    def __init__(self):
        """ Initializes an instance of LtHash. """
        self.hasher = None

    def init(self):
        """ Initializes the blake3 hasher. """
        self.hasher = blake3()

    def append(self, data: bytes):
        """ Appends data to the hash. """
        self.hasher.update(data)

    def fini(self):
        """ Finalizes the hash by extending the output to 2048 bytes. """
        temp_hash = self.hasher.digest()
        full_output = bytearray()

        while len(full_output) < 2048:
            temp_hash = blake3(temp_hash).digest()
            full_output.extend(temp_hash)

        return bytes(full_output[:2048])

    @staticmethod
    @njit(parallel=True, fastmath=True)
    def add(a: np.ndarray, b: np.ndarray) -> np.ndarray:
        """ Element-wise 16-bit addition with modulo 65536. """
        result = a.astype(np.uint32) + b.astype(np.uint32)
        return (result & np.uint16(0xFFFF)).astype(np.uint16)

    @staticmethod
    @njit(parallel=True, fastmath=True)
    def sub(a: np.ndarray, b: np.ndarray) -> np.ndarray:
        """ Element-wise 16-bit subtraction with modulo 65536. """
        result = a.astype(np.uint32) - b.astype(np.uint32)
        return (result & np.uint16(0xFFFF)).astype(np.uint16)

    @staticmethod
    def out(hash_bytes: bytes) -> str:
        """ Returns the first 32 characters of the hex-encoded blake3 digest. """
        return blake3(hash_bytes).digest().hex()[:32]


def generate_random_account():
    """ Generates a random account with blake3 hashed fields. """
    return {
        'lamports': random.randint(0, int(1e9)),
        'data': blake3(bytes(128)).digest(),
        'is_executable': bytes([random.randint(0, 1)]),
        'owner': blake3(bytes(32)).digest(),
        'pubkey': base58.b58encode(blake3(bytes(32)).digest()).decode('utf-8'),
    }


def compute_lt_hash(account):
    """ Computes the lattice hash for a given account. """
    if account['lamports'] == 0:
        return np.zeros(1024, dtype=np.uint16)

    lthash = LtHash()
    lthash.init()

    lthash.append(str(account['lamports']).encode('utf-8'))
    lthash.append(account['data'])
    lthash.append(account['is_executable'])
    lthash.append(account['owner'])
    lthash.append(account['pubkey'].encode('utf-8'))

    return np.frombuffer(lthash.fini(), dtype=np.uint16)


def process_accounts_parallel(accounts, combined_hash, add=True):
    """ Processes accounts in parallel, adding or subtracting their hashes. """
    num_workers = mp.cpu_count()
    pool = mp.Pool(num_workers)

    hashes = pool.map(compute_lt_hash, accounts)
    pool.close()
    pool.join()

    for h in hashes:
        if add:
            combined_hash[:] = LtHash.add(combined_hash, h)  # Modify in place
        else:
            combined_hash[:] = LtHash.sub(combined_hash, h)  # Modify in place

    return combined_hash


def main(num_accounts: int):
    """ Main function to generate accounts and compute the lattice hash. """
    accounts = [generate_random_account() for _ in range(num_accounts)]
    print(f"\n--- Lattice Hash Test for {num_accounts} Accounts ---")

    combined_hash = np.zeros(1024, dtype=np.uint16)

    # Step 1: Compute initial hash
    start = time.time()
    combined_hash = process_accounts_parallel(accounts, combined_hash, add=True)
    end = time.time()
    print(f"Time for Adding All Accounts: {end - start:.4f} seconds")

    initial_hash_hex = LtHash.out(combined_hash.tobytes())
    print(f"Initial Combined Hash (first 16 chars): {initial_hash_hex[:16]}")

    # Step 2: Remove the last 50 accounts
    last_50 = accounts[-50:] if len(accounts) >= 50 else accounts
    start = time.time()
    combined_hash = process_accounts_parallel(last_50, combined_hash, add=False)
    end = time.time()
    print(f"Time for Removing Last 50 Accounts: {end - start:.4f} seconds")

    after_removal_hash_hex = LtHash.out(combined_hash.tobytes())
    print(f"Hash after Removing Last 50 Accounts (first 16 chars): {after_removal_hash_hex[:16]}")

    # Step 3: Add back last 50 accounts
    start = time.time()
    combined_hash = process_accounts_parallel(last_50, combined_hash, add=True)
    end = time.time()
    print(f"Time for Adding Last 50 Accounts Back: {end - start:.4f} seconds")

    final_hash_hex = LtHash.out(combined_hash.tobytes())
    print(f"Final Combined Hash (first 16 chars): {final_hash_hex[:16]}")

    # Final verification
    print(f"\nVerification: {'PASS ✅' if (initial_hash_hex == final_hash_hex) else 'FAIL ❌'}")


if __name__ == "__main__":
    num_accounts = int(sys.argv[1]) if len(sys.argv) > 1 else 10000
    main(num_accounts)
