# cherubgyre
cherubgyre is an anonymous community defense social network

https://cherubgyre.com is under construction, but it's got some links.
https://api.cherubgyre.com has api docs.

## toolchain setup
1. Install rust, following instructions at https://rustup.rs
2. Clone this repo
3. Install RustRover from JetBrains (register for an account, free for non-commercial use)
4. Install lld linker for faster compile times
    ```
    brew install llvm
    ```
    or
    ```
    apt install llvm lld clang
    ```
6. Install cargo-watch to trigger commands when source code changes. Chain some commands so cargo watch runs check, (if successful) then test, (if successful) then run:
    ```
    cargo install cargo-watch
    cargo watch -x check -x test -x run
    ```
7. Install cargo-llvm-cov to measure code coverage, and run cargo llvm-cov to compute code coverage for the application.
   ```
   rustup component add llvm-tools-preview
   cargo install cargo-llvm-cov
   cargo llvm-cov
   ```
8. Make sure the linter "clippy" is installed. Run it to fail the pipeline if there are warnings.
   ```
   rustup component add clippy
   cargo clippy -- -D warnings
   ```
9. Add rustfmt for automatic code formatting. Run it using cargo fmt (or `cargo fmt -- ---check` if you'd prefer a formatting step for caution. I don't.)
   ```
   rustup component add rustfmt
   cargo fmt
   ```
10. Add cargo-audit for security audits. Run it to scan you dependency tree.
   ```
   cargo install cargo-audit
   cargo audit
   ```
### notes
- Check out `.github/workflows/general.yaml` in this repository: it will run some of the above fmt and clippy checks on every push to main.
- Check out `.github/workflows/audit.yaml` in this repository: it will run audits on every push to main.
- Tests will be in `tests/` here because it is preferable to externalize tests from the source for the purposes of visibility and security. We don't want to give tests any privileged access to the code.
