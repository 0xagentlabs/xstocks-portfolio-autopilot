# xStocks Portfolio Autopilot
## MVP 产品需求说明书

**版本：** v0.1  
**阶段：** MVP  
**产品形态：** Web App + Strategy Engine + Execution Service  
**目标网络：** Solana First  
**核心资产：** xStocks / USDC

---

# 1. 产品概述

## 1.1 一句话定义

**一个面向个人投资者的 xStocks 自动组合管理工具：用户选择股票和目标仓位，系统自动监控持仓、调仓，并利用部分交易仓位自动做 T。**

长期可演进为：

> Describe your strategy. Let it run.

用户通过自然语言描述自己的投资想法，由 AI 将其转换为结构化策略，程序负责确定性执行。

---

# 2. 产品背景

xStocks 将股票资产 Token 化，使 NVDAx、TSLAx、SPYx 等资产能够直接存在于链上钱包中。

这带来了传统券商账户较难实现的一种使用方式：

- 链上持有股票资产；
- 程序直接读取仓位；
- 自动交易；
- 自动组合管理；
- 自动调仓；
- 与 USDC 等链上资产组合；
- 未来进一步接入 Lending、Vault 等 DeFi 协议。

目前个人管理 xStocks 时仍然需要大量人工操作。

典型场景：

```text
观察价格
   ↓
判断是否买入
   ↓
手动 Swap
   ↓
记录成本
   ↓
价格上涨
   ↓
判断是否卖出
   ↓
再次 Swap
   ↓
重新计算仓位
```

如果同时管理 NVDAx、TSLAx、SPYx、GOOGLx 等多个资产，操作复杂度迅速增加。

因此，本项目希望将这些重复工作程序化。

---

# 3. 核心问题

产品第一阶段不解决：

> AI 能不能预测股票？

而解决：

> **能不能把已经明确的个人投资策略可靠地自动执行？**

主要解决三个问题。

### 3.1 Portfolio Management

用户希望长期持有多个 xStocks，但不同股票价格变化会导致组合权重偏离目标。

例如目标：

```text
NVDAx    40%
TSLAx    30%
SPYx     30%
```

一段时间后：

```text
NVDAx    48%
TSLAx    24%
SPYx     28%
```

系统应能够识别偏离，并根据规则自动恢复目标配置。

---

### 3.2 自动做 T

用户长期看好某个资产，但希望利用短期价格波动降低持仓成本。

例如：

```text
NVDAx 总目标仓位

Core Position      80%
Trading Position   20%
```

Core Position 原则上长期持有。

Trading Position 用于：

```text
下跌 → 买入
反弹 → 卖出
```

系统自动维护交易仓位，并记录实际产生的收益。

---

### 3.3 多资产统一管理

用户不希望分别维护：

```text
NVDA 策略
TSLA 策略
SPY 策略
...
```

系统需要提供 Portfolio 层统一管理：

```text
Portfolio
   │
   ├── NVDAx
   ├── TSLAx
   ├── SPYx
   └── USDC
```

同时管理：

- 总资产；
- 目标权重；
- 当前权重；
- Core Position；
- Trading Position；
- 可用 USDC；
- 调仓状态；
- 做 T 状态；
- 收益。

---

# 4. MVP 核心原则

## 4.1 AI 不直接控制资金

系统采用：

```text
LLM
 ↓
Strategy Configuration
 ↓
Risk Validation
 ↓
Deterministic Engine
 ↓
Execution
```

LLM 可以帮助用户生成策略，但最终交易由确定性程序执行。

---

## 4.2 Research 与 Execution 分离

系统严格区分：

```text
Strategy Decision
        ≠
Trade Execution
```

AI 可以提出：

```text
NVDAx target = 40%
TSLAx target = 30%
SPYx  target = 30%
```

但具体：

```text
卖多少？
买多少？
什么时候执行？
允许多大滑点？
使用哪个 Route？
```

由 Execution Engine 决定。

---

## 4.3 不追求高频交易

MVP 不做 HFT。

目标是：

```text
分钟 / 小时 / 日级策略
```

重点是自动化 Portfolio Management，而不是速度竞争。

---

# 5. 产品边界

## MVP 包含

- Solana Wallet
- xStocks 资产识别
- USDC
- Portfolio 创建
- Target Allocation
- Portfolio Dashboard
- Core / Trading Position
- 自动调仓
- 自动做 T
- DEX Quote
- 自动 Swap
- 风控
- PnL
- Strategy PnL Attribution
- 交易记录
- 策略暂停
- Emergency Stop

