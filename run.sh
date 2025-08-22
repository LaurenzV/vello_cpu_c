RUSTFLAGS="-C target-feature=+avx2" cargo build --release
cbindgen --config cbindgen.toml --output cpu_sparse.h