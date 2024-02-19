# Arbiter UniswapV3 Integration
The most hotly requested example to develop alongside Arbiter is with the Uniswap V3 contracts.
Uniswap V3 is a decentralized exchange that allows users to provide liquidity to the platform and earn fees in return.
Liquidity management is a key aspect of Uniswap V3 and there is much to learn about how to manage liquidity in a way that is profitable and sustainable.
We hope to provide useful behaviors and simulations from those that will allow people to battletest the protocol and their own strategies on top of it.

## Goals
**Initially:**
- [x] Create basic deployment behaviors to get the Uniswap V3 contracts deployed into the Arbiter environment.
- [ ] Create basic liquidity management behaviors to allow users to provide liquidity to the Uniswap V3 contracts.
- [ ] Create basic swapping behaviors to allow users to swap on the Uniswap V3 contracts.
    - [ ] Create an arbitrage behavior so that UniV3 can track simulated price paths.

--- 
**Later:**
- [ ] Fork state from mainnet to allow for simulations from historical data.
- [ ] Thoroughly manage gas costs and other costs associated with interacting with the Uniswap V3 contracts.
- [ ] Create more complex liquidity management behaviors to allow users to manage their liquidity in a more sophisticated way, e.g., with optimization schemes.


## Ideology
We should aim to create extensible and generic behaviors that can be customized to do more complex things.
For instance, swapping can be extended to include more complex logic for determining the best price to swap at, e.g., arbitrage.
We want these behaviors to be as generic and nuclear as possible so that they can be used in other contexts like legos.
This will allow us to build more complex strategies on top of these basic behaviors.
Code here will be reusable, extensible, modular, and composable.

Of course, we will also go about this iteratively. 
We will not have the perfect abstractions from the start, but we will aim to refactor and improve the code as we go along.
Efforts here will likely induce changes in the Arbiter core or engine, and we will aim to make those changes as well.

## Contribution
We are open to contributions from the community!
If you have any issues to make, PRs to open, or other discussions to have, please feel free to do so.