## MVP 暂不包含

- ERC-20 / SPL Portfolio Token
- 公募 ETF
- Vault Share
- Strategy Marketplace
- Copy Trading
- Social
- Creator Economy
- Lending
- Leverage
- Options
- Cross-chain
- 多用户资金池
- Agent 自主选股
- 高频交易

核心原则：

> **先证明个人工具有价值，再产品化资金管理。**

---

# 6. 用户流程

## 6.1 首次使用

```text
Connect Wallet
      ↓
读取 xStocks / USDC
      ↓
Create Portfolio
      ↓
选择 Assets
      ↓
设置 Target Weight
      ↓
设置 Trading Strategy
      ↓
Risk Check
      ↓
Preview
      ↓
Start Strategy
```

---

# 7. Portfolio

用户可以创建一个 Portfolio。

例如：

```text
AI + Growth Portfolio

Total Value
$10,000

Holdings

NVDAx    $4,000    40%
TSLAx    $3,000    30%
SPYx     $2,500    25%
USDC       $500     5%
```

---

# 8. Target Allocation

用户为每个资产设置目标权重。

例如：

```text
NVDAx    40%
TSLAx    30%
SPYx     25%
USDC      5%
```

必须满足：

```text
Σ weights = 100%
```

系统实时计算：

```text
Target Weight
Current Weight
Deviation
```

例如：

| Asset | Target | Current | Deviation |
|---|---:|---:|---:|
| NVDAx | 40% | 46% | +6% |
| TSLAx | 30% | 26% | -4% |
| SPYx | 25% | 23% | -2% |
| USDC | 5% | 5% | 0% |

---

# 9. Rebalance Engine

## 9.1 基本逻辑

系统周期性计算：

```text
current_weight - target_weight
```

如果偏差超过 threshold，则进入调仓候选状态。

例如：

```text
rebalance_threshold = 5%
```

NVDAx：

```text
Target     40%
Current    46%

Deviation  +6%
```

触发调仓。

---

## 9.2 调仓目标

系统计算：

```text
Target Value =
Portfolio NAV × Target Weight
```

然后：

```text
Delta =
Target Value - Current Value
```

生成：

```text
SELL NVDAx
BUY TSLAx
BUY SPYx
```

---

## 9.3 使用新增资金调仓

新增 USDC 不应机械按照目标比例购买。

优先购买低于目标仓位的资产。

例如：

```text
NVDAx

Target  40%
Current 46%
```

新资金不购买 NVDAx。

优先购买：

```text
TSLAx
SPYx
```

以降低交易成本。

---

## 9.4 Redeem / Sell Netting

如果存在卖出需求，应优先减少 overweight asset。

目标：

> 尽可能通过用户资金流实现自然调仓。

降低：

- turnover；
- slippage；
- swap fee。

---

# 10. 自动做 T

这是 MVP 的核心差异化能力。

每个资产划分：

```text
Core Position
+
Trading Position
```

例如：

```text
NVDAx

Core       80%
Trading    20%
```

---

# 11. Core Position

Core Position 表示长期战略仓位。

默认情况下：

```text
禁止 T Engine 卖出 Core Position
```

例如持有：

```text
100 NVDAx
```

配置：

```text
Core = 80%
```

则：

```text
80 NVDAx
```

原则上不允许 T Strategy 使用。

---

# 12. Trading Position

剩余：

```text
20 NVDAx
```

作为 Trading Position。

用于短期波动交易。

---

# 13. T Strategy V1

MVP 第一版采用简单、可解释的规则策略。

例如：

```text
reference_price = 100

buy_step  = -3%
sell_step = +3%
```

价格：

```text
100
 ↓
97
```

触发：

```text
BUY
```

例如使用 Trading Capital 的：

```text
25%
```

如果继续：

```text
97
 ↓
94
```

再次触发：

```text
BUY
```

---

价格反弹：

```text
94
 ↓
97
```

卖出对应 Trading Position。

形成：

```text
Buy Low
   ↓
Sell High
   ↓
Trading Profit
```

---

# 14. 多级 Grid

支持配置：

```text
Level 1    -3%
Level 2    -6%
Level 3    -9%
Level 4   -12%
```

对应：

```text
25%
25%
25%
25%
```

Trading Capital。

例如：

```text
Trading Capital = $1,000
```

则：

```text
-3%    BUY $250
-6%    BUY $250
-9%    BUY $250
-12%   BUY $250
```

---

