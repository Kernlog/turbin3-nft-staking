# Turbin3 NFT Staking

A Solana-based NFT staking program built with Anchor framework that allows users to stake their NFTs and earn rewards over time.

## Overview

This program implements a complete NFT staking system where users can:
- Stake NFTs from verified collections
- Earn reward points based on staking duration
- Unstake NFTs after a minimum lock period
- Claim accumulated points as reward tokens

## Features

- **Secure NFT Staking**: NFTs are frozen during staking to prevent unauthorized transfers
- **Collection Verification**: Only NFTs from verified collections can be staked
- **Time-based Rewards**: Points are calculated based on staking duration
- **Flexible Configuration**: Global parameters for reward rates, limits, and lock periods
- **PDA-based Architecture**: Uses Program Derived Addresses for secure account management

## Architecture

### Core Components

- **StakeConfig**: Global configuration for staking parameters
- **UserAccount**: Tracks individual user's staking activity and rewards
- **StakeAccount**: Records individual NFT staking instances
- **Rewards Mint**: Token mint for distributing rewards

### Program Instructions

1. **initialize_config**: Sets up global staking parameters and reward token mint
2. **initialize_user**: Creates user account for tracking staking activity
3. **stake**: Stakes an NFT and begins reward accumulation
4. **unstake**: Unstakes an NFT and calculates earned rewards
5. **claim**: Converts accumulated points into reward tokens

## Technical Details

### Security Model

- NFTs are frozen during staking using Metaplex metadata program
- PDA-controlled accounts prevent unauthorized access
- Collection verification ensures only legitimate NFTs can be staked
- Atomic state changes ensure consistency

### Reward System

- Points are awarded based on staking duration and configured rate
- Minimum staking period prevents gaming of the system
- Points can be converted to tokens with proper decimal handling

## Development

### Prerequisites

- Rust 1.70+
- Solana CLI 1.16+
- Anchor CLI 0.31+

### Setup

1. Clone the repository:
```bash
git clone git@github.com:Kernlog/turbin3-nft-staking.git
cd turbin3-nft-staking
```

2. Install dependencies:
```bash
yarn install
```

3. Build the program:
```bash
anchor build
```

4. Run tests:
```bash
anchor test
```

### Project Structure

```
programs/turbin3-nft-staking/src/
├── lib.rs              # Main program entry point
├── error.rs            # Custom error definitions
├── state/              # Account state structures
│   ├── stake_config.rs
│   ├── user_account.rs
│   └── stake_account.rs
└── instructions/       # Program instructions
    ├── initialize_config.rs
    ├── initialize_user.rs
    ├── stake.rs
    ├── unstake.rs
    └── claim.rs
```

## Usage

### Initialization

1. Initialize the global configuration with staking parameters
2. Create user accounts for participants
3. Configure reward token mint with proper decimals

### Staking Process

1. User approves NFT for staking
2. Program verifies collection membership
3. NFT is frozen and stake record created
4. User begins earning points based on duration

### Unstaking Process

1. Verify minimum staking period has elapsed
2. Calculate earned points based on duration
3. Thaw NFT and return control to user
4. Update user's stake count and points

### Claiming Rewards

1. Convert accumulated points to reward tokens
2. Mint tokens to user's reward account
3. Reset points to prevent double-claiming

## Configuration

The program supports the following global parameters:

- `points_per_stake`: Points awarded per day of staking
- `max_stake`: Maximum number of NFTs a user can stake
- `freeze_period`: Minimum staking duration in days
- `rewards_decimals`: Decimal places for reward tokens

## Testing

The project includes comprehensive tests covering:

- Account initialization and validation
- NFT staking and unstaking workflows
- Reward calculation and claiming
- Error handling and edge cases
- Security validations

Run the test suite with:
```bash
anchor test
```

## Security Considerations

- All account validations are enforced at the program level
- PDA seeds ensure unique account addresses
- Collection verification prevents unauthorized NFTs
- Freeze/thaw operations prevent NFT transfers during staking
- Proper error handling prevents invalid state transitions
