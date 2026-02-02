# 🎯 Number Guessing Game

[![Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey.svg)]()

---

## ✨ Can You Read Minds? Prove It!

*A thrilling two-player number guessing game built with pure Rust*

---

## 🚀 Features

| Feature | Description |
|---------|-------------|
| 🎯 **Smart Hints** | Get granular feedback - from "Way too high" to "Very close!" |
| 🕵️ **Privacy Mode** | Screen clears between players so no cheating! |
| 🎲 **Auto-Fallback** | Invalid secret number? No problem - we'll pick a random one! |
| ⚡ **Fast & Lightweight** | Zero dependencies except `rand` - runs instantly! |
| 🌐 **Bilingual Hints** | English + Hindi for that extra flavor! |

---

## 🏃 Quick Start

```bash
# Clone the repo
git clone https://github.com/ZaifMirza/Guessing_Game.git
cd Guessing_Game

# Run the game
cargo run
```

That's it! No installation, no setup, just pure gaming bliss. 🎉

---

## 📖 How to Play

1. **Player 1** enters a secret number (1-500)
2. Screen clears automatically 🙈
3. **Player 2** has **10 attempts** to guess it!
4. Receive hints based on how close you are:
   - 🔴 **Way too high/low** (gap ≥ 250)
   - 🟠 **Too high/low** (gap ≥ 125)
   - 🟡 **Bit high/low** (gap ≥ 10)
   - 🟢 **Very close!** (gap < 10)
5. Win or lose - either way, it's fun! 🏆

---

## 🎯 Hint System

```
┌─────────────────────────────────────────────────────────────────┐
│  GAP SIZE          │  HINT MESSAGE                              │
├────────────────────┼────────────────────────────────────────────┤
│  0                 │  🎉 YOU WON!!                              │
│  1 - 9             │  🟢 Very close to the number               │
│  10 - 124          │  🟡 Bit high / Bit low                     │
│  125 - 249         │  🟠 Too high / Too low                     │
│  250+              │  🔴 Way too high / Way too low             │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🎮 Gameplay Preview

```
========================================
   WELCOME TO THE GUESSING GAME   
========================================
Rules:
1. Guess a number between 1 and 500.
2. You have 10 attempts.
3. Hints will guide you if you are high or low.
4. If you get 'Very Close', you are within ±10.
========================================

Player 1, please enter the secret number (1–500):
[Screen clears...]

Please Input Your Number Gentleman:
250
[Hint] High hai thoda niche aao
Your total guesses left: 9

Please Input Your Number Gentleman:
200
[Hint] You are very close to the number
Your total guesses left: 8

Please Input Your Number Gentleman:
197
You won!! Congratulations 🎉
```

---

## 🛠️ Tech Stack

- **Language**: [Rust](https://www.rust-lang.org/) 🦀
- **Edition**: 2021
- **Dependencies**: 
  - [`rand`](https://crates.io/crates/rand) ^0.8.5

---

## 📁 Project Structure

```
Guessing_Game/
├── src/
│   └── main.rs          # Game logic & UI
├── Cargo.toml           # Project configuration
├── Cargo.lock           # Dependency lock file
├── .gitignore           # Git ignore rules
└── README.md            # This file
```

---

## 💻 Code Overview

### Main Components

#### 1. Guess Enum (`src/main.rs:5-14`)
The game uses a smart enum-based hint system:

```rust
enum Guess {
    Correct,      // 🎉 Perfect guess!
    ToomuchHigh,  // 🔴 Way too high
    TooHigh,      // 🟠 Too high
    BitHigh,      // 🟡 Slightly high
    ToomuchLow,   // 🔴 Way too low
    TooLow,       // 🟠 Too low
    BitLow,       // 🟡 Slightly low
    Veryclose,    // 🟢 Within ±10!
}
```

#### 2. Screen Clearing (`src/main.rs:16-21`)
Clears the terminal screen to prevent Player 2 from seeing the secret number:

```rust
fn clear_screen() {
    // Works on Linux/macOS
    Command::new("clear").status().unwrap();
    // For Windows, uncomment this:
    // Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
}
```

#### 3. Main Game Logic (`src/main.rs:23-139`)
- Welcomes players and displays rules
- Reads secret number from Player 1
- Clears screen for privacy
- Manages 10 attempts for Player 2
- Provides intelligent hints based on guess distance
- Handles win/lose conditions

### Key Functions

| Function | Purpose |
|----------|---------|
| `clear_screen()` | Clears terminal for privacy between players |
| `main()` | Main game loop and logic |

### Game Flow

```
┌─────────────────┐
│  Start Game     │
└────────┬────────┘
         ▼
┌─────────────────┐
│ Player 1 Enters │
│ Secret Number   │
└────────┬────────┘
         ▼
┌─────────────────┐
│  Clear Screen   │
└────────┬────────┘
         ▼
┌─────────────────┐
│  Player 2       │
│  Makes Guess    │
└────────┬────────┘
         ▼
┌─────────────────┐
│  Calculate Gap  │
│  & Show Hint    │
└────────┬────────┘
         ▼
    ┌────┴────┐
    │Win/Lose?│
    └────┬────┘
         │
    ┌────┴────┐
    ▼         ▼
  WIN       LOSE
```

---

## 🤝 Contributing

Got ideas to make this game even better?

1. Fork it 🍴
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request 🎉

---

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- Built with ❤️ using [Rust](https://www.rust-lang.org/)
- Inspired by classic number guessing games
- Special thanks to the Rust community! 🦀

---

<div align="center">

Star this repo if you had fun playing! ⭐

Made with 🦀 Rust and ☕ Coffee

[![GitHub stars](https://img.shields.io/github/stars/ZaifMirza/Guessing_Game?style=social)](https://github.com/ZaifMirza/Guessing_Game/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/ZaifMirza/Guessing_Game?style=social)](https://github.com/ZaifMirza/Guessing_Game/network/members)

</div>