# 15. Reference Price

Reference Price 是 T Strategy 的关键状态。

不能简单永远使用第一次启动价格。

系统至少支持：

```text
Last Trade Price
Daily Open
Moving Average
Manual Reference
```

MVP 推荐：

```text
Last Executed Trade Price
```

后续通过回测比较不同 Reference Model。

---

# 16. 趋势保护

简单 Grid 最大风险是：

> 单边下跌不断买入。

因此必须存在 Trend Protection。

例如：

```text
如果价格跌破 MA20
且 MA20 < MA60
```

暂停新的 Trading Buy。

状态：

```text
T Strategy

PAUSED
Reason:
Downtrend protection triggered
```

MVP 可以先实现简单规则。

---

# 17. 单日风险限制

每个 Strategy 设置：

```text
max_daily_trades
max_daily_turnover
max_daily_loss
max_position
```

例如：

```text
Max Daily Trades      10
Max Daily Turnover    20%
Max Trading Capital   20%
Max Single Asset      50%
```

超过限制：

```text
STOP EXECUTION
```

---

# 18. Execution Engine

执行层负责：

```text
Strategy Intent
      ↓
Quote
      ↓
Risk Check
      ↓
Slippage Check
      ↓
Execute
      ↓
Confirm
      ↓
Record
```

---

# 19. Quote

每笔交易执行前必须获取 Quote。

记录：

```text
Input
Output
Expected Price
Price Impact
Fee
Route
Timestamp
```

如果：

```text
price_impact > threshold
```

不执行。

---

# 20. Slippage Protection

用户设置：

```text
max_slippage
```

例如：

```text
0.5%
```

如果预计执行价格超过限制：

```text
REJECT
```

---

# 21. Price / Oracle

系统不能完全依赖单一 DEX Spot Price。

价格体系建议：

```text
Reference Price
      │
      ├── Oracle
      ├── DEX Quote
      └── TWAP
```

出现异常偏差时：

```text
Pause Trading
```

---

# 22. Wallet 权限

MVP 优先采用：

```text
Non-custodial
```

需要重点设计：

> 自动交易如何获得有限权限，而不是让服务器永久持有用户主钱包私钥。

禁止：

```text
上传 Wallet Private Key
```

到普通 Web Backend。

自动执行权限应限制：

```text
Allowed Assets
Allowed Programs
Max Trade Size
Daily Limit
Expiration
Emergency Revoke
```

---

# 23. Strategy State Machine

每个 Strategy 至少有：

```text
DRAFT
RUNNING
PAUSED
RISK_PAUSED
ERROR
STOPPED
```

只有：

```text
RUNNING
```

允许产生交易。

---

# 24. Emergency Stop

Dashboard 必须存在明显的：

```text
STOP STRATEGY
```

点击后：

```text
禁止生成新订单
禁止自动调仓
禁止自动 T
```

已有资产不自动卖出。

---

# 25. Dashboard

首页核心不是“股票涨跌”。

而是：

> **Autopilot 到底给用户创造了什么价值？**

建议：

```text
Portfolio Value

$10,382

Total PnL
+$382

────────────────────

Buy & Hold PnL
+$291

Trading Alpha
+$73

Rebalance Alpha
+$31

Trading Costs
-$13

────────────────────

Net Strategy Alpha
+$91
```

---

# 26. PnL Attribution

必须把收益拆开。

至少：

```text
Market PnL
Trading PnL
Rebalance PnL
Fees
Slippage
Net PnL
```

核心指标：

```text
Strategy Alpha =
Actual Portfolio Return
-
Benchmark Return
```

Benchmark 第一版：

```text
Buy & Hold
```

---

# 27. 为什么 PnL Attribution 是核心功能

如果只告诉用户：

```text
今天赚了 $100
```

用户无法判断：

> 是股票本身涨了，还是程序有价值？

必须能够回答：

```text
如果什么都不做：
+$72

使用 Autopilot：
+$100

Autopilot Contribution：
+$28
```

否则无法验证产品价值。

---

# 28. Trade History

每笔交易记录：

```text
Timestamp
Asset
Side
Amount
Price
Reason
Strategy
Fee
Slippage
Transaction
PnL
```

其中 Reason 非常重要。

例如：

```text
BUY NVDAx

Reason:
T Strategy Level 2

Reference:
$180.20

Trigger:
-6%

Execution:
$169.31
```

用户必须能够理解：

> 为什么程序动了我的钱？

---

# 29. Strategy Configuration

第一版采用结构化配置。

