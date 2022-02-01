# Workspace Setup

This document is to help you to set up your course workspace.

## Operating System

Course homeworks are guaranteed to compile and work in Linux and macOS. Please note that any other operating system **is not supported** through it may work using Docker, WLS, or other virtualization tools.

## Setup process

### Step 1 - Installing Rust and C linker

Install `rustup` either [by the official command line](https://www.rust-lang.org/tools/install) or using your package manager such as `apt`, `pacman` or `brew`.

On Linux, you probably have C language linker, but make sure you already have it by installing `build-essential` using your package manager.

On MacOS, users have to install XCode tools to get a C language linker.

```shell
xcode-select --install
```

Run this command to get stable Rust compiler:

```shell
rustup update stable
```

### Step 2 - VS Code and plugins

The only supported editor in the course is [Visual Studio Code](https://code.visualstudio.com). Install it. Then, install the following plugins:

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) - language server, your best friend.
- [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml) - syntax highlight for `.toml` files.
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) - optional, needed for debugging.

_IDE such as CLion or editors with plugins such ad Vim will work perfectly because of Rust's nature, but lecturer don't use them and won't support officially._

### Step 3 - Cloning repository

Clone the repository:

```shell
git clone https://gitlab.com/alex.stanovoy/mipt-rust.git
cd mipt-rust
```

### Step 4 - First solution

Read the document about [solving and submitting problems](solving.md). Solve the [add](problems/tutorial/add) problem.

### Step 5 - CI

Coming soon :)
