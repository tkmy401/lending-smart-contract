# Lending Smart Contract

A professional and secure lending smart contract built with Rust using the ink! framework for Substrate-based blockchains.

## Features

- **Loan Creation**: Users can create loan requests with specified amounts, interest rates, and collateral
- **Loan Funding**: Lenders can fund pending loans and earn interest
- **Loan Repayment**: Borrowers can repay loans with interest
- **Early Repayment with Discount**: Save 1-5% on interest by repaying early
- **Partial Repayment**: Pay off loans in multiple smaller installments
- **Loan Extension**: Extend loan duration with configurable fees
- **Late Payment Penalties**: Automatic late fees for overdue loans
- **Loan Refinancing**: Refinance loans with better terms and lower rates
- **Variable Interest Rates**: Dynamic rates with risk-based pricing
- **Compound Interest Calculation**: Sophisticated interest computation with multiple frequencies
- **Collateral Management**: Secure collateral system with configurable ratios
- **User Profiles**: Track user borrowing/lending history and credit scores
- **Event System**: Comprehensive event logging for all operations
- **Security Features**: Input validation, access control, and error handling
- **Credit Scoring**: Multi-factor credit assessment with risk level classification
- **Collateral Management**: Dynamic collateral requirements with liquidation thresholds
- **Insurance Policies**: Loan protection with configurable premiums and coverage
- **Fraud Detection**: Real-time monitoring with rule-based detection systems
- **Compliance Management**: KYC/AML verification and regulatory compliance tools

## 🚀 Feature Roadmap

For a comprehensive overview of planned features and development phases, see **[FEATURE_ROADMAP.md](./FEATURE_ROADMAP.md)**.

**Current Phase**: Phase 4 - Risk Management & Security ✅ **COMPLETED**  
**Next Phase**: Phase 5 - Advanced Analytics & Reporting

The roadmap includes 8 development phases covering:
- Core lending enhancements
- Advanced financial features
- Liquidity pool management
- Risk management & security
- DeFi integrations
- Performance optimizations

## Project Structure

```
lending-smart-contract/
├── src/
│   ├── lib.rs              # Main library entry point
│   ├── lending_contract.rs # Core lending contract implementation
│   ├── types.rs            # Data structures and types
│   └── errors.rs           # Custom error definitions
├── tests/
│   └── lending_contract_tests.rs # Test suite (requires updates for ink! 5.x)
├── examples/
│   ├── basic_usage.rs      # Basic usage examples
│   └── advanced_features.rs # Advanced features demonstration
├── scripts/
│   └── build.sh            # Build automation script
├── Cargo.toml              # Project dependencies and configuration
├── README.md               # This file
└── FEATURE_ROADMAP.md      # Comprehensive feature development roadmap
```

## Prerequisites

- Rust 1.70.0 or later
- Cargo package manager
- ink! 5.x toolchain

## Installation

1. Clone the repository:
```bash
git clone https://github.com/tkmy401/lending-smart-contract.git
cd lending-smart-contract
```

2. Build the contract:
```bash
cargo build
```

## Current Status

✅ **Core Contract**: Successfully compiles and builds  
✅ **Phase 1**: Core Enhancements (100% Complete)  
✅ **Phase 2**: Advanced Financial Features (100% Complete)  
✅ **Phase 3**: Liquidity Pool Management (100% Complete)  
✅ **Phase 4**: Risk Management & Security (100% Complete)  
⏳ **Phase 5**: Advanced Analytics & Reporting (Ready to begin)  
🔧 **Dependencies**: Updated to use ink! 5.1.1 with compatible crates

## Dependencies

The project uses the following key dependencies:
- `ink = "5.1.1"` - Core ink! framework
- `parity-scale-codec = "3.6.0"` - SCALE encoding/decoding
- `scale-info = "2.11.6"` - Type information for ink! contracts

## Usage

### Contract Deployment

After building, you can deploy the contract to a Substrate-based blockchain:

