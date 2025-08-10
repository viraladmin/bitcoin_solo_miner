# Bitcoin CPU Solo Miner

This is a CPU-based Bitcoin solo miner designed primarily for educational and experimental purposes.

It demonstrates how to build and submit valid Bitcoin blocks without joining a mining pool or using specialized ASIC hardware.

The miner takes a novel approach compared to traditional mining loops:

- Instead of only iterating over the block header’s `nonce` field, it inserts a random sentence from a user-provided file directly into the coinbase transaction's scriptSig as extra nonce data.
- It also switches the address every 30 seconds for maximum entropyi of data.
- This changes the Merkle root every attempt, producing a unique block header for each hash.
- The inserted text is not placed in an OP_RETURN output — it exists inside the coinbase input script, just like traditional extra nonce data.

While the odds of finding a block on a CPU are still astronomically low, they are statistically far better than trying to find a random private key with spendable Bitcoin (see why I built it). Also the fact remains, every single attempt to find a valid block has the __exact same odds__ as every other attempt on the network. The reason high powered ASIC miners find the blocks is not because their odds are different, but rather because they can make more attempts per second by magnitudes. Still, __every attempt has equal odds__.

If you’re going to burn CPU cycles, you might as well be trying to earn some BTC instead of hunting for keys that almost certainly do not exist. Also why not put old computers / CPUs to use — you never know, you could get lucky. :)

---

## Critical Notice
All versions prior to **v0.1.2** contain a bug in the mining process that prevents finding valid blocks.
Upgrade immediately to v0.1.2 or later.  

---

## Show Support

I make nothing creating and sharing the tools I create. I do it for my love of the space and my love of the people in the space.

Help a fellow dev out. I ain’t vibe coding here. What’s a sat or two between friends? :)

**Bitcoin:** `bc1qls06cusnr0w7f3q5xtf6d8lfx4gf649375tp87`

---

## Why I Built It

Many people waste huge amounts of computing power on “Satoshi key hunting” — scanning random private keys hoping to find BTC. The odds of success are so low they are effectively zero.

By contrast, Bitcoin mining, while still highly competitive and dominated by ASIC hardware, has a non-zero probability for any valid hash attempt to succeed.

Every hash you produce has exactly the same odds of being under the current target as one from any other machine, whether it’s a $20 CPU or a $20,000 ASIC.

This miner exists to:

- Show exactly how a valid block is built from a getblocktemplate response.
- Teach how extra nonce data inside the coinbase transaction affects the Merkle root.
- Provide a working example of block header hashing and submission via submitblock.
- Introduce address rotation per block to prevent address reuse (just like Satoshi did).

---

## Features

- SegWit-native mining — uses bc1q native SegWit addresses pulled from a rotating address list.
- Randomized extra nonce insertion using sentences from a file.
- Automatic payout address rotation every 30 seconds.
- Automatically removes the winning address from the list to prevent reuse.
- Continuous mining loop that:
  1. Fetches a fresh block template periodically.
  2. Inserts a random sentence into the coinbase.
  3. Rebuilds the Merkle root.
  4. Hashes the block header.
  5. Submits the block if the hash meets the target.
- Works with a fully synced Bitcoin Core node (full or pruned).

---

## Requirements

- Bitcoin Core node (v22+ recommended) running with RPC enabled.
- The node must be fully synced to the network tip.
- A bc1q native SegWit payout address (or list of addresses).
- RPC credentials for your node (set in `.env`).

---

## Configuration

Create a `.env` file in the miner’s directory:

```env
RPC_URL="http://127.0.0.1:8332"
RPC_USER="admin"
RPC_PASS="pass"
PAYOUT_ADDRESSES="payout_addresses.txt"
STRINGS_FILE="strings.txt"
NUM_TASKS=20
```

**Explanation of fields:**

- `RPC_URL` — URL to your Bitcoin node’s RPC endpoint.
- `RPC_USER` / `RPC_PASS` — RPC username and password (set in your `bitcoin.conf`).
- `PAYOUT_ADDRESSES` — Path to a file containing one payout address per line. The miner will auto-rotate between them every 30 seconds.
- `STRINGS_FILE` — Path to a UTF-8 text file containing one sentence per line.
- `NUM_TASKS` — Number of concurrent mining tasks to run.

---

## Address Rotation and Management

The miner now supports dynamic payout address rotation via the `payout_addresses.txt` file.

- Every 30 seconds, the miner selects a **new random address** from the file to use as the current mining payout target.
- If a block is found, the winning address is **automatically removed** from the file to ensure it is never reused again.
- This mimics Satoshi's original behavior of changing addresses per mined block, and follows best practices of never reusing a Bitcoin address.

This system is compatible with address files generated by [`bitcoin_mass_address_generator`](https://crates.io/crates/bitcoin_mass_address_generator). You can generate thousands of wallets and feed them directly into the miner.

**Example `payout_addresses.txt`:**
```
bc1qxyz...
bc1qabc...
bc1qlmn...
```

Each address must be a valid `bc1q` SegWit address. Invalid or duplicate entries will be ignored.

---

## Running the Miner

```
./bitcoin_solo_miner
```

There are no command-line flags. The miner will:

- Load configuration from `.env`
- Connect to your Bitcoin node
- Begin mining immediately using the sentence list and rotating address list

To run in the background:

```
nohup ./bitcoin_solo_miner > miner.log 2>&1 &
```

---

## Notes & Limitations

- This miner is for demonstration and educational purposes only — it is not competitive with ASIC hardware.
- CPU hash rate will be extremely low, so finding a block is extremely unlikely, but possible.
- Mining requires a fully synced node — the miner cannot operate without up-to-date block templates.
- The payout addresses must be valid bc1q (native SegWit) or blocks will be rejected.
- The miner uses `fastrand::u32(..)` to vary nonce values along with the extra nonce sentences.

---

## License

MIT License — free to use, modify, and share.

---

## Disclaimer

These tools are provided as is for educational and research purposes only. No warranty is provided for any damages incurred by using these tools.
