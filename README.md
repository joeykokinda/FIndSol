#FindSol

FindSol is a Rust-based Solana wallet key generator and balance checker. It continuously creates new random keypairs, queries the Solana mainnet for balances, and reports if any wallet contains SOL.

Generates fresh Solana Keypairs in a loop

Uses the Solana RpcClient to query balances on mainnet

Tracks total attempts during the search

Displays wallet public and private keys when a nonzero balance is detected

Tech Stack

Rust (edition 2024)

solana-sdk

solana-client
