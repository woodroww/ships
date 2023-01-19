# Rust rewrite of pygame tutorial by Tech with Tim
[PygameForBeginners GitHub](https://github.com/techwithtim/PygameForBeginners)
[YouTube vieo](https://www.youtube.com/watch?v=jO6qQDNa2UY&t=99s)

## To run as an app locally
`cargo run --release`

## To make a WASM (web assembly) build
### More information:
[Bevy](https://github.com/bevyengine/bevy/tree/main/examples#wasm)
[Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/platforms/wasm.html)

### add target to rustup
(only done once, if your rust installation doesn't already have it)
`rustup target install wasm32-unknown-unknown`

### wasm-server-runner
Setup wasm-server-runner to host the game: 
#### Add the following two lines to `.cargo/config.toml`
```
[target.wasm32-unknown-unknown]
runner = "wasm-server-runner"
```
#### or use an environment variable:
`export CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-server-runner`

### run on local webserver using the wasm-server-runner
`cargo run --target wasm32-unknown-unknown`

### build for deployment
from root directory of project

#### Using default release build
```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./webapp/ --target web ./target/wasm32-unknown-unknown/release/ships.wasm
```
#### Using optimizations from the Cargo.toml [profile.wasm-release]
```
cargo build --profile wasm-release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./webapp/ --target web ./target/wasm32-unknown-unknown/wasm-release/ships.wasm
```

Will place output files in `./webapp/`
- for local serving an assets symlink in webapp dir will work
in webapp directory
`ln -s ../assets/ assets`
- for deployment copy the assets into webapp/assets and then to your server

### serve
in webapp directory
`basic-http-server .`
or
`python -m http.server --directory .`



