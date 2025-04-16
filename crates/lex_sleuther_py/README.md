# lex_sleuther python bindings

Uses [PyO3](https://pyo3.rs/v0.24.1/) for FFI.

I prefer using [`uv`](https://github.com/astral-sh/uv) to work with this.

```bash
# install the library into a local venv
uv sync
# build a source distribution and wheel
uv build
```

In CICD, we build with [maturin Docker images based on manylinux2014](https://github.com/pyo3/maturin/pkgs/container/maturin).