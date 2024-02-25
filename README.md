# Univ-Multi-Arb
This code actually still works! Just git clone the repository and use `cargo run -r`. If you want to check taxes or ensure the pairs are valid, just run `cargo run -r load`, This will make a fresh db.jso>
It is a Multi Hop arbitrage bot with a maximum length of 5, saves and precalculate paths before searching, Uses a Quadratic Gradient ascent search to find optimal trade.

For Configuration,  fill the variable in `env.toml`
`private_key = "0x"

 mnemonic = "X X"

 https = "https://virginia.rpc.blxrbdn.com"

 wss = "wss://.."`

### Little Alpha
Using multiple wallets help with nonce and bundle conflicts, it's much efficient to make and track changes

### Use case
For learning and reusing components such as a uni v2 token tax checker, a generalized framework for arbitrage, etc...

### This is a fork of https://github.com/duoxehyon/univ2-tri-arb.git, I added V3 Dexes, An OffChain Simulator to prevent malicious smart contracts and some other Improvements


