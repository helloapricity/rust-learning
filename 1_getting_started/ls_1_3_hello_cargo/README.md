HELLO, CARGO!
Cargo is Rust's build system and package manager. It builds code,

downloads and builds dependencies. Most Rust projects use Cargo.

Installing Rust officially includes Cago; check with `cargo --version`.

**1. CREATING A PROJECT WITH CARGO**

`cargo new` creates a standard project: Cargo.toml + src/main.rs + a Git repository.

Source code goes in src, configuration is in Cargo.toml. Add dependence in[dependencies].

For an existing project -> use `cargo init` to convert it to Cargo.

**2. BUILDING AND RUNNING A CARGO PROJECT**

**2.1. Creating a new project**
- Command: `cargo new <project_name>`
- Cargo generates a basic project directory structure.

**2.2. Building a project**
- Command: `cargo build`
- Default is a **debug build** -> creates an executable in `target/debug/`.
- On Windows: `target\debug\<project_name>.exe`
- Cargo also creates a `Cargo.lock` file to track dependency versions.
- `cargo build` only builds, it does not run the program.

**2.3. Running a project**
- Directly using the path in Windows:
```rust
.\target\debug\<project_name>.exe
```

- 