# Rust Python Binding with Maturin

---

## Getting started

- Install rust
  - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Install all the requirements with pip (python: 3.9.12)
  - `pip install -r requirements.txt`
- Build the crate
  - `cargo build --release`
  - `maturin build --release`
- Install the exported wheel
  - `pip install --force-reinstall target/wheels/rustify*.whl`
- Run smoke tests
  - `pytest -vs`
