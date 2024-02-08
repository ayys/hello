# hello
Hello World in wasm

# running
```bash
wasmer run hello-world

# or run your local version
# cargo wasix build --release
wasmer run .
```

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
