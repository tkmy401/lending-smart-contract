# Lending Smart Contract

A professional and secure lending smart contract built with Rust using the ink! framework for Substrate-based blockchains.

## Features

- **Loan Creation**: Users can create loan requests with specified amounts, interest rates, and collateral
- **Loan Funding**: Lenders can fund pending loans and earn interest
- **Loan Repayment**: Borrowers can repay loans with interest
- **Collateral Management**: Secure collateral system with configurable ratios
- **User Profiles**: Track user borrowing/lending history and credit scores
- **Event System**: Comprehensive event logging for all operations
- **Security Features**: Input validation, access control, and error handling

## Project Structure

```
lending-smart-contract/
├── src/
│   ├── lib.rs              # Main library entry point
│   ├── lending_contract.rs # Core lending contract implementation
│   ├── types.rs            # Data structures and types
│   └── errors.rs           # Custom error definitions
├── tests/
│   └── lending_contract_tests.rs # Comprehensive test suite
├── Cargo.toml              # Project dependencies and configuration
└── README.md               # This file
```

## Prerequisites

- Rust 1.70.0 or later
- Cargo package manager
- ink! toolchain

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd lending-smart-contract
```

2. Install ink! toolchain:
```bash
cargo install cargo-contract --locked --git https://github.com/paritytech/cargo-contract
```

3. Build the contract:
```bash
cargo contract build
```

## Usage

### Contract Deployment

After building, you can deploy the contract to a Substrate-based blockchain:

```bash
cargo contract upload --suri //Alice
cargo contract instantiate --suri //Alice
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

#### Query Functions
```rust
get_loan(loan_id: u64) -> Option<Loan>
get_user_profile(user: AccountId) -> Option<UserProfile>
get_total_loans() -> u64
get_total_liquidity() -> Balance
```

## Testing

Run the test suite:

```bash
cargo test
```

Run end-to-end tests:

```bash
cargo test --features e2e-tests
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