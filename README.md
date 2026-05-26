<div align="center">

# rentaswap
**The secondary market for subscriptions on Solana.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Solana](https://img.shields.io/badge/Solana-Mainnet-green.svg)](https://solana.com)

Sell the time you don't use; buy the time you don't want to commit to.
StubHub built ticket resale for events. We built it for subscriptions.

</div>

---

## What is rentaswap?

The average user holds 7–12 active subscriptions and actively uses only 3–4. Roughly 30% of all SaaS spend is "forgot to cancel" dead weight. Cancelling only stops future charges; the remaining paid days evaporate.

Rentaswap transforms unused subscription time into a liquid asset. By wrapping a Solana subscription delegation into a transferable NFT receipt, users can sell their remaining time for USDC via trustless atomic swaps.

## Core Mechanics

| Feature | Description |
|---------|-------------|
| **Wrap & List** | Convert an active subscription delegation into a transferable NFT receipt and set a USDC ask price. |
| **Atomic Swap** | Buyers pay USDC, receiving the NFT receipt and delegation rights instantly in a single transaction. |
| **Non-Custodial** | Escrow is managed entirely via on-chain Solana programs. No centralized clearinghouse. |
| **Anti-Gaming** | Live on-chain delegation balance checks prevent the listing or trading of drained/expired subs. |

## Token & Holder Tiers ($RENTASWAP)

Holding `$RENTASWAP` unlocks tiered access and reduces trading fees. 50% of protocol fees are directed to a weekly buyback and burn engine.

| Tier | Min Hold | Perks |
|------|----------|-------|
| **Public** | 0 | 2.0% fee, basic listing |
| **Saver** | ≥100k | 1.5% fee, price alerts, watchlist |
| **Trader** | ≥1M | 1.0% fee, featured slots, bulk-list |
| **Whale** | ≥10M | 0.5% fee, verified badge, governance |

## How It Works

```text
┌──────────────┐    ┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│  1. WRAP     │───▶│  2. LIST     │───▶│  3. SWAP     │───▶│  4. SETTLE   │
│              │    │              │    │              │    │              │
│ delegation   │    │ set ask      │    │ buyer        │    │ USDC →       │
│ → NFT        │    │ price in     │    │ pays         │    │ seller,      │
│ receipt      │    │ USDC         │    │ atomic       │    │ delegation   │
│              │    │              │    │ swap         │    │ → buyer      │
└──────────────┘    └──────────────┘    └──────────────┘    └──────────────┘
```

## Tech Stack

| Domain | Technology | Purpose |
|--------|------------|---------|
| **Frontend** | Next.js 16, React 19, Tailwind v4 | UI/UX & Marketplace Dashboard |
| **On-Chain** | Rust (Anchor Framework), SPL Token-2022 | Escrow Program, Atomic Swaps, NFT Receipts |
| **Backend/DB** | PostgreSQL (Realtime enabled) | Off-chain Order Book & Indexing |
| **RPC & Webhooks**| Helius | Transaction Listeners & Settlement Verify |
| **Bot Integration**| grammY (Telegram) | Notifications & Mini App Interface |

## Phased Roadmap

- **Phase 1 (MVP):** Marketplace live, wrap/sell/buy mechanics, 50-100 pre-seeded listings, fair launch on PumpFun.
- **Phase 2 (Growth):** Auto-detect delegations, Merchant opt-in SDK, transfer webhooks, bidding order book.
- **Phase 3 (Scale):** Sub-bundles, verified merchant program, DAO governance.
- **Phase 4 (Ecosystem):** Cross-chain delegations, third-party marketplace API, subscription-backed credit.

## Reference Links

| Resource | Link |
|----------|------|
| **Website** | [rentaswap.com](https://rentaswap.com) |
| **Telegram** | [@rentaswap](https://t.me/rentaswap) |
| **Docs** | [rentaswap-docs](https://rentaswap.com/) |

---
<div align="center">
<sub>$RENTASWAP is a community token for the rentaswap marketplace. It is not an investment, security, or financial product. Marketplace activity involves on-chain transactions with inherent risk. Do your own research.</sub>
</div>
