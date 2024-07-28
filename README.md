<h1 align="center">CMD-Manager</h1>

<div align="center">
<img alt="Rust CI" src="https://github.com/DavidoAprendiz/Rust-in-Progress/actions/workflows/rust.yml/badge.svg">
<br>
<img alt="CMD-Manager 0.4.0" src="https://img.shields.io/badge/cmd_manager-0.3.3-000000?style=for-the-badge&color=blue">
<br>
<img alt="Made with Rust and SQLite" src="https://skillicons.dev/icons?i=rust,sqlite">
<img alt="ollama" height="50px" src="https://github.com/ollama/ollama/assets/3325447/0d0b44e2-8f4a-4e99-9b52-a5c1c741c8f7">

<a>Made with Rust, SQLite and Ollama</a>

<div align="center">
• <a href="#-description">⚡ Description</a> •
  <a href="#-how-to-use">🚀 How to use</a> •
  <a href="#-or-if-you-want-to-changeedit-the-application">📥 Install</a> •
  <a href="#-manage-the-database-via-graphical-interface-gui">⛁ Manage Database</a> •
  <a href="#CONTRIBUTING.md">💻 Contributing</a> •
  <a href="#LICENSE">📃 License</a> •
  <a href="#LICENSE">✅ Roadmap</a> •
</div>

---

![cmd](https://github.com/DavidoAprendiz/CMD-Manager/assets/21132833/fb8dcfe3-9e21-49c1-aab9-b73984f29e54)

![db](https://github.com/user-attachments/assets/e34ee34b-3cc1-41d3-83aa-ee33c6ce75fe)

</div>

## ⚡Description

A **multi-application** command-line powered by AI, focused on privacy (because privacy matters!).

From the simple to-do/note taking app to your own personal assistant, all within the comfort of your database!

### **To-do Manager**

- You can add, view and delete your to-dos/notes in an offline environment.
- All your to-dos/notes are saved in the database!

### **File Manager**

- Search Mode - Search any character/word/phrase in text documents locally (similar to fzf tool).
- Compare Mode - Compare any document and see all the differences using Myers algo (similar to Git/Github diff tool).

### **Web Manager**

- Download Mode - Download simple data from web to your local drive.
- Get API Mode - Get the current price of Ergo and Cardano from Coingecko API.
- All your downloads are saved in the folder '/Project/Web'

### **Brain Manager**

- Talk with **your own** AI personal assistant and save all conversation histories in **your own** database **and/or** to markdown files. You can use them in Webpages, Github, ...
- Search for a keyword in all your conversation (questions and answers)
- Manage all your history and upkeep your database by deleting the unwanted conversations
- All your actions are audited and sent to the 'Security' database
- All your conversation are saved in the folder '/Project/Database'

## 🚀 How to use

### Download the latest version

> #### Standalone

- [Releases](https://github.com/DavidoAprendiz/CMD-Manager/releases)

---

### 📥 Or if you want to change/edit the application

> #### Install pre-requirements

Rust

- Install the latest [Rust](https://www.rust-lang.org/learn/get-started)

Ollama

- Install Llama3 in [Ollama](https://ollama.com/)
  (needed for "Brain module")

> #### Clone (or Fork directly)

- Open a terminal and clone the repository:
  - `git clone https://github.com/DavidoAprendiz/CMD-Manager.git`
- Enter the directory:
  - `cd CMD-Manager`
- Run Cargo:
  - `cargo run`

---

### ⛁ Manage the database via Graphical Interface (GUI)

> #### To view all your database information

- Download and install [SQLite Browser](https://sqlitebrowser.org/)
- Open database in the default path:
  - `/Project/database.db`

## ✅Roadmap

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
- [X] **Brain Manager**
  - [X] Brainstorm with your personal AI.
  - [X] You can save the answers directly in a Markdown file.
  - [X] Search 'keywords' in 'Brain' database.
  - [X] View 'Brain' history database.
  - [X] Manage 'Brain' history database.
  - [X] View 'Security' history database.
  - [X] Security Audit. Everything is logged in Security database.

  **(in-progress)**

- [ ] Database integration with all missing modules:
  - [X] Todo manager
  - [ ] File manager
  - [ ] Web manager
- [ ] Database security:
  - [X] Add Security Manager
  - [ ] Add password protection.
  - [ ] Evaluate gpg symmetric encryption...
- [ ] Verify (and implement) new llama 3.1 version.
