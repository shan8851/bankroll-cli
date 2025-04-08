# 💰 Bankroll CLI 💸

A interactive Rust CLI tool to help poker players manage their bankroll using solid **bankroll management (BRM)** principles. Whether you play **cash games**, **sit & gos**, or **tournaments**, this tool gives you smart, risk-adjusted stake recommendations.

---

## 🎯 Features

- 📈 Risk-adjusted BRM strategy for:
  - **Cash Games**
  - **Sit & Go's**
  - **MTTs (Tournaments)**
- 🧠 Choose your **risk profile**: Conservative, Moderate, Aggressive, or UltraAggressive
- 🎮 Game-type-aware recommendations
- 🪜 Get your current stake + guidance on when to **move up or down**
- 💎 Colourful, friendly UX using `figlet_rs`, `inquire`, and `colored`

---

## 🦀 Why Rust?

This was my first Rush project, so the idea was to keep things simple, but still hit some key ideas and principles of the language:
- Learn idiomatic Rust design: enums, pattern matching, modules, and structs
- Explore interactive CLI development
- Apply practical logic to a familiar domain (poker!)


---

## 🛠️ Installation

### Clone + Run

```sh
git clone https://github.com/shan8851/bankroll-cli.git
cd bankroll-cli
cargo run
```

## 🧪 Example Output

```
💰 Bankroll CLI 💸

💵 Enter your current bankroll:
1500

🎮 What type of game do you play?
> Cash

🚦 What is your risk level?
> Moderate

🧠 Calculating your optimal stake...

🎯 Recommended Stake: NL50
🟢 Move up to NL100 at: $4000.00
🔻 Move down to NL25 if below: $1000.00

Good luck at the tables! ♠️♥️♣️♦️
```

## 📚 Concepts Covered

This project helped me learn:

1. Enums for modelling domain logic

2. Pattern matching over multiple values

3. Modular project structure

4. Public/private visibility and clean API boundaries

4. Styling and interacting via Rust CLI crates

# 📜 License

MIT
