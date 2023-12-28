cargo build --profile wasm-release --target wasm32-unknown-unknown && \
wasm-bindgen --out-dir ./webapp/ --target web --no-typescript ./target/wasm32-unknown-unknown/wasm-release/ships.wasm
cd webapp && \
wasm-opt -Oz -o ships_bg.wasm ships_bg.wasm
