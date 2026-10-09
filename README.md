# CLI Calculator

A simple interactive command-line calculator written in Rust.

## Features

- Addition (`+`)
- Subtraction (`-`)
- Multiplication (`*`)
- Division (`/`) with division-by-zero handling
- Power (`^`)
- Square root (`v`) with negative-number handling
- Exit commands: `exit` or `q`

## Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install) (includes `cargo`)

## Run

From the repository root:

```bash
cargo run
```

## Build

```bash
cargo build
```

## Test

This project currently has no automated tests.

## Usage

1. Start the app with `cargo run`.
2. Choose an operation (`+`, `-`, `*`, `/`, `^`, or `v`).
3. Enter the first number.
4. If the operation is not `v`, enter the second number.
5. Read the result and continue, or type `exit` / `q` to quit.
