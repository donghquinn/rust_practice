# Rust And Cargo
- Cargo is Package Manager of Rust like yarn or npm from javascript
- All the Dependencies and information will be exist on Cargo.toml like package.json of javascript
- Rust file is end with .rs

## Run
- Can Run rust code by commanding below

``` cargo run {filePath/fileName} ```

## Build
- Build Rust code by commanding below. It will compile at target/debug.

``` cargo build {filePath/fileName} ```

- Build Rust code with Optimize. It will puts resulting binary in target/release.
- It would take longer for optimizing than just debugging. But it will run faster.

``` cargo build {filePath/fileName} --release ```

