---
sidebar_position: 1
---

# The Ideal Bitcoin L2

Success as measured by user adoption and builder adoption alike for Bitcoin sidechains, L2s, drivechains, and other constructions is lacking compared to alterantive L1s (such as Ethereum, Avalanche, ...) and L2s on Ethereum (OP Stack, Arbitrum, Starkware, ...). We see two main technical reasons as a root cause:

1. **Rust smart contracts**: Provide developers with the ability to use their existing SDKs and libraries based on Rust.Extending Bitcoin with new apps (e.g., https://github.com/ordinals/ord and https://github.com/rust-nostr/nostr), libraries (e.g., https://github.com/rust-bitcoin), and SDKs (e.g., https://github.com/lightningdevkit and https://bitcoindevkit.org/) are primarily based on Rust. Existing L2 or sidechains fail to deliver on that as they are EVM-based or use other execution layers like Stacks and Liquid.
2. **EVM-compatible interfaces**: Provide projects with access to the largest user base, asset selection, and UX tooling ecosystem possible. Since building an L2 directly on Bitcoin with Bitcoin’s consensus security is impossible without changes to Bitcoin core (more years than months away), the next best ecosystem is Ethereum. One might even argue that Ethereum has an advantage over Bitcoin as an L2 host due to its clear commitment to improving L2 technology and vast builder ecosystem. The EVM is the de-facto standard of blockchain builders as shown by the many alternative EVM offerings ranging from Polygon, to OP Stack, Arbitrum, Binance Chain, and so on. Moreover, L2s are focussing on cross-rollup interoperability (e.g., OP Superchain) offering benefits for staying within the same stack. By being EVM-compatible projects can leverage improvements made for other EVM-projects.

The two goals seem at odds: Rust Bitcoin libraries and SDKs are incompatible with the EVM without major conversion work. So do we need to choose one over the other? No.

![why-not-both](https://i.imgflip.com/30xotn.jpg)
