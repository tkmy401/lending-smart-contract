use ink::storage::Mapping;
use ink_prelude::vec::Vec;

use crate::types::{Loan, LoanStatus, UserProfile};
use crate::errors::LendingError;

#[ink::contract]
pub mod lending_contract {
    use super::*;

    #[ink(storage)]
    pub struct LendingContract {
        owner: AccountId,
        total_loans: u64,
        loans: Mapping<u64, Loan>,
        user_profiles: Mapping<AccountId, UserProfile>,
        total_liquidity: Balance,
        protocol_fee: u16, // Basis points
        min_collateral_ratio: u16, // Basis points
    }

    #[ink(event)]
    pub struct LoanCreated {
        #[ink(topic)]
        loan_id: u64,
        borrower: AccountId,
        amount: Balance,
        interest_rate: u16,
        duration: u64,
    }

    #[ink(event)]
    pub struct LoanFunded {
        #[ink(topic)]
        loan_id: u64,
        lender: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct LoanRepaid {
        #[ink(topic)]
        loan_id: u64,
        borrower: AccountId,
        amount: Balance,
    }

    impl LendingContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                total_loans: 0,
                loans: Mapping::default(),
                user_profiles: Mapping::default(),
                total_liquidity: 0,
                protocol_fee: 50, // 0.5%
                min_collateral_ratio: 150, // 150%
            }
        }

        #[ink(message)]
        pub fn create_loan(
            &mut self,
            amount: Balance,
            interest_rate: u16,
            duration: u64,
            collateral: Balance,
        ) -> Result<u64, LendingError> {
            let caller = self.env().caller();
            
            // Validate parameters
            if amount == 0 || interest_rate == 0 || duration == 0 {
                return Err(LendingError::InvalidAmount);
            }
            
            if interest_rate > 10000 { // Max 100%
                return Err(LendingError::InvalidInterestRate);
            }
            
            if duration > 1000000 { // Max ~1 year (assuming 6s blocks)
                return Err(LendingError::InvalidDuration);
            }

            // Check if user is blacklisted
            let user_profile = self.get_or_create_user_profile(caller);
            if user_profile.is_blacklisted {
                return Err(LendingError::UserBlacklisted);
            }

            // Validate collateral ratio
            let required_collateral = (amount * self.min_collateral_ratio as u128) / 10000;
            if collateral < required_collateral {
                return Err(LendingError::InsufficientCollateral);
            }

            let loan_id = self.total_loans + 1;
            let current_block = self.env().block_number() as u64;
            
            let loan = Loan {
                id: loan_id,
                borrower: caller,
                lender: None,
                amount,
                interest_rate,
                duration,
                collateral,
                status: LoanStatus::Pending,
                created_at: current_block,
                due_date: current_block + duration,
            };

            self.loans.insert(loan_id, &loan);
            self.total_loans = loan_id;

            // Update user profile
            let mut profile = user_profile;
            profile.active_loans.push(loan_id);
            self.user_profiles.insert(caller, &profile);

            self.env().emit_event(LoanCreated {
                loan_id,
                borrower: caller,
                amount,
                interest_rate,
                duration,
            });

            Ok(loan_id)
        }

        #[ink(message)]
        pub fn fund_loan(&mut self, loan_id: u64) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            if loan.status != LoanStatus::Pending {
                return Err(LendingError::LoanNotActive);
            }

            if loan.lender.is_some() {
                return Err(LendingError::Unauthorized);
            }

            // Transfer funds from lender to contract
            if self.env().transferred_value() != loan.amount {
                return Err(LendingError::InvalidAmount);
            }

            loan.lender = Some(caller);
            loan.status = LoanStatus::Active;
            self.loans.insert(loan_id, &loan);

            // Update lender profile
            let mut lender_profile = self.get_or_create_user_profile(caller);
            lender_profile.total_lent += loan.amount;
            lender_profile.active_loans.push(loan_id);
            self.user_profiles.insert(caller, &lender_profile);

            self.total_liquidity += loan.amount;

            self.env().emit_event(LoanFunded {
                loan_id,
                lender: caller,
                amount: loan.amount,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn repay_loan(&mut self, loan_id: u64) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            if loan.borrower != caller {
                return Err(LendingError::Unauthorized);
            }
            
            if loan.status != LoanStatus::Active {
                return Err(LendingError::LoanNotActive);
            }

            let repayment_amount = self.calculate_repayment_amount(loan_id)?;
            
            if self.env().transferred_value() != repayment_amount {
                return Err(LendingError::InvalidAmount);
            }

            // Transfer repayment to lender
            if let Some(lender) = loan.lender {
                let lender_repayment = loan.amount + 
                    ((loan.amount * loan.interest_rate as u128) / 10000);
                
                self.env().transfer(lender, lender_repayment)
                    .map_err(|_| LendingError::TransferFailed)?;
            }

            loan.status = LoanStatus::Repaid;
            self.loans.insert(loan_id, &loan);

            // Update borrower profile
            let mut borrower_profile = self.get_or_create_user_profile(caller);
            borrower_profile.total_borrowed += loan.amount;
            borrower_profile.active_loans.retain(|&id| id != loan_id);
            self.user_profiles.insert(caller, &borrower_profile);

            // Update lender profile
            if let Some(lender) = loan.lender {
                let mut lender_profile = self.get_or_create_user_profile(lender);
                lender_profile.active_loans.retain(|&id| id != loan_id);
                self.user_profiles.insert(lender, &lender_profile);
            }

            self.total_liquidity -= loan.amount;

            self.env().emit_event(LoanRepaid {
                loan_id,
                borrower: caller,
                amount: repayment_amount,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn get_loan(&self, loan_id: u64) -> Option<Loan> {
            self.loans.get(loan_id)
        }

        #[ink(message)]
        pub fn get_user_profile(&self, user: AccountId) -> Option<UserProfile> {
            self.user_profiles.get(user)
        }

        #[ink(message)]
        pub fn get_total_loans(&self) -> u64 {
            self.total_loans
        }

        #[ink(message)]
        pub fn get_total_liquidity(&self) -> Balance {
            self.total_liquidity
        }

        // Private helper methods
        fn get_or_create_user_profile(&self, user: AccountId) -> UserProfile {
            self.user_profiles.get(user).unwrap_or(UserProfile {
                total_borrowed: 0,
                total_lent: 0,
                active_loans: Vec::new(),
                credit_score: 700, // Default credit score
                is_blacklisted: false,
            })
        }

        fn calculate_repayment_amount(&self, loan_id: u64) -> Result<Balance, LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            let interest_amount = (loan.amount * loan.interest_rate as u128) / 10000;
            Ok(loan.amount + interest_amount)
        }
    }
} 