# Changelog

## [0.1.2] - 2025-08-10
- __Critical bug fix__ in mining algorithm

### Added
- Added .env tempate file to release

### Changed
- Fixed `hash_meets_target` function to correctly compare the block hash and the target in __big endian__ format matching Bitcoin Core’s proof-of-work check..
- Updated frequency of changing addresses
- Update RPC connection variable in linux

### Notes 
- __All previous releases are broken and obsolete__ and should not be used due to critical bug in mining that prevented ever finding valid blocks. Upgrade Immdiately.


## [0.1.1] - 2025-08-07

### Added
- **Payout address rotation support** via `payout_addresses.txt`
  - New `.env` field: `PAYOUT_ADDRESSES`
  - Automatically selects a new payout address every 30 seconds
  - Winning address is removed from the list after a block is found
  - Prevents address reuse and mimics Satoshi's block-by-block behavior
- **Compatibility with [bitcoin_mass_address_generator](https://crates.io/crates/bitcoin_mass_address_generator)**
  - Use generated HD wallets to populate your payout address file
- Support for multi-threaded mining with `NUM_TASKS` config
- Enhanced `.env` configurability and miner runtime diagnostics

### Changed
- Updated `README.md` with new configuration, examples, and address file format
- Improved internal logging and error handling during mining and block submission
- Minor CLI argument parsing improvements

### Notes
- This release focuses on privacy, automation, and education — helping solo miners experiment with rotating address strategies in line with Bitcoin best practices.
- While statistically unlikely to find a block on CPU, this version demonstrates fully valid block construction, hashing, and submission.

### Security
- Ensure that all payout addresses in `payout_addresses.txt` are valid `bc1q` SegWit addresses to avoid invalid block submissions.
