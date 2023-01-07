### Running Rust programs and the Cargo package manager
- `cargo new` creates a skeleton Rust project in a new directory (`cargo init` uses the current directory)
- `cargo build` downloads dependencies and compiles the code.
- `cargo run` executes `cargo build` and then also runs the resulting executable file.
- Can have multiple executable files (in addition to main) in one project by putting them in the `src/bin` folder. Use `cargo run --bin file` to run that file.
- `cargo doc` builds HTML documentation for every dependency in the current project.
- Can find dependencies in `Cargo.toml`.

### Rust principles
- First priority is safety.
- Data within Rust is immutable by default.
- Compile-time checks are strongly preferred. Safety should be a "zero-cost abstraction".

### Rust's big features
- Performance
- Concurrency
- Memory efficiency


