# Yoinky

A lightweight and efficient terminal-based tool for monitoring CPU usage, built with Rust. Yoinky provides a real-time view of your system's CPU activity, leveraging `ratatui` for a clean and responsive terminal user interface and `procfs` to gather system information directly from the `/proc` filesystem.

## Features

- **Real-time CPU Monitoring**: Displays the number of CPU cores.
- **Terminal-Based UI**: A clean, minimal interface built with `ratatui`.
- **Lightweight**: Written in Rust for performance and low resource consumption.

## Technology Stack

- **[Rust](https://www.rust-lang.org/)**: The core programming language, chosen for its safety and performance.
- **[ratatui](https://ratatui.rs/)**: A TUI (Terminal User Interface) library for creating rich, interactive terminal applications.
- **[crossterm](https://github.com/crossterm-rs/crossterm)**: A terminal manipulation library used as a backend for `ratatui`.
- **[procfs](https://github.com/eminence/procfs)**: A library to access system and process information from the `/proc` filesystem on Linux.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (and Cargo) installed on your system.

### Installation & Running

1.  Clone the repository:

    ```bash
    git clone <repository-url>
    cd yoinky
    ```

2.  Run the application:

    ```bash
    cargo run
    ```

3.  Press `q` to exit the application.

---

_This is a project in early development. More features like detailed core usage, process lists, and memory information are planned for the future!_
