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
    pub early_repayment_discount: u16, // Early repayment discount in basis points (default: 200 = 2%)
    pub total_paid: Balance, // Total amount paid so far (principal + interest)
    pub remaining_balance: Balance, // Remaining balance to be paid
    pub partial_payments: Vec<PartialPayment>, // History of partial payments
    pub extension_count: u32, // Number of times loan has been extended
    pub max_extensions: u32, // Maximum allowed extensions (default: 3)
    pub extension_fee_rate: u16, // Extension fee in basis points (default: 100 = 1%)
    pub late_fee_rate: u16, // Daily late fee rate in basis points (default: 50 = 0.5%)
    pub max_late_fee_rate: u16, // Maximum late fee rate in basis points (default: 1000 = 10%)
    pub total_late_fees: Balance, // Total late fees accumulated
    pub overdue_since: Option<u64>, // Block number when loan became overdue
    pub grace_period: u64, // Grace period in blocks before late fees start (default: 100 = ~10 minutes)
    pub refinance_count: u32, // Number of times loan has been refinanced
    pub max_refinances: u32, // Maximum allowed refinances (default: 2)
    pub refinance_fee_rate: u16, // Refinance fee in basis points (default: 200 = 2%)
    pub original_loan_id: Option<u64>, // ID of the original loan if this is a refinanced loan
    pub refinance_history: Vec<RefinanceRecord>, // History of refinancing operations
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum LoanStatus {
    Pending,
    Active,
    PartiallyPaid, // New status for loans with partial payments
    Repaid,
    EarlyRepaid, // New status for early repayment
    Overdue, // New status for overdue loans
    Refinanced, // New status for refinanced loans
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

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PartialPayment {
    pub amount: Balance,
    pub timestamp: u64, // Block number when payment was made
    pub payment_type: PaymentType,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct RefinanceRecord {
    pub timestamp: u64, // Block number when refinancing occurred
    pub old_lender: AccountId, // Previous lender
    pub new_lender: AccountId, // New lender
    pub old_interest_rate: u16, // Previous interest rate
    pub new_interest_rate: u16, // New interest rate
    pub refinance_fee: Balance, // Fee paid for refinancing
    pub remaining_balance: Balance, // Remaining balance at time of refinancing
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum PaymentType {
    Partial,
    Full,
    Early,
}

pub type AccountId = <ink_env::DefaultEnvironment as ink_env::Environment>::AccountId;
pub type Balance = <ink_env::DefaultEnvironment as ink_env::Environment>::Balance;
pub type BlockNumber = u64; 