---
sidebar_position: 2
hide_table_of_contents: true
---

# Research at BOB

Technological advantage stems from persistent and rigorous research.
Founded by ex-Imperial College London PhDs, we are dedicated to contribute to cutting-edge research in the areas of blockchain interoperability, security, and crypto-economics.

Our team members have co-authored over 30 scientific publications in collaboration with leading universities and research centres around the globe, resulting in more than 1,000 citations.

## Latest Research Papers

### [BOB: The Hybrid L2](https://docs.gobob.xyz/whitepaper.pdf)

<details>
  <summary>Read abstract</summary>

BOB is a new type of Bitcoin-secured blockchain: a Hybrid L2. Hybrid L2s inherit security from Bitcoin, as the most secure and decentralized network. Bitcoin security is then used to create trustminimized bridges to Bitcoin, Ethereum, and other L1s. As a result, the Hybrid L2 does not rely on third-party bridges for interoperability and solves the problem of fragmented BTC multi-chain liquidity.

</details>

- **Zamyatin A, Harz D**
- Pre-print, 2024

### [BitVM2: Bridging Bitcoin to Second Layers](https://bitvm.org/bitvm_bridge.pdf)

<details>
  <summary>Read abstract</summary>

BitVM2 is a novel paradigm that enables arbitrary program execution in Bitcoin, thereby combining Turing-complete expressiveness with the security of Bitcoin consensus. At its core, BitVM2 leverages optimistic computation, assuming operators are honest unless proven otherwise by challengers through fraud proofs, and SNARK proof verification scripts, which are split into sub-programs that are executed within Bitcoin transactions. As a result, BitVM2 ensures program correctness with just three on-chain transactions. BitVM2 significantly improves over prior BitVM designs by enabling, for the first time, permissionless challenging and by reducing the complexity and number of on-chain transactions required to resolve disputes. Our construction requires no consensus changes to Bitcoin. BitVM2 enables the design of an entirely new class of applications in Bitcoin. We showcase that by presenting BitVM Bridge, a protocol that enhances prior Bitcoin bridges by reducing trust assumptions for the safety of deposits from an honest majority (t-of-n) to existential honesty (1-of-n) during setup. To guarantee liveness, we only require one active rational operator (while the others can be malicious). Any user can act as challenger, facilitating permissionless verification of the protocol.

</details>

- Linus R, Aumayr L, **Zamyatin A**, Pelosi A, Avarikioti Z, Meffei M
- Pre-print, 2024

### [OptiMine: Optimistically Sequenced Merged Mining for Rollups and Sidechains](https://gobob.xyz/optimine)

<details>
  <summary>Read abstract</summary>

We coin the hybrid consensus technique “Optimistically Sequenced Merged Mining”. The technique enables any sidechain or rollup connected to an L1 chain to receive Proof-of-Work security from Bitcoin Miners. Rollups and sidechains retain fast block production and reduce the trust in centralized Sequencers and block-producing nodes.

</details>

- The BOB Collective
- Pre-print, 2024

## Previous Research Work

Our team has been actively contributing to cutting-edge research in the blockchain space over the past 9 years.

A lot of the concepts developed by our team during their previous research careers find their way into BOB's products. Some of the most recent and relevant works of our team members are outlined below:

### [XCC: Theft-Resilient and Collateral-Optimized Cryptocurrency-Backed Assets](https://docs.interlay.io/_assets/papers/XCC_paper.pdf)

<details>
  <summary>Read abstract</summary>

The need for cross-blockchain interoperability is higher than ever. Today, there exists a plethora of blockchain-based cryptocurrencies, with varying levels of adoption and diverse niche use cases, and yet communication across blockchains is still in its infancy. Despite the vast potential for novel applications in an interoperable ecosystem, cross-chain tools and protocols are few and often limited.

Cross-chain communication requires a trusted third party, as the Fair Exchange problem is reducible to it. However, the decentralised consensus of blockchains can be used as a source of trust, and financial incentives can achieve security. XCLAIM uses these principles to enable collateralised cryptocurrency-backed assets (CbAs) to be created and used. However, full collateralization is inefficient, and to protect against exchange rate fluctuations overcollateralization is necessary. This is a significant barrier to scaling, and as a result, in practice, most systems still employ a centralised architecture.

In this work, we introduce XCC, an extension to the XCLAIM framework which allows for a significant reduction in collateral required. By making use of periodic, timelocked commitments on the backing blockchain, XCC decouples locked collateral from issued CbAs, allowing fractional collateralization without loss of security. We instantiate XCC between Bitcoin and Ethereum to showcase practical feasibility. XCC is compatible with the majority of existing blockchains without modification.

</details>

- Bugnet T, **Zamyatin A**
- Pre-print, 2022
- In collaboration with Imperial College London

