# Rust rewrite of pygame tutorial by Tech with Tim
[PygameForBeginners GitHub](https://github.com/techwithtim/PygameForBeginners)

[Tech with Time YouTube video](https://www.youtube.com/watch?v=jO6qQDNa2UY&t=99s)

## To run as an app locally
`cargo run --release`

## To make a WASM (web assembly) build
### Add target to rustup
Only done once, if your rust installation doesn't already have it.
```
rustup target install wasm32-unknown-unknown
```

### run on local webserver using the wasm-server-runner
`cargo run --target wasm32-unknown-unknown`

### build for deployment
from root directory of project

#### a) Using default release build
```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./webapp/ --target web --no-typescript ./target/wasm32-unknown-unknown/release/ships.wasm
```
#### b) Using optimizations from the Cargo.toml [profile.wasm-release]
```
cargo build --profile wasm-release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./webapp/ --target web --no-typescript ./target/wasm32-unknown-unknown/wasm-release/ships.wasm
```

wasm-bindgen will place output files in `./webapp/`
for local serving an assets symlink to the assets folder in webapp dir will work

in webapp directory:
`ln -s ../assets/ assets`

for deployment copy the assets into webapp/assets on the server

### serve
If simple-http-server is not installed:
`cargo install simple-http-server`
Then in webapp directory:
`basic-http-server .`

### More information
[Bevy](https://github.com/bevyengine/bevy/tree/main/examples#wasm)

[Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/platforms/wasm.html)

## Assets
[Sounds](https://github.com/techwithtim/PygameForBeginners)

[Lasers and ships](https://opengameart.org/content/space-shooter-redux)

[Font](https://developers.google.com/fonts)
