# xStocks Dev Path

一份面向工程团队的 Solana xStocks 开发路线图。网页将资产模型、Token-2022、组合数据、Jupiter 交易、Pinocchio 自动化、Oracle 风控和渐进上线拆成 6 个阶段、18 个模块，每项均带权威文档入口与退出门禁。

仓库同时保留已部署的 Pinocchio Autopilot 参考程序、ABI 和测试，供路线图第 4 阶段对照。

## 本地运行

```bash
cd dapp
pnpm install --frozen-lockfile
pnpm test
pnpm dev
```

生产构建：`cd dapp && pnpm build`。完整说明见 `docs/项目使用说明书.md`，合约接口见 `docs/ABI.md`。

