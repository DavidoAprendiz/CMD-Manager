<h1 align="center">CMD-Manager</h1>

<p align="center">
<img alt="CMD-Manager 0.3.0" src="https://img.shields.io/badge/cmd_manager-0.3.0-000000?style=for-the-badge&color=blue">
<img alt="Rust CI" src="https://github.com/DavidoAprendiz/Rust-in-Progress/actions/workflows/rust.yml/badge.svg">
</p>

<p align="center">
</p>

---

<p align="center">
<img alt="CMD-Manage" src="https://github.com/DavidoAprendiz/CMD-Manager/assets/21132833/07356fdb-9ce9-4905-b02a-f7db84287af6">
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
- **Brain Manager**
  - Talk with **your own** AI and save the conversation history to **your own** database and/or to Markdown files.
  - Search and manage all your history.

Fully functional on Linux and Windows!

Feel free to contribute!

## How to use

### Pre-requirements

Rust

- To run the program (without Brain module)
  - Install the latest [Rust](https://www.rust-lang.org/learn/get-started)

Ollama

- To run Brain module
  - Install Llama3 in [Ollama](https://ollama.com/)

### Install / Clone

- Open a terminal and clone the repository
  - `git clone https://github.com/DavidoAprendiz/CMD-Manager.git`
- Enter the directory and run Cargo:
  - `cargo run`

## Roadmap

- [X] **Todo Manager**
  - [X] Add new tasks.
  - [X] View your saved tasks.
  - [X] Delete your saved tasks.
- [X] **File Manager**
  - [X] Search for a "keyword" in any file.
  - [X] Compare any two files and see all differences between them.
- [X] **Web Manager**
  - [X] Download any webpage from the web.
  - [X] Get data from Coingecko APIs.
- [X] Improve TUI (added colors)
- [X] Add SQLite database.
- [X] Basic unit tests (for best practices).
- [ ] **Brain Manager (in-progress)**
  - [X] Brainstorm with your personal AI.
    - You can save the answers directly in a Markdown file.
  - [ ] Search in all your questions and answers.
  - [ ] Manage your 'Brain' history.
  - [X] Manage your 'Security' history.
