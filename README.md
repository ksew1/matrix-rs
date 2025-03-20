# matrix-rs

## Prerequisites

- [Rust](https://www.rust-lang.org/)
- [uv](https://github.com/astral-sh/uv)

## Build bindings

```bash
uvx  maturin develop
```

### Running checks

#### Rust


```bash
./scripts/rust_check.sh
```

This will run `cargo fmt`, `cargo lint` and `cargo test`.

#### Python

```bash
./scripts/python_check.sh
```

This will run `ruff check .` and `ruff format --check .`