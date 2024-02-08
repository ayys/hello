# hello
Hello World in wasm

# Building

```bash
# install wasmer
curl https://get.wasmer.io -sSfL | sh

# install cargo-wasix
cargo install cargo-wasix

# build the release version
cargo wasix build --release

# run it with wasmer
wasmer run .
```
