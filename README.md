e2rustcli
---------

A template for CLI tools written in Rust.

### Setup
Install useful cargo extensions (optional):
```
cargo install cargo-edit cargo-watch
```
Edit `Cargo.toml` and fill in your project's details.

### Build
```
cargo build           // build once
cargo watch -x build  // re-build on src file changes
cargo build --release // build for public release
```
Release binary will be located at `e2rustcli/target/release/e2rustcli` (replace `e2rustcli` with your project name).

### Install
```
cargo install --path .  // install binary locally to ~/.cargo/bin
```
Now running `e2rustcli` directly will work (ensure `~/.cargo/bin` is on your `$PATH`).

Alternately, install via symlink to your `bin` of choice (on your `$PATH`):
```
ln -s "$(pwd)/target/debug/e2rustcli" ~/bin/e2rustcli
```

### Run
Four ways to run your CLI tool:
```
cargo run                 // run via cargo
cargo run -- --help       // run with arguments
./target/debug/e2rustcli  // execute binary directly
e2rustcli                 // execute installed binary
```

### Lint
```
cargo clippy  // lints & builds per https://github.com/rust-lang/rust-clippy
```

### Distribute

Portable binary is located at `e2rustcli/target/release/e2rustcli` after successful `cargo build --release` (replace `e2rustcli` with your project name).

```
rustcli $ ./target/release/e2rustcli --help

e2rustcli 0.0.1
evnp <ep@e2.gg>
A template for CLI tools written in Rust.

USAGE:
    e2rustcli [FLAGS] [OPTIONS] [TARGET]

FLAGS:
    -d, --delete     delete something
    -h, --help       Prints help information
    -q, --quiet      suppress all info + debug output
    -V, --version    Prints version information
    -v, --verbose    log info + debug output to terminal

OPTIONS:
    -e, --edit <EDIT>        edit something
    -r, --rename <RENAME>    rename something

ARGS:
    <TARGET>    something to execute or operate on
```