例如：

```yaml
portfolio:
  name: ai-growth

assets:

  NVDAx:
    target_weight: 0.40
    rebalance_band: 0.05

    core_ratio: 0.80

    trading:
      enabled: true
      buy_step: 0.03
      sell_step: 0.03
      levels: 4

  TSLAx:
    target_weight: 0.30
    rebalance_band: 0.07

    core_ratio: 0.70

    trading:
      enabled: true
      buy_step: 0.05
      sell_step: 0.05

  SPYx:
    target_weight: 0.25
    core_ratio: 0.90

cash:
  target_weight: 0.05

risk:
  max_daily_turnover: 0.20
  max_slippage: 0.005
```

这实际上已经形成：

> **Strategy DSL**

---

# 30. AI Strategy Builder

MVP 后半阶段加入。

用户输入：

> 我长期看好 NVDA、Google 和 Tesla。NVDA 仓位最大，Tesla 波动大所以少一点。保留 10% USDC。每只股票拿 20% 仓位自动做 T。

LLM 输出：

```text
Strategy Draft
```

展示：

```text
NVDAx   40%
GOOGLx  30%
TSLAx   20%
USDC    10%
```

以及：

```text
Core / Trading
Rebalance Band
Grid
Risk Limits
```

用户：

```text
Review
   ↓
Confirm
   ↓
Run
```

LLM 不直接提交交易。

---

# 31. 系统架构

```text
                Web UI
                   │
                   ▼
            Portfolio API
                   │
        ┌──────────┼──────────┐
        ▼          ▼          ▼
   Portfolio    Strategy     PnL
    Engine       Engine     Engine
        │          │
        └────┬─────┘
             ▼
       Risk Engine
             │
             ▼
     Execution Engine
             │
      ┌──────┼──────┐
      ▼      ▼      ▼
    Quote  Oracle  DEX
             │
             ▼
          Solana
             │
      ┌──────┴──────┐
      ▼             ▼
   xStocks         USDC
```

---

# 32. Scheduler

后台 Scheduler 周期执行：

```text
Price Update
Portfolio Update
Strategy Evaluation
Rebalance Evaluation
T Strategy Evaluation
Risk Check
Execution
PnL Calculation
```

注意：

**监控频率 ≠ 交易频率。**

例如：

```text
Price Check        1 min
Strategy Check     1 min
Portfolio Refresh  5 min
Rebalance Check    1 hour
PnL                5 min
```

具体参数后续通过实盘数据调整。

---

# 33. 数据模型

核心对象：

```text
User
Wallet
Portfolio
PortfolioAsset
Strategy
StrategyVersion
Position
Order
Trade
Price
PnLSnapshot
RiskEvent
```

Strategy 必须 versioned。

例如：

```text
Strategy v1
   ↓
用户修改
   ↓
Strategy v2
```

所有交易记录对应：

```text
strategy_version
```

保证后续可以完整审计。

---

# 34. Backtest / Paper Trading

正式 Auto Trade 前，建议加入：

```text
Paper Mode
```

运行：

```text
真实价格
+
真实 Quote
+
模拟成交
```

但：

```text
不发送链上交易
```

至少观察：

```text
7D
30D
```

结果。

---

# 35. MVP 最重要的实验

不要先验证：

> 用户喜欢不喜欢 AI。

验证：

> **自动策略扣除所有交易成本以后，是否优于简单 Buy & Hold。**

必须计算：

```text
Gross Trading Alpha
      -
DEX Fees
      -
Slippage
      -
Network Fees
      -
Failed Transaction Cost
      =
Net Strategy Alpha
```

如果：

```text
Net Strategy Alpha <= 0
```

则自动做 T 的产品假设需要重新评估。

不能通过增加 AI 功能掩盖这个问题。

---

# 36. 第一阶段测试 Portfolio

建议只选择 3 个资产。

例如：

```text
NVDAx
TSLAx
SPYx
```

原因：

```text
NVDAx → Growth / AI
TSLAx → High Volatility
SPYx  → Benchmark-like asset
```

便于观察不同波动特征下策略表现。

---

# 37. MVP 页面

第一版只需要 5 个核心页面。

### Dashboard

展示：

```text
Portfolio Value
PnL
Strategy Alpha
Positions
Strategy Status
Recent Trades
```

### Portfolio

管理：

```text
Assets
Target Weight
Current Weight
Core / Trading
```

### Strategy

管理：

```text
Rebalance
T Strategy
Grid
Risk
```

