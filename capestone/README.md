# 🌅 Stake2Wake — Stake BONK to Beat the Sunrise!

Stake2Wake is a Solana-based decentralized app (dApp) that gamifies early rising by letting users stake BONK tokens into a daily wake-up challenge. If they wake up on time and check in, they keep their stake — if they fail, their BONK is slashed and sent to the protocol treasury.

This project is built using Anchor for smart contracts and React Native (Expo) for the mobile dApp.

---

## 🚨 The Problem

Modern routines often suffer from:
- Poor sleep discipline
- Lack of motivation to wake up early
- Low accountability and gamification in personal habits

Habit-tracking apps exist, but they’re centralized, and users don’t face meaningful consequences for skipping commitments.

---

## 💡 The Solution

Stake2Wake turns morning discipline into a high-stakes habit — literally.

By requiring users to stake BONK tokens against their commitment, we introduce:
- **Incentive** to wake up
- **Accountability** via smart contracts
- **Gamification** through streaks and social comparison (future roadmap)

> If you wake up on time and check in via a simple transaction — you keep your BONK.
> If you fail — your stake is slashed and sent to the protocol treasury.

---

## ⚙️ How It Works

1. **User stakes BONK** and sets a wake-up challenge (e.g., 7:00 AM every day for 7 days).
2. A smart contract locks the funds in a PDA vault.
3. Each morning, the user must **check in within a valid time window** (e.g., 7:00–7:15 AM).
4. If the check-in is on time:
   - Challenge progress is updated.
   - User can withdraw funds after successful completion.
5. If the user **fails to check in**, the staked BONK is **slashed** and routed to the protocol treasury.

---

## 🧱 Components

- 🟡 **Smart Contract** (Anchor on Solana)
  - Challenge initialization
  - BONK staking vault
  - Timestamp-based check-ins
  - Slashing logic
  - Treasury account

- 🟢 **Frontend App** (React Native + Expo)
  - Wallet connection (Phantom, Solflare)
  - Wake-up timer setup
  - "Check-in" button
  - Success/failure feedback
  - Basic challenge stats display

---

## 🔐 Trust & Transparency

- All funds are managed by **on-chain smart contracts**.
- BONK vaults and treasury are PDA-controlled.
- Time validation uses Solana’s system `Clock` program.
- Minimal off-chain infrastructure for MVP — no user accounts or private data stored.

---

## 🚀 Future Roadmap

- 🏆 Leaderboards and streak tracking
- 🎁 NFT badges for challenge completions
- 🟣 Multi-token support (USDC, SHDW, etc.)
- 📱 Notifications (Push protocol / Dialect)
- 🧠 DAO-governed treasury distribution

---
