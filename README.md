# Ticketing-system
# 🎟️ Solana Ticket Platform — 基于 Solana 的去中心化售票系统

> 一个基于 **Solana 区块链 + Anchor + React/Next.js** 的简化票务平台示例。  
> 支持票务发行方上架 NFT 票、购票方购买与退票、自动化资金托管与退款逻辑。  
> 这是一个全栈 DApp 示例，演示 Solana 上的 NFT + Escrow 机制。

---

## 🧠 功能概览

- 🎫 **活动票务发行** — 售票方通过前端创建活动并铸造 NFT 票。
- 🕐 **时间锁发行** — 支持设置票务开放购买的区块时间。
- 💰 **购票与退款** — 用户可通过合约购买与退回 NFT（非销毁，仅状态切换）。
- 🔐 **Escrow 托管机制** — 合约托管资金，确保公平交易。
- 🧾 **NFT 元数据生成** — 票务信息与图片通过 metadata 存储。
- 🪙 **SOL 支付与收益提取** — 售票方可在活动结束后提现收益。

---

## 🧩 技术栈

| Layer | Tech |
|-------|------|
| 智能合约 | 🦀 [Anchor Framework](https://www.anchor-lang.com) (Rust) |
| 区块链网络 | 🌞 [Solana Devnet](https://solana.com/developers) |
| 前端框架 | ⚛️ [Next.js 14](https://nextjs.org/) + TypeScript |
| 样式 | 💨 [Tailwind CSS](https://tailwindcss.com/) |
| 钱包交互 | 🦊 [@solana/wallet-adapter](https://github.com/solana-labs/wallet-adapter) |
| 合约客户端 | 🧱 [@project-serum/anchor](https://github.com/coral-xyz/anchor) |

---

## 🏗️ 项目架构

```
project-root/
├── Anchor.toml
├── programs/
│   └── ticket_platform/
│       └── src/lib.rs
├── target/
│   └── idl/ticket_platform.json
│
└── app/
    ├── package.json
    ├── next.config.js
    ├── tsconfig.json
    ├── public/
    │   └── logo.svg
    └── src/
        ├── components/
        │   ├── WalletConnectButton.tsx
        │   ├── CreateEventForm.tsx
        │   ├── BuyTicketButton.tsx
        │   ├── RefundButton.tsx
        │   └── OrganizerWithdrawButton.tsx
        ├── lib/
        │   ├── anchorClient.ts
        │   ├── pda.ts
        │   ├── metadata.ts
        │   └── utils.ts
        ├── pages/
        │   ├── index.tsx
        │   ├── create.tsx
        │   ├── event/[id].tsx
        │   └── _app.tsx
        └── styles/
            └── globals.css
```

---

## ⚙️ 安装与运行步骤

### 1️⃣ 安装依赖

```bash
# 安装 Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.18.16/install)"

# 安装 Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install latest
avm use latest

# 安装 Node 前端依赖
cd app
yarn install
```

---

### 2️⃣ 构建与部署合约

```bash
anchor build
solana-test-validator
anchor deploy
```

---

### 3️⃣ 启动前端

```bash
cd app
yarn dev
```

访问 [http://localhost:3000](http://localhost:3000)

---

## 💡 前端功能展示

| 页面 | 功能 |
|------|------|
| `/` | 查看活动列表 |
| `/create` | 售票方发布新活动并铸造 NFT |
| `/event/[id]` | 购票方购买、退票，主办方提取收益 |

---

## 🪙 合约核心逻辑

```rust
pub struct Ticket {
    pub organizer: Pubkey,
    pub buyer: Option<Pubkey>,
    pub price: u64,
    pub is_sold: bool,
    pub metadata_uri: String,
}
```

---

## 🧾 环境变量

```env
NEXT_PUBLIC_SOLANA_RPC=https://api.devnet.solana.com
NEXT_PUBLIC_PROGRAM_ID=<your program id>
```

---

## 🧰 调试技巧

```toml
[toolchain]
anchor_version = "0.32.1"
```



[MIT License](LICENSE)