### [SoK: Communication Across Distributed Ledgers](https://eprint.iacr.org/2019/1128.pdf)

<details>
  <summary>Read abstract</summary>

Since the inception of Bitcoin, a plethora of distributed ledgers differing in design and purpose has been created. While by design, blockchains provide no means to securely communicate with external systems, numerous attempts towards trustless cross-chain communication have been proposed over the years. Today, cross-chain communication (CCC) plays a fundamental role in cryptocurrency exchanges, scalability efforts via sharding, extension of existing systems through sidechains, and bootstrapping of new blockchains. Unfortunately, existing proposals are designed ad-hoc for specific use-cases, making it hard to gain confidence in their correctness and composability.

We provide the first systematic exposition of cross-chain communication protocols. We formalize the underlying research problem and show that CCC is impossible without a trusted third party, contrary to common beliefs in the blockchain community. With this result in mind, we develop a framework to design new and evaluate existing CCC protocols, focusing on the inherent trust assumptions thereof, and derive a classification covering the field of cross-chain communication to date. We conclude by discussing open challenges for CCC research and the implications of interoperability on the security and privacy of blockchains.

</details>

- **Zamyatin A**, Al-Bassam M, Zindros D, Kokoris-Kogias E, Moreno-Sanchez P, Kiayias A, Knottenbelt WJ
- Financial Cryptography and Data Security 2021
- In collaboration with Imperial College London, UCL, IOHK, EPFL, TU Vienna, University of Athens, University of Edinburgh

### [TxChain: Efficient Cryptocurrency Light Clients via Contingent Transaction Aggregation](https://eprint.iacr.org/2020/580.pdf)

<details>
  <summary>Read abstract</summary>

Cryptocurrency light- or simplified payment verification (SPV) clients allow nodes with limited resources to efficiently verify execution of payments. Instead of downloading the entire blockchain, only block headers and selected transactions are stored. Still, the storage and bandwidth cost, linear in blockchain size, remain non-negligible, especially for smart contracts and mobile devices: as of April 2020, these amount to 50 MB in Bitcoin and 5 GB in Ethereum.

Recently, two improved sublinear light clients were proposed: to validate the blockchain, NIPoPoWs and FlyClient only download a polylogarithmic number of block headers, sampled at random. The actual verification of payments, however, remains costly: for each verified transaction, the corresponding block must too be downloaded. This yields NIPoPoWs and FlyClient only effective under low transaction volumes.

We present TxChain, a novel mechanism to maintain efficiency of light clients even under high transaction volumes. Specifically, we introduce the concept of contingent transaction aggregation, where proving inclusion of a single contingent transaction implicitly proves that n other transactions exist in the blockchain. To verify n payments, TxChain requires only a single transaction in the best (n≤c), and n/c+logc(n) transactions in the worst case (n>c). We deploy TxChain on Bitcoin without consensus changes and implement a soft fork for Ethereum. To demonstrate effectiveness in the cross-chain setting, we implement TxChain as a smart contract on Ethereum to efficiently verify Bitcoin payments.

</details>

- **Zamyatin A**, Avarikioti Z, Perez D, Knottenbelt WJ,
- 4th International Workshop on Cryptocurrencies and Blockchain Technology - CBT 2020
- In collaboration with Imperial College London and ETH Zurich

### [Promise: Leveraging Future Gains for Collateral Reduction](https://eprint.iacr.org/2020/532.pdf)

<details>
  <summary>Read abstract</summary>

Collateral employed in cryptoeconomic protocols protects against the misbehavior of economically rational agents, compensating honest users for damages and punishing misbehaving parties. The introduction of collateral, however, carries three disadvantages: (i) requiring agents to lock up a substantial amount of collateral can be an entry barrier, limiting the set of candidates to wealthy agents; (ii) affected agents incur ongoing opportunity costs as the collateral cannot be utilized elsewhere; and (iii) users wishing to interact with an agent on a frequent basis (e.g., with a service provider to facilitate second-layer payments), have to ensure the correctness of each interaction individually instead of subscribing to a service period in which interactions are secured by the underlying collateral.

We present Promise, a subscription mechanism to decrease the initial capital requirements of economically rational service providers in cryptoeconomic protocols. The mechanism leverages future income (such as service fees) prepaid by users to reduce the collateral actively locked up by service providers, while sustaining secure operation of the protocol. Promise is applicable in the context of multiple service providers competing for users. We provide a model for evaluating its effectiveness and argue its security. Demonstrating Promise's applicability, we discuss how Promise can be integrated into a cross-chain interoperability protocol, XCLAIM, and a second-layer scaling protocol, NOCUST. Last, we present an implementation of the protocol on Ethereum showing that all functions of the protocol can be implemented in constant time complexity and Promise only adds USD 0.05 for a setup per user and service provider and USD 0.01 per service delivery during the subscription period.

