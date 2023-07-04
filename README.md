# Fly.io distributed challenge
This is implementation of the [Fly.io](https://fly.io/dist-sys) distributed challenge in rust.

There are multiple implementations of the "Node".
- [V1](https://github.com/wexder/gossip-rs/tree/v1) naive implementations that fails with challenge #3c
- [V2](https://github.com/wexder/gossip-rs/tree/v2) code was refactor and basic use of threads, but still not optimal implementation of #3c
- [V3](https://github.com/wexder/gossip-rs/tree/v3) can sometimes pass #3d, but with too many messages. Need better sync protocol.
- [V4](https://github.com/wexder/gossip-rs/tree/v4) Finnally passing all of the #3 challenges. But this implementation would never work in real word.
