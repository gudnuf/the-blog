---
title: "Getting Started with Rust"
slug: "getting-started-with-rust"
date: 2025-01-15
author: "Your Name"
description: "A beginner's guide to the Rust programming language, covering installation, basic concepts, and your first program."
tags: ["rust", "programming", "tutorial", "beginner"]
category: "programming"
template: "post"
draft: false
toc: true
---

# Getting Started with Rust

Welcome to this introduction to Rust! In this post, we'll cover the basics of getting started with this powerful systems programming language.

## Why Rust?

Rust offers several compelling features:

- **Memory safety** without garbage collection
- **Zero-cost abstractions** for high performance
- **Fearless concurrency** with compile-time guarantees
- A helpful compiler with great error messages

## Installation

The easiest way to install Rust is using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, verify it works:

```bash
rustc --version
cargo --version
```

## Your First Program

Let's write the classic "Hello, World!" program:

```rust
fn main() {
    println!("Hello, world!");
}
```

Save this as `main.rs` and compile it:

```bash
rustc main.rs
./main
```

## Using Cargo

For real projects, you'll want to use Cargo, Rust's package manager and build tool:

```bash
cargo new my_project
cd my_project
cargo run
```

### Project Structure

A new Cargo project looks like this:

```
my_project/
├── Cargo.toml
└── src/
    └── main.rs
```

The `Cargo.toml` file contains your project's metadata and dependencies:

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
```

## Variables and Data Types

Rust is statically typed but often infers types:

```rust
fn main() {
    // Immutable by default
    let x = 5;

    // Mutable variables use `mut`
    let mut y = 10;
    y = 20;

    // Explicit type annotation
    let z: i32 = 42;

    println!("x = {}, y = {}, z = {}", x, y, z);
}
```

### Common Data Types

| Type | Description | Example |
|------|-------------|---------|
| `i32` | 32-bit signed integer | `let x: i32 = -42;` |
| `u64` | 64-bit unsigned integer | `let y: u64 = 100;` |
| `f64` | 64-bit floating point | `let z: f64 = 3.14;` |
| `bool` | Boolean | `let b: bool = true;` |
| `char` | Unicode character | `let c: char = 'a';` |
| `String` | Owned string | `let s = String::from("hello");` |

## Functions

Functions are declared with `fn`:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = return value
}

fn main() {
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
}
```

## What's Next?

This is just the beginning! In future posts, we'll explore:

- Ownership and borrowing
- Structs and enums
- Error handling with `Result` and `Option`
- Traits and generics
- Async programming with Tokio

Happy coding! 🦀
