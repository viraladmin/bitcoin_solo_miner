# Bitcoin CPU Solo Miner

This is a CPU-based Bitcoin solo miner designed primarily for educational and experimental purposes.  
It demonstrates how to build and submit valid Bitcoin blocks without joining a mining pool or using specialized ASIC hardware.

The miner takes a novel approach compared to traditional mining loops:

- Instead of only iterating over the block header’s `nonce` field, it inserts a random sentence from a user-provided file directly into the coinbase transaction's scriptSig as extra nonce data.
- This changes the Merkle root every attempt, producing a unique block header for each hash.
- The inserted text is not placed in an OP_RETURN output — it exists inside the coinbase input script, just like traditional extra nonce data.

While the odds of finding a block on a CPU are still astronomically low, they are statistically far better than trying to find a random private key with spendable Bitcoin (see why I built it). Also the fact remains, every single attempt to find a valid block, has the __exact same odds__ as every other attempt on the network. The reason high powered ASIC miners find the blocks is not because their odds are different, but rather because they can make more attempts per second by magnitudes. Still __every attempt has equal odds__.

If you’re going to burn CPU cycles, you might as well be trying to earn some BTC instead of hunting for keys that almost certainly do not exist. Also why not put old computers / CPUs to use, you never know - you could get lucky? :) 

---

## Show Support

I make nothing creating and sharing the tools I create. I do it for my love of the space and my love of the people in the space.

Help a fellow dev out, I aint vibe codinghere. Whats a sat or two between friends. :)

Bitcoin: bc1qls06cusnr0w7f3q5xtf6d8lfx4gf649375tp87

---

## Why I Built It

Many people waste huge amounts of computing power on “Satoshi key hunting” — scanning random private keys hoping to find BTC. The odds of success are so low they are effectively zero.

By contrast, Bitcoin mining, while still highly competitive and dominated by ASIC hardware, has a non-zero probability for any valid hash attempt to succeed.

Every hash you produce has exactly the same odds of being under the current target as one from any other machine, whether it’s a $20 CPU or a $20,000 ASIC.

This miner exists to:
- Show exactly how a valid block is built from a getblocktemplate response.
- Teach how extra nonce data inside the coinbase transaction affects the Merkle root.
- Provide a working example of block header hashing and submission via submitblock.

---

## Features

- SegWit-native mining — requires a bc1q native SegWit address as the payout address.
- Randomized extra nonce insertion using sentences from a file.
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
- A bc1q native SegWit payout address.
- RPC credentials for your node (set in `.env`).

---

## Configuration

Create a `.env` file in the miner’s directory:

RPC_URL="http://127.0.0.1:8332"
RPC_USER="admin"
RPC_PASS="pass"
PAYOUT_ADDRESS="bc1q...."
STRINGS_FILE="strings.txt"
NUM_TASKS=20

**Explanation of fields:**
- `RPC_URL` — URL to your Bitcoin node’s RPC endpoint.
- `RPC_USER` / `RPC_PASS` — RPC username and password (set in your `bitcoin.conf`).
- `PAYOUT_ADDRESS` — Your bc1q native SegWit address where mining rewards will be sent if you find a block.
- `STRINGS_FILE` — Path to a UTF-8 text file containing one sentence per line.
- `NUM_TASKS` — Number of concurrent mining tasks to run.

---

## Running the Miner

./bitcoin_solo_miner

There are no command-line flags. The miner will:
- Load configuration from `.env`.
- Connect to your Bitcoin node.
- Begin mining immediately using the provided sentence list.

If you want it to run in the background, you must configure that yourself, e.g.:

nohup ./bitcoin_solo_miner > miner.log 2>&1 &

---

## Notes & Limitations

- This miner is for demonstration and educational purposes only — it is not competitive with ASIC hardware.
- CPU hash rate will be extremely low, so finding a block is extremely unlikely, but possible.
- Mining requires a fully synced node — the miner cannot operate without up-to-date block templates.
- The payout address must be bc1q (native SegWit) or blocks will be invalid.
- The miner uses `fastrand::u32(..)` to vary nonce values along with the extra nonce sentences.

---

## License

MIT License — free to use, modify, and share.

---

## Disclaimer

These tools are provided as is for educational and research purposes only. No warranty is provided for any damages incured by using these tools.
