# Sombrero Protocol

A dynamic bug bounty protocol that completely reshapes how web3 security is built.

The Sombrero Protocol is an open-source, decentralized, and automatic bounty marketplace for the Cosmos ecosystem, allowing hackers to search for subscribed contracts that could be exploited in exchange for a reward.

The concept behind this protocol is that it creates a win-win scenario for all parties involved:

    1. Hackers earn a reward for their efforts while remaining anonymous.
    2. Subscribed contracts recover most of their funds and can quickly update the bug, keeping smart contracts in the ecosystem more secure.

To participate in the bounty marketplace, contracts must subscribe to the protocol and set their bounty conditions upfront. These conditions include:

    1. The address of the smart contract to be protected.
    2. The percentage of the total hacked amount that hackers will keep as a reward.
    3. The minimum amount for small hack attempts.

All bounty conditions are recorded on-chain in the smart contract, eliminating the need for negotiations or personal information sharing.

Once a hacker successfully exploits a protected contract and sends the stolen funds to the Sombrero smart contract, the reward is automatically paid to the hacker, the subscribed contract receives the remaining tokens, and a fee is collected by the protocol, which can be withdrawn later by the contract administrator.

As a proof of their achievement, hackers will receive a freshly minted NFT that records the hack conditions on-chain. This NFT can be kept as a badge of honor to showcase the hacker's skills and contributions to the ecosystem's security.

The NFT metadata will live on-chain and contains the following traits:

    Date,
    Contract exploited,
    Total amount hacked,
    Bounty colelcted,
    Hacker address,