</details>

- **Harz D**, Gudgeon L, Khalil R, **Zamyatin A**
  affiliations: Imperial College London
- International Conference on Mathematical Research for Blockchain Economy 2020
- In collaboration with Imperial College London

### [XCLAIM: Trustless, Interoperable Cryptocurrency-Backed Assets](https://eprint.iacr.org/2018/643.pdf)

<details>
  <summary>Read abstract</summary>

Building trustless cross-blockchain trading protocols is challenging. Centralized exchanges thus remain the preferred route to execute transfers across blockchains. However, these services require trust and therefore undermine the very nature of the blockchains on which they operate. To overcome this, several decentralized exchanges have recently emerged which offer support for atomic cross-chain swaps (ACCS). ACCS enable the trustless exchange of cryptocurrencies across blockchains, and are the only known mechanism to do so. However, ACCS suffer significant limitations; they are slow, inefficient and costly, meaning that they are rarely used in practice.

We present XCLAIM: the first generic framework for achieving trustless and efficient cross-chain exchanges using cryptocurrency-backed assets (CbAs). XCLAIM offers protocols for issuing, transferring, swapping and redeeming CbAs securely in a non-interactive manner on existing blockchains. We instantiate XCLAIM between Bitcoin and Ethereum and evaluate our implementation; it costs less than USD 0.50 to issue an arbitrary amount of Bitcoin-backed tokens on Ethereum. We show XCLAIM is not only faster, but also significantly cheaper than atomic cross-chain swaps. Finally, XCLAIM is compatible with the majority of existing blockchains without modification, and enables several novel cryptocurrency applications, such as cross-chain payment channels and efficient multi-party swaps.

</details>

- **Zamyatin A**, **Harz D**, Lind J, Panayiotou P, Arthur G, Knottenbelt WJ
- IEEE Symposium on Security and Privacy (S&P)
- In collaboration with Imperial College London, SBA Research

### [SoK: Decentralized Finance (DeFi)](https://arxiv.org/abs/2101.08778)

<details>
  <summary>Read abstract</summary>

Decentralized Finance (DeFi), a blockchain powered peer-to-peer financial system, is mushrooming. One and a half years ago the total value locked in DeFi systems was approximately 700m USD, now, as of September 2021, it stands at around 100bn USD. The frenetic evolution of the ecosystem has created challenges in understanding the basic principles of these systems and their security risks. In this Systematization of Knowledge (SoK) we delineate the DeFi ecosystem along the following axes: its primitives, its operational protocol types and its security. We provide a distinction between technical security, which has a healthy literature, and economic security, which is largely unexplored, connecting the latter with new models and thereby synthesizing insights from computer science, economics and finance. Finally, we outline the open research challenges in the ecosystem across these security types.

</details>

- Werner S, Perez D, Gudgeon L, Klages-Mundt A, **Harz D**, Knottenbelt WJ
- In collaboration with Imperial College London, Cornell University

### [Stablecoins 2.0: Economic Foundations and Risk-based Models](https://arxiv.org/pdf/2006.12388.pdf)

<details>
  <summary>Read abstract</summary>

Stablecoins are one of the most widely capitalized type of cryptocurrency. However, their risks vary significantly according to their design and are often poorly understood. We seek to provide a sound foundation for stablecoin theory, with a risk-based functional characterization of the economic structure of stablecoins. First, we match existing economic models to the disparate set of custodial systems. Next, we characterize the unique risks that emerge in non-custodial stablecoins and develop a model framework that unifies existing models from economics and computer science. We further discuss how this modeling framework is applicable to a wide array of cryptoeconomic systems, including cross-chain protocols, collateralized lending, and decentralized exchanges. These unique risks yield unanswered research questions that will form the crux of research in decentralized finance going forward.

</details>

- Klages-Mundt A, **Harz D**, Gudgeon L, Liu J, Minca A
- ACM conference on Advances in Financial Technologies (AFT’20)
- In collaboration with Cornell University, Imperial College London

### [The Decentralized Financial Crisis](https://arxiv.org/pdf/2002.08099.pdf)

<details>
  <summary>Read abstract</summary>

