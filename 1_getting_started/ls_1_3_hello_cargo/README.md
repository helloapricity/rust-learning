# **HELLO, CARGO!**

Cargo is Rust's build system and package manager. It builds code,

downloads and builds dependencies. Most Rust projects use Cargo.

Installing Rust officially includes Cago; check with `cargo --version`.

## **1. CREATING A PROJECT WITH CARGO**

`cargo new` creates a standard project: Cargo.toml + src/main.rs + a Git repository.

Source code goes in src, configuration is in Cargo.toml. Add dependence in[dependencies].

For an existing project -> use `cargo init` to convert it to Cargo.

## **2. BUILDING AND RUNNING A CARGO PROJECT**

### **2.1. Creating a new project**
- Command: `cargo new <project_name>`
- Cargo generates a basic project directory structure.

### **2.2. Building a project**
- Command: `cargo build`
- Default is a **debug build** -> creates an executable in `target/debug/`.
- On Windows: `target\debug\<project_name>.exe`
- Cargo also creates a `Cargo.lock` file to track dependency versions.
- `cargo build` only builds, it does not run the program.

### **2.3. Running a project**
- Directly using the path in Windows:
```rust
.\target\debug\<project_name>.exe
```

- Command to build and run in one step:
```rust
cargo run
```
- If the code hasn't changed, Cargo just runs without rebuilding.
- If the code has changed, Cargo rebuilds before running.

### **2.4. Checking code without creating a binary**
- Command:
```rust
cargo check
```
- Only checks whether the code compiles, does not create an executable.
- Faster than `cargo build` -> useful for continuously checking code during development.

### **2.5. Other advantages**
- Build files are stored in `target/debug/` instead of the project directory.
- Cargo commands are the same across all operating systems -> convenient for cross-platform development.

### **2.6. Summary of important commands**
| Command | Function |
|----------|----------|
| `cargo new <project>` | Create a new project |
| `cargo build` | Build the project (debug, create binary) |
| `cargo run` | Build + run the project in one step |
| `cargo check` | Check if the code compiles, without creating a binary |