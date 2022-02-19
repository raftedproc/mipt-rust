# Workspace Setup

This document is to help you to set up your course workspace.

## Operating System

Course homeworks are guaranteed to compile and work in Linux and macOS. Please note that any other operating system **is not supported** through it may work using Docker, WLS, or other virtualization tools.

## Setup process

### Step 1 - Installing Rust and C linker

Install `rustup` either [by the official command line](https://www.rust-lang.org/tools/install) or using your package manager such as `apt`, `pacman` or `brew`.

On Linux, you probably have the C language linker, but make sure you already have it by installing `build-essential` using your package manager.

On MacOS, users have to install XCode tools to get a C language linker.

```shell
xcode-select --install
```

Run this command to get the stable Rust compiler:

```shell
rustup update stable
```

### Step 2 - VS Code and plugins

The only supported editor in the course is [Visual Studio Code](https://code.visualstudio.com). Install it. Then, install the following plugins:

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) - language server, your best friend.
- [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml) - syntax highlight for `.toml` files.
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) - optional, needed for debugging.

_IDE such as CLion or editors with plugins such ad Vim will work perfectly because of Rust's nature, but the lecturer doesn't use them and won't support them officially._

### Step 3 - Cloning repository

Clone the repository:

```shell
git clone https://gitlab.com/alex.stanovoy/mipt-rust.git
cd mipt-rust
```

### Step 4 - Rover tool

We have a course assistant named `rover`. It will automatize the part of your solving routine. Go to directory with it, build and install:

```shell
cargo install --path tools/rover
```

From this moment, you can call it from any place you want! Just try to type:

```shell
rover --help
```

To uninstall it later, run this line from anyplace:

```shell
cargo uninstall rover
```

[Here](../tools/rover/README.md) you can read more information (mostly useless for a student) about `rover`.

### Step 5 - First solution

Read the document about [solving and submitting problems](solving.md). Solve the [add](problems/tutorial/add) problem.

### Step 6 - Student CI

1. Register in [testing system](https://mipt-rust.manytask.org). You can find the secret code in the course channel.
2. Generate an SSH key if you don't have one:

    ```shell
    ssh-keygen -N "" -f ~/.ssh/id_rsa
    ```

3. Copy `id_rsa.pub` (`cat ~/.ssh/id_rsa.pub`) to [`gitlab.manytask.org/profile/keys`](https://gitlab.manytask.org/profile/keys).
4. Check if the SSH key is working by running the command `ssh git@gitlab.manytask.org`:

    ```shell
    $ ssh git@gitlab.manytask.org
    PTY allocation request failed on channel 0
    Welcome to GitLab, Alexander Stanovoy!
    Connection to gitlab.manytask.org closed.
    ```

5. Clone your repository

TODO: Just a bit later :)
