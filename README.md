<h1 align="center">CMD-Manager</h1>

<p align="center">
<img alt="CMD-Manager 0.2.1" src="https://img.shields.io/badge/cmd_manager-0.2.1-000000?style=for-the-badge&color=blue">
<img alt="Rust CI" src="https://github.com/DavidoAprendiz/Rust-in-Progress/actions/workflows/rust.yml/badge.svg">
</p>

<p align="center">
</p>

---

<p align="center">
<img alt="CMD-Manage" src="https://github.com/DavidoAprendiz/CMD-Manager/assets/21132833/cf467918-65ea-4c39-9886-d28a4ab5be85">
</p>

## Description

An **AiO(all-in-one)** command-line application (but with colors 😃) focused on privacy because privacy matters!
Made with Rust, SQLite and Ollama.

You can find the following tools:

- **To-do Manager**
  - You can add, view and delete your to-dos/notes in an offline environment.
- **File Manager**
  - Search Mode - Search any character/word/phrase in text documents locally.
  - Compare Mode - Compare any document and see all the differences using Myers algo (similar to Git/Github).
- **Web Manager**
  - Download Mode - Download simple data from web to your local drive.
  - Get API Mode - Get the current price of Ergo and Cardano.
- **Brain Manager (in-progress)**
  - Talk with **your own** AI and save the history to **your own** database.
  - Search and manage all your history.

Fully functional on Linux and Windows!

Feel free to contribute!

## How to use

### Pre-requirements

Rust

- To run the program
  - Install the latest [Rust](https://www.rust-lang.org/learn/get-started)

Ollama

- To run Brain module (in-progress)
  - Install Llama3 in [Ollama](https://ollama.com/)

### Install / Clone

- Open a terminal and clone the repository
  - `git clone https://github.com/DavidoAprendiz/CMD-Manager.git`
- Enter the directory and run Cargo:
  - `cargo run`

## Roadmap

- [X] Todo Manager
  - [X] Manage tasks (add/view/delete).
- [X] File Manager
  - [X] Search in files.
  - [X] Compare two files.
- [X] Web Manager
  - [X] Download files.
  - [X] Get data from APIs.
- [X] Improve TUI (just a little)
- [X] Add SQLite.
- [ ] Unit tests (for best practices).
- [ ] Brain Manager
  - [ ] Brainstorm with your personal AI.
  - [ ] Manage history (search/view/delete).