```bash
# Build the contract
cargo build --release

# The compiled contract will be available in target/ink/
```

### Core Functions

#### Create Loan
```rust
create_loan(
    amount: Balance,        // Loan amount
    interest_rate: u16,     // Interest rate in basis points (500 = 5%)
    duration: u64,          // Loan duration in blocks
    collateral: Balance     // Collateral amount
) -> Result<u64, LendingError>
```

#### Fund Loan
```rust
fund_loan(loan_id: u64) -> Result<(), LendingError>
```

#### Repay Loan
```rust
repay_loan(loan_id: u64) -> Result<(), LendingError>
```

#### Early Repay Loan (with Discount)
```rust
early_repay_loan(loan_id: u64) -> Result<(), LendingError>
```

#### Partial Repay Loan
```rust
partial_repay_loan(loan_id: u64) -> Result<(), LendingError>
```

#### Extend Loan
```rust
extend_loan(loan_id: u64, extension_duration: u64) -> Result<(), LendingError>
```

#### Apply Late Fees
```rust
apply_late_fees(loan_id: u64) -> Result<(), LendingError>
```

#### Refinance Loan
```rust
refinance_loan(loan_id: u64, new_interest_rate: u16, new_duration: u64) -> Result<(), LendingError>
```

#### Variable Interest Rate Management
```rust
adjust_interest_rate(loan_id: u64, new_base_rate: u16, reason: RateAdjustmentReason) -> Result<(), LendingError>
update_risk_multiplier(loan_id: u64, new_risk_multiplier: u16) -> Result<(), LendingError>
convert_to_variable_rate(loan_id: u64, new_base_rate: u16) -> Result<(), LendingError>
```

#### Compound Interest Management
```rust
convert_to_compound_interest(loan_id: u64, frequency: CompoundFrequency) -> Result<(), LendingError>
compound_interest(loan_id: u64) -> Result<(), LendingError>
get_compound_interest_info(loan_id: u64) -> Result<(InterestType, CompoundFrequency, u64, Balance, Balance), LendingError>
calculate_accrued_interest(loan_id: u64) -> Result<Balance, LendingError>
```

#### Interest-Only Payment Management
```rust
set_interest_only_periods(loan_id: u64, periods: u32, payment_period_blocks: u64) -> Result<(), LendingError>
make_interest_only_payment(loan_id: u64) -> Result<(), LendingError>
switch_to_principal_and_interest(loan_id: u64) -> Result<(), LendingError>
get_payment_structure_info(loan_id: u64) -> Result<(PaymentStructure, u32, u32, u32, u64, Balance), LendingError>
```

#### Grace Period Management
```rust
grant_grace_period(loan_id: u64, duration: u64, reason: GracePeriodReason) -> Result<(), LendingError>
is_within_grace_period(loan_id: u64) -> Result<bool, LendingError>
get_grace_period_info(loan_id: u64) -> Result<(u64, u64, u32, u32, GracePeriodReason, Vec<GracePeriodRecord>), LendingError>
calculate_remaining_grace_period(loan_id: u64) -> Result<u64, LendingError>
set_custom_grace_period(loan_id: u64, grace_period_blocks: u64, max_extensions: u32) -> Result<(), LendingError>
```

#### Liquidity Pool Management
```rust
create_liquidity_pool(name: String, initial_liquidity: Balance, pool_fee_rate: u16, reward_rate: u16, min_liquidity: Balance, max_liquidity: Balance) -> Result<u64, LendingError>
provide_liquidity(pool_id: u64, amount: Balance) -> Result<(), LendingError>
claim_pool_rewards(pool_id: u64) -> Result<Balance, LendingError>
get_liquidity_pool_info(pool_id: u64) -> Result<(String, Balance, u32, Balance, u16, u16, PoolStatus), LendingError>
get_liquidity_provider_info(pool_id: u64, provider: AccountId) -> Result<(Balance, u16, Balance, u64), LendingError>
```

