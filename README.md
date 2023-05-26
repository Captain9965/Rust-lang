# Getting started
## Why Rust?

- It is extremely fast and requires less memory
- Memory safety as it natively handles garbage collection for you
- Safe concurrency as it natively allows multiple parts of the program to run simultaneously.
- It is platform independent
## Installing Rust:
Install rustc (rust compiler) via the following command on linux:

> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

For windows users, visit [this](https://rustup.rs/#) website and run `rustup-init.exe`

One could build rust crates (yes, rust files and libraries are called crates) with rustc but `Cargo` is preferrable since it handles all build configurations and dependencies.

## Installing Cargo:
To install Cargo, follow the instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)

To start a new package with Cargo, run:
> `cargo new hello_world`

To build:
> `cargo build`

To run executable, change directory and run as follows:
> ` ./target/debug/hello_world`
