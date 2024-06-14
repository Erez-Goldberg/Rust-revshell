# Rust Reverse Shell
## Description
A simple demonstration of a TCP reverse shell implemented in Rust.

## Features
- Written in Rust
- Basic reverse shell functionality
- Cross-platform support

## Prerequisites
### Linux

1. Download and run the rustup script to install Rust  
```
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Configure  current shell session to use the installed Rust environment
```
  source $HOME/.cargo/env
```
3. Verify the Installation
```
  rustc --version
  cargo --version
```
Create a New Rust Project
```
  cargo new shell_rust
  cd shell_rust
```

## Compilation Instructions
1. **Clone the Repository** Clone this repository to your local machine.
    ```sh
    git clone https://github.com/Erez-Goldberg/Rust-revshell.git
    cd Rust-revshell
    ```
3. **Build the Project**: Use Cargo to build the project.
    ```sh
    cargo build --release
    ```
4. **Run the Executable**

### Windows

1. **Install Rust**: If you don't have Rust installed, download and install it from [rust-lang.org](https://www.rust-lang.org/). Ensure you install the `x86_64-pc-windows-gnu` toolchain:
    ```sh
    rustup toolchain install stable-x86_64-pc-windows-gnu
    rustup target add x86_64-pc-windows-gnu
    ```

2. Clone this repository to your local machine.
    ```sh
    git clone https://github.com/Erez-Goldberg/Rust-revshell.git
    cd Rust-revshell
    ```

3. **Install MinGW**: Download and install [MinGW-w64](http://mingw-w64.org/doku.php/download) to get the necessary GCC toolchain for Windows.

4. **Build the Project**: Use Cargo to build the project for Windows.
    ```sh
    cargo build --release --target x86_64-pc-windows-gnu
    ```

5. **Run the Executable**

## Usage
Replace the `target_ip` and `target_port` variables in the `main.rs` file with the IP address and port of the attacker's machine.

```rust
let target_ip = "192.168.152.235";
let target_port = "4444";
```

### Typical Rust Project Structure
```rust
my_project/
├── Cargo.toml 📄
├── Cargo.lock 🔒
└── src/
    ├── main.rs 📂
    └── lib.rs 📂
```
### File Descriptions

- 📄 **Cargo.toml**: Configuration file for the Rust project.
- 🔒 **Cargo.lock**: Contains exact versions of dependencies.
- 📂 **main.rs**: Entry point for the Rust application.
- 📂 **lib.rs**: Library file for the Rust project.
