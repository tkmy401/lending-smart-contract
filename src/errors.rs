use ink_prelude::string::String;

#[derive(Debug, PartialEq, Eq, parity_scale_codec::Encode, parity_scale_codec::Decode, scale_info::TypeInfo)]
pub enum LendingError {
    InsufficientBalance,
    InsufficientCollateral,
    LoanNotFound,
    LoanNotActive,
    LoanAlreadyRepaid,
    InvalidInterestRate,
    InvalidDuration,
    UserBlacklisted,
    Unauthorized,
    TransferFailed,
    InvalidAmount,
    LoanExpired,
    CollateralSeized,
}

impl From<LendingError> for String {
    fn from(error: LendingError) -> Self {
        match error {
            LendingError::InsufficientBalance => "Insufficient balance".into(),
            LendingError::InsufficientCollateral => "Insufficient collateral".into(),
            LendingError::LoanNotFound => "Loan not found".into(),
            LendingError::LoanNotActive => "Loan is not active".into(),
            LendingError::LoanAlreadyRepaid => "Loan already repaid".into(),
            LendingError::InvalidInterestRate => "Invalid interest rate".into(),
            LendingError::InvalidDuration => "Invalid duration".into(),
            LendingError::UserBlacklisted => "User is blacklisted".into(),
            LendingError::Unauthorized => "Unauthorized action".into(),
            LendingError::TransferFailed => "Transfer failed".into(),
            LendingError::InvalidAmount => "Invalid amount".into(),
            LendingError::LoanExpired => "Loan has expired".into(),
            LendingError::CollateralSeized => "Collateral has been seized".into(),
        }
    }
} 