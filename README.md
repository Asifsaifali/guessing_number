# 🎮 Rust + WebAssembly Guessing Game

A simple "Guess the Number" game written in Rust and compiled to WebAssembly using `wasm-pack`. It features a clean, animated frontend built with HTML and CSS.

---

## 🚀 How It Works

- Rust generates a secret number using `rand`
- WebAssembly exposes two functions: `start_game` and `check_guess`
- Frontend interacts with these using JavaScript
- UI is animated and responsive, with bounce effect on win 🥳

---

## 📦 Prerequisites

- Rust & Cargo
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Python 3 (for local server)
- WSL or Linux/Unix recommended for building

---

## 🛠 Build Instructions

```bash
# Clone or move into your project directory
cd guessing-game

# Build the WASM package
wasm-pack build --target web

# Run local server
python3 -m http.server
