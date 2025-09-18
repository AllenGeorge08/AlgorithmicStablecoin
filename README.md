## Stablecoin (Anchor + Solana)

A minimal stablecoin protocol built with Anchor on Solana. Users deposit SOL as collateral and mint a token backed by the collateral. The protocol integrates Pyth price feeds to compute collateral value, enforce a minimum health factor, and support liquidation when positions are undercollateralized.

### Prerequisites
- Rust toolchain and Cargo
- Solana CLI (matching the Anchor toolchain)
- Anchor CLI
- Node.js and Yarn (for the TypeScript tests)

### Setup
```bash
git clone https://github.com/AllenGeorge08/AlgorithmicStablecoin.git
cd stablecoin
npm install
```

### Build
```bash
anchor build
```

### Test
```bash
anchor test
```
If you encounter Node module resolution issues, run a clean install:
```bash
rm -rf node_modules package-lock.json && npm install
```

## Program Instructions

The on-chain program is defined in `programs/stablecoin/src`. Entry points are in `lib.rs` and delegate to instruction handlers.

### initialize_config
Initializes protocol configuration and the token mint. Stores authority, mint, liquidation parameters, and bumps.

### update_config
Updates selected configuration fields (e.g., `min_health_factor`). Restricted to the protocol authority.

### deposit_collateral_and_mint
Deposits SOL as collateral and mints stablecoins to the user’s associated token account. Checks health factor using the latest Pyth price update.

### redeem_collateral_and_burn_tokens
Burns a specified amount of stablecoins and withdraws SOL collateral back to the user, ensuring the position remains healthy.

### liquidate
When a position’s health factor falls below the minimum, a liquidator can burn stablecoins to seize a discounted amount of the position’s SOL collateral (liquidation bonus applied).

## Key Components
- `constants.rs`: Seeds, price feed id, decimals, thresholds, and configuration constants.
- `state/`: Account structures such as `Config` and `Collateral`.
- `instructions/`: Handlers for deposit/mint, redeem/burn, and liquidation flows, plus admin setup.

## Notes
- Price data is consumed from Pyth via `PriceUpdateV2` accounts.
- Token functionality uses SPL Token 2022 interfaces via `anchor_spl::token_interface`.

