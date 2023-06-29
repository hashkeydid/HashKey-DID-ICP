## Generate candid for rust canister
```
sudo cargo r > src/did.did
```
## generate wasm file
```
sudo cargo build --release --target wasm32-unknown-unknown
sudo ic-cdk-optimizer target/wasm32-unknown-unknown/release/caller.wasm -o target/wasm32-unknown-unknown/release/opt.wasm
```