The Global Financial Crisis of 2008, caused by the accumulation of excessive financial risk, inspired Satoshi Nakamoto to create Bitcoin. Now, more than ten years later, Decentralized Finance (DeFi), a peer-to-peer financial paradigm which leverages blockchain-based smart contracts to ensure its integrity and security, contains over 702m USD of capital as of April 15th, 2020. As this ecosystem develops, it is at risk of the very sort of financial meltdown it is supposed to be preventing. In this paper we explore how design weaknesses and price fluctuations in DeFi protocols could lead to a DeFi crisis. We focus on DeFi lending protocols as they currently constitute most of the DeFi ecosystem with a 76% market share by capital as of April 15th, 2020.
First, we demonstrate the feasibility of attacking Maker's governance design to take full control of the protocol, the largest DeFi protocol by market share, which would have allowed the theft of 0.5bn USD of collateral and the minting of an unlimited supply of DAI tokens. In doing so, we present a novel strategy utilizing so-called flash loans that would have in principle allowed the execution of the governance attack in just two transactions and without the need to lock any assets. Approximately two weeks after we disclosed the attack details, Maker modified the governance parameters mitigating the attack vectors. Second, we turn to a central component of financial risk in DeFi lending protocols. Inspired by stress-testing as performed by central banks, we develop a stress-testing framework for a stylized DeFi lending protocol, focusing our attention on the impact of a drying-up of liquidity on protocol solvency. Based on our parameters, we find that with sufficiently illiquidity a lending protocol with a total debt of 400m USD could become undercollateralized within 19 days.

</details>

- Gudgeon L, Perez D, **Harz D**, Livshits B, Gervais A
- 2020 Crypto Valley Conference on Blockchain Technology (CVCBT)
- In collaboration with Imperial College London

### [Commit-Chains: Secure, Scalable Off-Chain Payments](https://eprint.iacr.org/2018/642.pdf)

<details>
  <summary>Read abstract</summary>

Current permissionless blockchains suffer from scalability limitations. To scale without changing the underlying blockchain, one avenue is to lock funds into blockchain smart-contracts (collateral) and enact transactions outside, or off- the blockchain, via accountable peer-to-peer messages. Disputes among peers are resolved with appropriate collateral redistribution on the blockchain. In this work we lay the foundations for commit-chains, a novel off-chain scaling solution for existing blockchains where an untrusted and non-custodial operator commits the state of its user account balances via constant-sized, periodic checkpoints. Users dispute operator misbehavior via a smart contract. The commit-chain paradigm enables for the first time that off-chain users can receive payments while being offline. Moreover, locked funds can be managed efficiently at constant communication costs, alleviating collateral fragmentation.

We instantiate two account-based commit-chain constructions: NOCUST, based on a cost-effective challenge-response dispute mechanism; and NOCUST-ZKP, which provides provably correct operation via zkSNARKs. These constructions offer a trade-off between correctness, verification, and efficiency while both are practical and ensure key properties such as balance safety; that is, no honest user loses coins. We implemented both constructions on a smart contract enabled blockchain. Our evaluation demonstrates that NOCUST's operational costs in terms of computation and communication scale logarithmically in the number of users and transactions, and allow very efficient lightweight clients (a user involved in e.g. 100 daily transactions only needs to store a constant 46 kb of data, allowing secure payments even on mobile devices). NOCUST is operational in production since March 2019.

</details>

- Khalil R, **Zamyatin A**, Felley G, Moreno-Sanchez P, Gervais A
- In collaboration with Imperial College London, TU Wien, Liquidity Network

### [Pay To Win: Cheap, Crowdfundable, Cross-chain Algorithmic Incentive Manipulation Attacks on PoW Cryptocurrencies](https://eprint.iacr.org/2019/775.pdf)

<details>
  <summary>Read abstract</summary>

In this paper we extend the attack landscape of bribing attacks on cryptocurrencies by presenting a new method, which we call Pay-To-Win (P2W). To the best of our knowledge, it is the first approach capable of facilitating double-spend collusion across different blockchains. Moreover, our technique can also be used to specifically incentivize transaction exclusion or (re)ordering. For our construction we rely on smart contracts to render the payment and receipt of bribes trustless for the briber as well as the bribee. Attacks using our approach are operated and financed out-of-band i.e., on a funding cryptocurrency, while the consequences are induced in a different target cryptocurrency. Hereby, the main requirement is that smart contracts on the funding cryptocurrency are able to verify consensus rules of the target. For a concrete instantiation of our P2W method, we choose Bitcoin as a target and Ethereum as a funding cryptocurrency. Our P2W method is designed in a way that reimburses collaborators even in the case of an unsuccessful attack. Interestingly, this actually renders our approach approximately one order of magnitude cheaper than comparable bribing techniques (e.g., the whale attack). We demonstrate the technical feasibility of P2W attacks through publishing all relevant artifacts of this paper, ranging from calculations of success probabilities to a fully functional proof-of-concept implementation, consisting of an Ethereum smart contract and a Python client.

</details>

- Judmayer A, Stifter N, **Zamyatin A**, Tsabary I, Eyal I, Gazi P, Meiklejohn S, Weippl E
- In collaboration with SBA Research, TU Wien, Imperial College London, Technion, IC3, IOHK, and UCL
