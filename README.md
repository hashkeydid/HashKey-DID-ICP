# HashKey DID

Canister Link: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=7eo4t-qyaaa-aaaal-acsla-cai

## Introduction
HashKey DID is a multi-chain decentralised identity data aggregator powered by smart contract, NFT, and privacy protection decentralized protocol to provide identity services to Web3 users. As an essential piece of Web3 infrastructure in the HashKey ecosystem, the fundamental goal of HashKey DID is to build a community-governed HashKey DID DAO together with users.

Highlight some features:
- Credential: Open, flexible, and easy-to-use credential NFT
- Islands: A DAO governance tool based on Credential
- Space: Customized dashboard for users in Web3
- Credit Score: The credit rating system in Web3

## Generate candid for rust canister
```
sudo cargo r > src/did.did
```
## generate wasm file
```
sudo cargo build --release --target wasm32-unknown-unknown
sudo ic-cdk-optimizer target/wasm32-unknown-unknown/release/caller.wasm -o target/wasm32-unknown-unknown/release/opt.wasm
```

## Roadmap

- [x] Product Development：Supporting Dfinity Ecosystem
- [ ] Marketing & Operations：Building Dfinity Community Together & Enhancing User’s Web3 Experience