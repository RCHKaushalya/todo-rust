#  todo-rust

A simple command-line to-do list app built with Rust. Supports adding, listing, marking as done, deleting tasks, and clearing your list with confirmation—persisted in a local JSON file.

##  Features

- Add tasks: `cargo run -- add "Buy milk"`
- List tasks: `cargo run -- list`
- Mark as done: `cargo run -- done 1`
- Delete tasks: `cargo run -- delete 1`
- Clear all tasks (with confirmation): `cargo run -- clear`

##  Why Rust?

This project is designed to explore Rust’s ownership model, file I/O, JSON serialization, and safe CLI design.

##  Installation

Clone the repo and build it with Cargo:

bash
git clone https://github.com/YOUR_USERNAME/todo-rust
cd todo-rust
cargo build


* License*

MIT