### Trades

查看：

```text
Orders
Trades
Reason
PnL
Transaction
```

### Settings

管理：

```text
Wallet
Execution
Slippage
Risk
Emergency Stop
```

---

# 38. 非功能需求

## Security

- 不在普通数据库存储主钱包私钥；
- 自动交易权限最小化；
- 所有交易前进行 risk validation；
- 所有 Strategy 修改保留版本；
- 所有自动交易可审计；
- 支持一键暂停；
- RPC / Quote 异常时 fail closed。

## Reliability

Execution Engine 必须考虑：

```text
RPC Timeout
Quote Expired
Transaction Failed
Transaction Pending
Duplicate Execution
Price Changed
Insufficient Balance
Partial Execution
```

交易执行必须具备：

```text
Idempotency
```

避免任务重试导致重复交易。

---

# 39. MVP 成功指标

第一阶段不以 TVL、用户量作为核心 KPI。

核心指标：

### Strategy Performance

```text
Net Strategy Alpha
```

### Cost

```text
Trading Cost / AUM
```

### Execution

```text
Successful Execution Rate
```

### Automation

```text
Manual Intervention Rate
```

### Reliability

```text
Unexpected Trade = 0
```

其中最后一项是硬指标。

---

# 40. MVP 开发顺序

## Phase 0 — Simulator

先不要碰真实资金。

实现：

```text
xStocks Price
      ↓
Portfolio Simulator
      ↓
T Strategy
      ↓
Rebalance Engine
      ↓
PnL Attribution
```

验证策略。

---

## Phase 1 — Read Only

连接真实 Wallet：

```text
Wallet
 ↓
读取 xStocks
 ↓
Portfolio Dashboard
 ↓
Strategy Simulation
```

不执行交易。

---

## Phase 2 — Manual Execution

程序产生：

```text
BUY NVDAx $200
```

用户点击：

```text
Confirm
```

钱包签名。

验证 execution pipeline。

---

## Phase 3 — Auto Execution

增加受限自动执行权限。

实现：

```text
Strategy
 ↓
Risk Engine
 ↓
Execution Engine
 ↓
Automatic Trade
```

---

## Phase 4 — AI Strategy

最后增加：

```text
Natural Language
       ↓
LLM
       ↓
Strategy DSL
       ↓
Simulation
       ↓
User Confirm
       ↓
Run
```

---

# 41. 产品演进

如果个人工具验证成功：

```text
Personal Portfolio
        ↓
Auto T
        ↓
Auto Rebalance
        ↓
AI Strategy
        ↓
Share Strategy
        ↓
Copy Strategy
        ↓
Strategy Marketplace
```

再进一步：

```text
Strategy
   ↓
Vault
   ↓
Portfolio Share Token
   ↓
SPL / ERC-20
```

最终才可能形成：

```text
“一句话发行 ETF”
```

即：

```text
Prompt
 ↓
Research
 ↓
Portfolio
 ↓
Strategy
 ↓
Vault
 ↓
Token
 ↓
Invest
```

因此，“一句话 ETF”应该被视为产品长期演进结果，而不是 MVP 的起点。

---

# 42. 核心产品判断

本项目第一阶段真正需要回答的问题只有三个：

**问题一：**

> 能否把个人手工做 T 的经验转换成确定性的程序规则？

**问题二：**

> 扣除手续费、滑点和执行损耗后，自动策略是否仍然产生正的增量收益？

**问题三：**

> 当同时持有 3～10 个 xStocks 时，自动管理是否显著降低用户操作成本？

如果三个答案都是 Yes：

产品就有继续发展的基础。

如果自动做 T 无法稳定产生 Net Alpha，但自动 Portfolio Management 明显降低操作成本，则应该及时调整定位为：

> **xStocks Portfolio Automation**

而不是继续强化“套利”。

---

# 43. 最终产品定义

MVP：

> **xStocks Portfolio Autopilot — 自动管理你的链上股票组合。**

核心闭环：

```text
Choose Stocks
     ↓
Set Allocation
     ↓
Set T Strategy
     ↓
Run
     ↓
Auto Rebalance
     +
Auto Trading
     ↓
Measure Alpha
```

未来：

> **Describe a strategy. Let it run.**

再未来：

> **Describe a thesis. Launch an ETF.**

整个产品应按照：

**个人工具 → 策略工具 → 自动化资产管理 → Strategy Marketplace → Tokenized Portfolio**

逐步验证，而不是从第一天构建一个完整的“AI 基金平台”。