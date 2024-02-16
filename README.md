# Online Tic-Tac-Toe

## Description

This is a simple online tic-tac-toe game following a client-server architecture.

## Server

When a client connects, handles it's connection through a thread in a thread pool. The server is responsible for managing the game state and sending updates to the clients. Only one game can be played at a time.


It is responsible for the following tasks:
- Managing the game state.
- Sending updates to both clients when a move is made.
- Handling disconnections.

```sh
cargo run --bin server -- <PORT>
```

## Client

The client is a simple TUI that displays the game board and waits for the user to make a move.

```sh
cargo run --bin client -- <IP>:<PORT>
```

## Purpose

This project was made to learn more about networking and multithreading in **Rust**, as well as to learn how to use the [serde](https://crates.io/crates/serde) crate to serialize and deserialize data.