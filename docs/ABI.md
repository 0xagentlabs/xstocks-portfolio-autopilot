# Program ABI

Program: `6Uzr4jz1SENxn3DdprQQ24zxaJXr6rThQNB48QxbuJed`; integers are little-endian. Portfolio PDA: `["portfolio", authority]`.

| Tag | Instruction | Data after tag | Accounts |
|---:|---|---|---|
| 0 | Initialize | bump u8, asset_count u8, name [u8;24], weights [u16;8], core_ratios [u16;8], rebalance_bps u16, slippage_bps u16, max_daily_trades u16, max_turnover_bps u16 | authority signer+writable; portfolio PDA writable; system program |
| 1 | UpdateStrategy | same 66-byte configuration payload | authority signer; portfolio PDA writable |
| 2 | SetStatus | status u8: draft=0, running=1, paused=2, risk_paused=3, error=4, stopped=5 | authority signer; portfolio PDA writable |
| 3 | RecordExecution | execution_id u64, input_amount u64, output_amount u64, fee u64, side u8 (buy=0/sell=1), asset_index u8 | authority signer; portfolio PDA writable |

State is exactly 160 bytes: magic `[0..8]`, authority `[8..40]`, bump `40`, status `41`, asset_count `42`, name `[43..67]`, weights `[67..83]`, core ratios `[83..99]`, risk fields `[99..107]`, version `[107..115]`, last execution id `[115..123]`, trade count `[123..131]`, turnover `[131..139]`, fees `[139..147]`, reserved `[147..160]`.

Errors 100–109: bad instruction, bad accounts, unauthorized, bad PDA, already initialized, invalid weights, invalid risk limits, invalid status, duplicate execution, arithmetic overflow.

