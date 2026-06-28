# Rust CLI Calculator

A simple command-line calculator written in Rust.

This project was built as part of my journey learning Rust. Rather than relying on external crates or complex abstractions, the focus is on understanding Rust fundamentals such as modules, functions, user input, pattern matching, error handling, and basic project structure.

---

## Features

- ➕ Addition
- ➖ Subtraction

---

## Project Structure

```
calculator/
├── src/
│   ├── main.rs          # Program entry point
│   └── functions.rs     # Calculator functions and input handling
├── Cargo.toml
└── README.md
```

---

## Getting Started

### Clone the repository

```bash
git clone https://github.com/floatowl/calculator.git
cd calculator
```

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

---

## Example

```
Choose your operation!

1. Addition
2. Subtraction
3. Exit

> 1

Please enter two numbers

10
20

The sum is 30
```
---

## Built With

- Rust
- Cargo