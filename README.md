# Escrow Q3 2026

An Anchor/Solana token escrow program for trustless swaps between two parties.
The maker deposits token A into a vault controlled by an escrow PDA and specifies
how much token B they want. A taker can complete the swap atomically; otherwise,
the maker can reclaim the deposit.

## Instructions

- `make(seed, deposit, receive, expiration)`: creates the escrow and deposits token A.
- `update(receive)`: lets the maker change the requested token-B amount.
- `take()`: transfers token B from the taker to the maker, releases token A to the taker, and closes the escrow.
- `refund()`: returns token A to the maker and closes the escrow and vault.

The escrow PDA is derived from `escrow`, the maker public key, and the little-endian
`u64` seed. Expiration must be in the future when the escrow is created, and a
swap cannot be completed after it expires. In other words, the escrow runs on a
timestamp-based condition: it is executable while the current timestamp is before
the expiration timestamp, and no longer executable once that deadline passes.

## Local testing

Prerequisites:

- Rust `1.89.0` (selected by `rust-toolchain.toml`)
- Cargo and the dependencies in `Cargo.lock`

Run the full test suite from the repository root:

```bash
anchor keys sync

anchor test
```

The tests use LiteSVM, so no local Solana validator is required. They cover:

- creating, updating, and taking an escrow;
- refunding an escrow; and
- rejecting a take after expiration.

`Anchor.toml` also exposes the same command through `anchor test` and skips
starting a local validator.
