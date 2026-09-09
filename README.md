# xStocks Portfolio Autopilot

Pinocchio-based Solana devnet program and Next.js dashboard for an auditable, non-custodial xStocks portfolio strategy. The on-chain program stores versioned allocation/risk configuration, controls the strategy state machine, and records idempotent execution summaries. Quotes, oracle checks and swaps remain client/execution-service responsibilities; the MVP UI defaults to paper/read-only mode.

See `docs/ABI.md` and `docs/项目使用说明书.md`.

