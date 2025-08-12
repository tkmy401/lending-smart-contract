use ink_prelude::vec::Vec;
use ink::storage::traits::StorageLayout;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct Loan {
    pub id: u64,
    pub borrower: AccountId,
    pub lender: Option<AccountId>,
    pub amount: Balance,
    pub interest_rate: u16, // Basis points (e.g., 500 = 5%)
    pub duration: u64, // Duration in blocks
    pub collateral: Balance,
    pub status: LoanStatus,
    pub created_at: u64,
    pub due_date: u64,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum LoanStatus {
    Pending,
    Active,
    Repaid,
    Defaulted,
    Liquidated,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct UserProfile {
    pub total_borrowed: Balance,
    pub total_lent: Balance,
    pub active_loans: Vec<u64>,
    pub credit_score: u16,
    pub is_blacklisted: bool,
}

pub type AccountId = <ink_env::DefaultEnvironment as ink_env::Environment>::AccountId;
pub type Balance = <ink_env::DefaultEnvironment as ink_env::Environment>::Balance;
pub type BlockNumber = u64; 