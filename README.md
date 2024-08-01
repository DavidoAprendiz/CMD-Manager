<h1 align="center">CMD-Manager</h1>

<div align="center">
<img alt="Rust CI" src="https://github.com/DavidoAprendiz/Rust-in-Progress/actions/workflows/rust.yml/badge.svg">
<br>
<img alt="CMD-Manager" src="https://img.shields.io/badge/CMD_Manager-v0.5.0-blue">
<br>
<img alt="Made with Rust, SQLite and Ollama" src="https://skillicons.dev/icons?i=rust,sqlite">
<img alt="ollama" height="50px" src="https://github.com/ollama/ollama/assets/3325447/0d0b44e2-8f4a-4e99-9b52-a5c1c741c8f7">

<a>Made with Rust, SQLite and Ollama</a>

<div align="center">
• <a href="#-description">⚡ Description</a> •
  <a href="#-how-to-use">🚀 Install/Download</a> •
  <a href="#-or-if-you-want-to-changeedit-the-application">📥 Clone</a> •
  <a href="#-manage-the-database-via-graphical-interface-gui">⛁ Manage Database</a> •
  <a href="https://github.com/DavidoAprendiz/CMD-Manager/blob/main/CONTRIBUTING.md">💻 Contributing</a> •
  <a href="https://github.com/DavidoAprendiz/CMD-Manager/blob/main/LICENSE">📃 License</a> •
  <a href="#-Roadmap">✅ Roadmap</a> •
</div>

---

![cmd-manager_gif](https://github.com/user-attachments/assets/6af21706-4419-434e-82ac-9d09e1d4e0df)

</div>

## ⚡ Description

A **multi-application** command-line powered by AI, focused on privacy (because privacy matters!) 🔐🔐🔐

From a simple to-do/note taking app to your own personal assistant!

You can encrypt your database using GPG (AES256 symmetric keys) and then save it in any cloud provider with a peaceful mind. 😀

All actions from the "managers" are saved/stored in the respective table.
The security logs are saved/stored in the 'Security' table.

### **To-do Manager**

- You can add, view and delete your to-dos/notes in an offline environment.

### **File Manager**

- Search Mode - Search any character/word/phrase in text documents locally (similar to fzf tool).
- Compare Mode - Compare any document and see all the differences using Myers Algo (similar to Git/Github diff tool).

### **Web Manager**

- Download Mode - Download full HTML content.
- Get API Mode - Get the current price of Ergo and Cardano from Coingecko API.

### **Brain Manager**

- Talk with **your own** AI personal assistant and save all conversation histories in **your own** database **and/or** to markdown files. You can use them in Webpages, Github, ...
- Search for a keyword in all your conversation (questions and answers)
- Manage all your history and upkeep your database by deleting unwanted conversations

## 🚀 How to use

### Download the latest version

#### Standalone

> - [Releases](https://github.com/DavidoAprendiz/CMD-Manager/releases)

---

### 📥 Or if you want to change/edit the application

#### Install pre-requirements

- Rust
>
> - Install the latest [Rust](https://www.rust-lang.org/learn/get-started)
>
- Ollama
>
> - Install Llama3.1 from [Ollama](https://ollama.com/)
>   - (needed for "Brain module")
>
- GPG
>
> - Check if GPG is installed:
>   - `gpg --version`
>
> - Install [GPG](https://www.gnupg.org/download/)
>   - (mostly windows users, used to encrypt database)
>

#### Clone (or Fork directly)

- Open a terminal and clone the repository:
>   - `git clone https://github.com/DavidoAprendiz/CMD-Manager.git`
- Enter the directory:
>   - `cd CMD-Manager`
- Run Cargo:
>   - `cargo run`

---

### ⛁ Manage the database via Graphical Interface (GUI)

#### To view all your database information

- Download and install [SQLite Browser](https://sqlitebrowser.org/)
> - Open database in the default path:
>   - `/Project/database.db`

#### Tables used

> <br>
>
> | API | ColumnType |
> | :-: | :-: |
> | timestamp | text |
> | name | blob |
> | price | blob |
>
> | BRAIN | ColumnType |
> | :-: | :-: |
> | timestamp | text |
> | questions | blob |
> | answers | blob |
>
> | DOWNLOAD | ColumnType |
> | :-: | :-: |
> | timestamp | text |
> | download_url | blob |
> | download_body | blob |
>
> | FILE_COMPARE | ColumnType |
> | :-: | :-: |
> | timestamp | text |
> | file_name | text |
> | file_name_2 | text |
> | file_content | blob |
>
> | FILE_SEARCH | ColumnType |
> | :-: | :-: |
> | timestamp | text |
> | file_name | blob |
> | file_content | blob |
>
> | SECURITY | ColumnType |
> | :-: | :-: |
> | timestamp | text |
> | security_tag | text |
>
> | TODO | ColumnType |
> | :-: | :-: |
> | timestamp | text |
> | title | blob |
> | task | blob |
> <br>

## ✅ Roadmap

- [X] **Todo Manager**
  - [X] Add new tasks.
  - [X] View saved tasks.
  - [X] Delete saved tasks.
- [X] **File Manager**
  - [X] Search 'keywords' in any file.
  - [X] Compare any two files and check differences between them.
- [X] **Web Manager**
  - [X] Download any webpage content from the web.
  - [X] Get price data from Coingecko APIs.
- [X] Improve TUI (added colors)
- [X] Add SQLite database.
- [X] Basic unit tests (for best practices).
- [X] **Brain Manager**
  - [X] Brainstorm with your personal AI.
  - [X] You can save the answers directly in a Markdown file.
  - [X] Search 'keywords' in 'Brain' database.
  - [X] View 'Brain' database.
  - [X] Manage 'Brain' database.
- [X] View 'Security' database.
- [X] Security Audit logs.
- [X] Database integration with all missing modules:
  - [X] Todo manager.
  - [X] File manager.
  - [X] Web manager.
- [X] Database security:
  - [X] Add Security Manager.
  - [X] Encrypt DB with GPG (symmetric keys).
- [X] Update to new Ollama model (llama 3.1).
