# Program ABI v2

Program ID: `6Uzr4jz1SENxn3DdprQQ24zxaJXr6rThQNB48QxbuJed`. All integers are little-endian. Portfolio PDA seeds are UTF-8 `portfolio` and the 32-byte authority public key.

## Instructions

| Tag | Instruction | Data after tag | Accounts in order |
|---:|---|---|---|
| 0 | Initialize | bump u8; asset_count u8; name [u8;24]; weights [u16;8]; core_ratios [u16;8]; rebalance_bps u16; slippage_bps u16; max_daily_trades u16; max_turnover_bps u16 | authority signer+writable; portfolio PDA writable; system program |
| 1 | UpdateStrategy | same 66-byte configuration payload (byte 1 is ignored) | authority signer; portfolio PDA writable |
| 2 | SetStatus | status u8: Draft=0, Running=1, Paused=2, RiskPaused=3, Error=4, Stopped=5 | authority signer; portfolio PDA writable |
| 3 | RecordExecution | execution_id u64; input_value u64; output_value u64; fee_value u64; side u8 (buy=0/sell=1); asset_index u8; portfolio_nav u64 | authority signer; portfolio PDA writable; Clock sysvar |

`input_value`, `output_value`, `fee_value`, and `portfolio_nav` must use the same valuation unit and scale (for example USDC micros). The instruction is 43 bytes including its tag. `execution_id` must be strictly greater than the stored value. Input, output, and NAV must be non-zero; fee cannot exceed input. Slippage is checked as `output >= (input - fee) * (10000 - slippage_bps) / 10000`. UTC-day trade count cannot exceed `max_daily_trades`; UTC-day cumulative input cannot exceed `portfolio_nav * max_turnover_bps / 10000`. All intermediate risk arithmetic is checked or widened to u128.

Oracle freshness, quote authenticity, token-decimal normalization, DEX price impact, realized daily loss, actual custody balances, and allowed CPI programs cannot be proven from this audit-only instruction and remain mandatory executor-side fail-closed checks. The program does not perform a swap CPI.

## State machine

Allowed transitions are:

- Draft → Running or Stopped
- Running → Paused, RiskPaused, Error, or Stopped
- Paused → Running or Stopped
- RiskPaused → Paused or Stopped
- Error → Paused or Stopped
- Stopped → no state; it is terminal and cannot be recovered

Self-transitions and every transition not listed above return `InvalidStatus`.

## Account layout

State is exactly 160 bytes: magic `[0..8]`; authority `[8..40]`; bump `40`; status `41`; asset_count `42`; name `[43..67]`; weights `[67..83]`; core ratios `[83..99]`; risk fields `[99..107]`; version `[107..115]`; last execution id `[115..123]`; lifetime trade count `[123..131]`; current UTC-day turnover `[131..139]`; lifetime fees `[139..147]`; UTC day number `[147..155]`; current UTC-day trade count `[155..157]`; reserved `[157..160]`.

## Errors

| Code | Name |
|---:|---|
| 100 | BadInstruction |
| 101 | BadAccounts |
| 102 | Unauthorized |
| 103 | BadPda |
| 104 | AlreadyInitialized |
| 105 | InvalidWeights |
| 106 | InvalidRiskLimits |
| 107 | InvalidStatus |
| 108 | DuplicateExecution |
| 109 | Overflow |
| 110 | RiskLimitExceeded |

Shared Rust/TypeScript fixtures are in `docs/golden-vectors.json` and are verified by both `program/tests/sbf_litesvm.rs` and `dapp/lib/program.test.ts`.