#### Pool Rebalancing & Dynamic Liquidity Management
```rust
rebalance_pool(pool_id: u64) -> Result<(), LendingError>
set_auto_rebalancing(pool_id: u64, enabled: bool) -> Result<(), LendingError>
set_rebalancing_parameters(pool_id: u64, frequency: u64, target_ratio: u16, threshold: u16) -> Result<(), LendingError>
needs_rebalancing(pool_id: u64) -> Result<bool, LendingError>
get_pool_rebalancing_info(pool_id: u64) -> Result<(u16, u64, u64, u16, u16, bool), LendingError>
```

#### Yield Farming & Advanced Rewards
```rust
enable_yield_farming(pool_id: u64, reward_tokens: Vec<RewardToken>) -> Result<(), LendingError>
stake_tokens(pool_id: u64, amount: Balance) -> Result<(), LendingError>
claim_yield_rewards(pool_id: u64) -> Result<Balance, LendingError>
get_yield_farming_info(pool_id: u64) -> Result<(bool, u32, Balance, u32), LendingError>
get_staking_tiers(pool_id: u64) -> Result<Vec<(String, Balance, u16, u16)>, LendingError>
```

#### Query Functions
```rust
get_loan(loan_id: u64) -> Option<Loan>
get_user_profile(user: AccountId) -> Option<UserProfile>
get_total_loans() -> u64
get_total_liquidity() -> Balance
get_early_repayment_discount(loan_id: u64) -> Result<u16, LendingError>
get_loan_payment_info(loan_id: u64) -> Result<(Balance, Balance, Vec<PartialPayment>), LendingError>
get_partial_payment_count(loan_id: u64) -> Result<u32, LendingError>
get_loan_extension_info(loan_id: u64) -> Result<(u32, u32, u16), LendingError>
can_extend_loan(loan_id: u64) -> Result<bool, LendingError>
calculate_extension_fee(loan_id: u64) -> Result<Balance, LendingError>
get_late_fee_info(loan_id: u64) -> Result<(Balance, u16, u16, Option<u64>), LendingError>
calculate_current_late_fees(loan_id: u64) -> Result<Balance, LendingError>
is_loan_overdue(loan_id: u64) -> Result<bool, LendingError>
get_loan_refinance_info(loan_id: u64) -> Result<(u32, u32, u16), LendingError>
can_refinance_loan(loan_id: u64) -> Result<bool, LendingError>
calculate_refinance_fee(loan_id: u64) -> Result<Balance, LendingError>
get_refinance_history(loan_id: u64) -> Result<Vec<RefinanceRecord>, LendingError>
```

## Testing

Currently, the test suite requires updates for ink! 5.x compatibility. The main contract compiles successfully:

```bash
# Build the contract (works)
cargo build

# Run tests (requires updates)
cargo test
```

## Security Features

- **Input Validation**: All inputs are validated before processing
- **Access Control**: Only authorized users can perform specific actions
- **Collateral Requirements**: Minimum collateral ratios enforced
- **Error Handling**: Comprehensive error handling with custom error types
- **Event Logging**: All operations are logged for transparency

## Configuration

The contract includes several configurable parameters:

- `protocol_fee`: Protocol fee in basis points (default: 50 = 0.5%)
- `min_collateral_ratio`: Minimum collateral ratio (default: 150 = 150%)
- `max_interest_rate`: Maximum allowed interest rate (default: 10000 = 100%)

## Recent Fixes Applied

The following issues have been resolved:
- ✅ Fixed compilation errors with ink! 5.x compatibility
- ✅ Updated dependencies to use compatible versions
- ✅ Added missing `TypeInfo` derive macros
- ✅ Fixed type mismatches and import issues
- ✅ Corrected module structure and exports

## Next Steps

To complete the project setup:
1. Update test suite for ink! 5.x compatibility
2. Update examples for ink! 5.x compatibility
3. Add comprehensive integration tests
4. Implement additional security features

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Disclaimer

This smart contract is for educational and development purposes. Please conduct thorough testing and security audits before using in production environments. 