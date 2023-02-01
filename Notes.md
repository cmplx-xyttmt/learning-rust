### Running Rust programs and the Cargo package manager
- `cargo new` creates a skeleton Rust project in a new directory (`cargo init` uses the current directory)
- `cargo build` downloads dependencies and compiles the code.
- `cargo run` executes `cargo build` and then also runs the resulting executable file.
- Can have multiple executable files (in addition to main) in one project by putting them in the `src/bin` folder. Use `cargo run --bin file` to run that file.
- `cargo doc` builds HTML documentation for every dependency in the current project.
- Can find dependencies in `Cargo.toml`.
- You can also compile and run single Rust files as follows:
```
rustc ok.rs
./ok
```
- `cargo` "drives" `rustc`. To see what `cargo` is doing under the hood, you can add a verbose flag: `cargo run -v`.

### Rust principles
- First priority is safety.
- Data within Rust is immutable by default.
- Compile-time checks are strongly preferred. Safety should be a "zero-cost abstraction".

### Rust's big features
- Performance
- Concurrency
- Memory efficiency

### Rust Numbers
- Rust includes a large number of nummeric types. The size is declared in bytes (affects size number can represent and whether or not it can represent negative numbers)
- Conversion between types are always explicit.
- Rust's numbers can have methods e.g `24.5_f32.round()`.

Rust types for representing scalar (single numbers)

|                    |                                                                                                                       |
|--------------------|-----------------------------------------------------------------------------------------------------------------------|
| `i8, i16, i32,i64` | Signed integers ranging from 8 bit to 64 bit                                                                          |
| `u8,u16,u32,u64`   | Unsigned integers ranging from 8 bit to 64 bit                                                                        |
| `f32, f64`         | Floating-point numbers in 32-bit and 64-bit variants                                                                  |
| `isize, usize`     | Integers that assume the CPU's "native" width. For example, in 64-bit CPUs, `usize` and `isize` will be 64-bits wide. |

#### Comparing numbers
- Numeric types in Rust support a large suite of comparisons provided by a Rust feature called `traits`.
- Rust's type safety requirements prevent comparisons between types. (Need to use an `as` operator to cast one of the operands to the other's type).


#### Adding third-party dependencies
- Can add manually to `Cargo.toml` or use `cargo add`
- To use `cargo add`, install `cargo-edit`: `cargo install cargo-edit`
- e.g to add the `num` dependency: `cargo add num`.

#### Traits
A `trait` is a Rust language feature that's analogous to an interface, protocol or contract.

A `trait` is similar to an `abstract base class` in OOP languages or `type classes` in Haskell.

Traits enable types to advertise that they are using common behavior.
