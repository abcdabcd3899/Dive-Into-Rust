## Workspace Management

### Create a new project
```shell
mkdir workspace_demo
cd workspace_demo
touch Cargo.toml # write the [workspace] and members information
cargo new --lib add_one
cargo new adder
cargo build
```

We find the Cargo.lock file in the only root directory.

```shell
cargo build -p add_one
cargo build -p adder
cargo run -p adder # only binary crate files
```

**This is a minimum workspace project.**