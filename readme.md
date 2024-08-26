# Simple Automated Market Maker (AMM) on Solana

## Summary

This project is a basic implementation of an Automated Market Maker (AMM) smart contract using Rust and the Solana Program Library (SPL). The contract allows users to create and manage a liquidity pool for two tokens, enabling liquidity providers to add or remove liquidity and traders to swap between the tokens. The AMM operates using the constant product formula (x * y = k) and includes a simple fee mechanism to incentivize liquidity providers.

## Features

- **Liquidity Pool Creation**: Users can create a new liquidity pool with two tokens.
- **Add Liquidity**: Liquidity providers can add specified amounts of the two tokens to the pool.
- **Remove Liquidity**: Liquidity providers can remove their share of liquidity from the pool.
- **Token Swaps**: Users can swap between the two tokens in the pool using the constant product formula.
- **Fee Mechanism**: A 0.3% fee is applied to all swaps, benefiting liquidity providers.
- **Fee Management**: The fee recipient and fee amount can be updated by authorized users.
- **Basic Error Handling**: Includes validation for input amounts and pool conditions.
- **Upgradeable Parameters**: The fee recipient address and fee amount are updatable.

## Assumptions and Limitations

- **No Price Oracle Integration**: This implementation does not include a price oracle, meaning it does not track external market prices or adjust prices accordingly.
- **No Frontend**: A frontend is not included, but includes basic CLI tool to interact with the contract.
- **Single Pool Support**: This implementation is designed for a single liquidity pool. Supporting multiple pools or more complex pool configurations would require additional development.
- **Basic Error Handling**: While basic error handling is implemented, more comprehensive validation could be added for production use.
- **Constant Product Formula**: The AMM uses the constant product formula for price determination, which may not be suitable for all types of assets, especially those with low liquidity.

## Prerequisites

**Anchor**: This project is built with Anchor so visit [Anchor official installation](https://www.anchor-lang.com/docs/installation) page and install all necessary modules.

## Build, Test and Deploy

First, install dependencies:

```
$ yarn
```

Next, we will build and deploy the program via Anchor.

Build the program:

```
$ anchor build
```

Let's deploy the program.
```
$ anchor deploy
```

Finally, run the test:

```
$ anchor test
```