# White Hat Hacker NFT

The White Hat Hacker NFT is an open-source, descentralized, and automatic bounty marketplace for the Cosmos ecosystem, allowing hackers to search for subscribed contracts that could be exploited in exchange for a reward.

The concept behind this smart contract is that it creates a win-win scenario for all parties involved. Hackers are able to earn a reward for their efforts without facing legal repercussions, subscribed contracts are able to recover most of their funds, and overall, smart contracts in the ecosystem become more secure.

To participate in the bounty marketplace, contracts must subscribe to the protocol and set their bounty conditions upfront. These conditions include the address of the smart contract to be protected, the percentage of the total hacked amount that hackers will keep as a reward, and a minimum amount for small hack attempts.

The `Subscribe` message contains the following fields:

    protected_contract: String
    bounty_pct: u16
    min_bounty: Option<u128>

All bounty conditions are recorded on-chain in the smart contract, eliminating the need for negotiations or personal information sharing.

Once a hacker successfully exploits a protected contract and sends the stolen funds to the White Hat Hacker NFT smart contract, the reward is automatically paid to the hacker, the subscribed contract receives the remaining tokens, and a fee is collected by the protocol which can be withdrawn later by the contract administrator.

As a proof of their achievement, hackers will receive a freshly minted NFT that records the hack conditions on chain. This NFT can be kept as a badge of honor to showcase the hacker's skills and contributions to the ecosystem's security.

The NFT metadata will live onchain and contains the following traits:

    date: String,
    contract_exploited: String,
    total_amount_hacked: String,
    bounty: String,
    hacker_addr: String,

