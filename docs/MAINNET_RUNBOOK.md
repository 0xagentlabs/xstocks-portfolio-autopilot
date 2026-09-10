# Mainnet deployment runbook

This runbook is for the portfolio owner to execute personally. Repository automation and the delivery agent are restricted to devnet and must never handle a mainnet keypair or deploy this program to mainnet-beta.

## Frozen release inputs

- Contract source commit: `FROZEN_CONTRACT_COMMIT` (replaced by the documentation-only release commit)
- Expected SBF SHA-256: `FROZEN_SO_SHA256` (replaced by the documentation-only release commit)
- Program ID: `6Uzr4jz1SENxn3DdprQQ24zxaJXr6rThQNB48QxbuJed`
- Build toolchain: Rust 1.89.0 SBF platform tools, `cargo-build-sbf 4.1.0`, Solana CLI 4.2.1

Do not proceed unless an independent build of the frozen commit produces the exact expected hash. A Program ID is network-independent; deploying this ID for the first time on mainnet requires the corresponding program keypair. The devnet upgrade-authority key does not substitute for it.

## Offline preparation and build

On a trusted, clean machine:

```sh
git clone https://github.com/0xagentlabs/xstocks-portfolio-autopilot.git
cd xstocks-portfolio-autopilot
git checkout FROZEN_CONTRACT_COMMIT
cargo fmt --all -- --check
cargo build-sbf
cargo test --workspace
sha256sum target/deploy/xstocks_autopilot.so
solana-keygen pubkey /secure/offline/program-keypair.json
```

Confirm the last command returns the stated Program ID. Never copy, commit, paste into a ticket, or print the contents of either the program keypair or authority keypair.

## Authority and funding checklist

- Run `solana config get`; explicitly set `solana config set --url mainnet-beta` only in the owner's isolated deployment shell.
- Run `solana address` and record the expected fee payer/upgrade-authority public key through an approved offline process.
- Use `solana balance` to estimate deployment funding, then fund only that address through the owner's normal custody process.
- Keep fee payer, deploy authority, and upgrade authority on a hardware wallet or appropriately secured offline signer where supported.
- Decide before deployment whether upgrades remain enabled. Do not use `--final` until post-deployment verification is complete; finalization is irreversible.
- Verify there is no existing account at the Program ID: `solana program show 6Uzr...`. Stop if an unexpected program exists.

## Deploy and verify

Replace paths with the owner's secure paths, then execute interactively:

```sh
solana program deploy target/deploy/xstocks_autopilot.so \
  --program-id /secure/offline/program-keypair.json \
  --upgrade-authority /secure/offline/upgrade-authority.json \
  --fee-payer /secure/offline/fee-payer.json \
  --url mainnet-beta --output json
solana program show 6Uzr4jz1SENxn3DdprQQ24zxaJXr6rThQNB48QxbuJed --url mainnet-beta --output json
solana program dump 6Uzr4jz1SENxn3DdprQQ24zxaJXr6rThQNB48QxbuJed ./deployed.so --url mainnet-beta
sha256sum ./deployed.so
```

Confirm Program ID, loader owner, executable status, deployment slot, program-data address, and upgrade authority. Confirm the dumped hash equals the frozen SBF hash. Archive command output and signatures without private material.

## Post-deployment validation

Before allocating meaningful value, initialize the authority's PDA with a conservative configuration using the exact ABI in `docs/ABI.md`; immediately read it back with `solana account <PDA> --output json`. Simulate and then submit: UpdateStrategy, Draft → Running, one minimal RecordExecution, Running → Paused, and Paused → Stopped on a disposable test authority. Confirm malformed data, fake authority, excessive slippage, duplicate execution ID, and limit overflow are rejected and do not change account bytes. Because Stopped is terminal, never use the operational portfolio for this destructive rehearsal.

Only increase funds after reconciling the TypeScript decoder, the raw state bytes, and every transaction signature. The executor must independently fail closed on oracle freshness, quote source, decimal normalization, allowed DEX programs, price impact, realized loss, and custody balances.

## Emergency stop

1. Stop the off-chain scheduler and transaction submitter first.
2. From the authority wallet, send `SetStatus(2)` with value `5` to the portfolio PDA. This is valid from Draft, Running, Paused, RiskPaused, or Error.
3. Confirm the transaction at finalized commitment and read byte 41 of the 160-byte account; it must equal `5`.
4. Confirm subsequent RecordExecution and every SetStatus attempt are rejected.
5. Revoke executor credentials and investigate. `STOPPED` is deliberately irreversible; recovery requires a separately reviewed migration/new portfolio design, not a status reset.

Emergency stop only prevents this program from accepting new execution records. It does not unwind positions, revoke unrelated token delegates, stop an external bot by itself, or sell assets.
