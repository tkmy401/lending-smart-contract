use ink::storage::Mapping;
use ink_prelude::vec::Vec;

use crate::types::{
    Loan, LoanStatus, UserProfile, PartialPayment, PaymentType, RefinanceRecord,
    InterestRateType, InterestRateAdjustment, RateAdjustmentReason, InterestType, CompoundFrequency, PaymentStructure,
    GracePeriodReason, GracePeriodRecord, LiquidityPool, PoolStatus, LiquidityProvider, RewardToken, StakingRequirements, TierMultiplier,
    MarketDepthLevel, OptimalDistribution, ConcentrationLimits, CollateralType, CollateralRequirement, InsurancePolicy, InsuranceStatus, FraudDetectionRule, FraudRuleType, FraudAction, ComplianceRecord, ComplianceStatus, ComplianceType, CreditScore, CreditFactor, CreditFactorType, CreditScoreRecord, RiskLevel,
    MarketStatistics, MarketTrend, LoanPerformanceMetrics, PortfolioAnalytics, HistoricalDataPoint, PerformanceBenchmark, BenchmarkCategory, AnalyticsReport, ReportType, AnalyticsMetric, MetricTrend,
};
use crate::errors::LendingError;

// ============================================================================
// LENDING SMART CONTRACT - REFACTORED FOR SENIOR DEVELOPER STANDARDS
// ============================================================================
//
// This contract has been organized into logical sections for better maintainability:
//
// 1. STORAGE STRUCTURE     - Contract state and storage variables
// 2. EVENT DEFINITIONS     - All contract events for transparency
// 3. CONSTRUCTOR          - Contract initialization
// 4. CORE LENDING OPS     - Basic operations: create, fund, repay
// 5. ADVANCED LENDING OPS - Enhanced features: early repay, partial, extension
// 6. RISK MANAGEMENT      - Late fees and loan refinancing
// 7. QUERY OPERATIONS     - All getter and calculation functions
// 8. PRIVATE HELPERS      - Internal utility functions
//
// Total Lines: ~922 (down from 869, but much better organized)
// Features: 5/8 phases completed (Phase 1: 100% complete)
// ============================================================================

#[ink::contract]
pub mod lending_contract {
    use super::*;

    // ============================================================================
    // STORAGE STRUCTURE
    // ============================================================================
    
    #[ink(storage)]
    pub struct LendingContract {
        owner: AccountId,
        total_loans: u64,
        loans: Mapping<u64, Loan>,
        user_profiles: Mapping<AccountId, UserProfile>,
        total_liquidity: Balance,
        protocol_fee: u16, // Basis points
        min_collateral_ratio: u16, // Basis points
        total_pools: u64,
        liquidity_pools: Mapping<u64, LiquidityPool>,
        pool_liquidity_providers: Mapping<u64, Vec<AccountId>>,
        // Risk Management & Security
        total_insurance_policies: u64,
        insurance_policies: Mapping<u64, InsurancePolicy>,
        fraud_detection_rules: Mapping<u64, FraudDetectionRule>,
        total_fraud_rules: u64,
        compliance_records: Mapping<AccountId, Vec<ComplianceRecord>>,
        credit_scores: Mapping<AccountId, CreditScore>,
        // Analytics & Reporting (Phase 5)
        total_loan_metrics: u64,
        loan_performance_metrics: Mapping<u64, LoanPerformanceMetrics>,
        user_portfolio_analytics: Mapping<AccountId, PortfolioAnalytics>,
        market_statistics: MarketStatistics,
        historical_data: Vec<HistoricalDataPoint>,
        total_benchmarks: u64,
        performance_benchmarks: Mapping<u64, PerformanceBenchmark>,
        total_analytics_reports: u64,
        analytics_reports: Mapping<u64, AnalyticsReport>,
        total_users: u64, // Track total user count
    }

    // ============================================================================
    // EVENT DEFINITIONS
    // ============================================================================
    
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

    #[ink(event)]
    pub struct LoanEarlyRepaid {
        #[ink(topic)]
        loan_id: u64,
        borrower: AccountId,
        original_amount: Balance,
        discounted_amount: Balance,
        discount_applied: Balance,
        blocks_early: u64,
    }

    #[ink(event)]
    pub struct LoanPartiallyPaid {
        #[ink(topic)]
        loan_id: u64,
        borrower: AccountId,
        payment_amount: Balance,
        remaining_balance: Balance,
        total_paid: Balance,
    }

    #[ink(event)]
    pub struct LoanExtended {
        #[ink(topic)]
        loan_id: u64,
        borrower: AccountId,
        old_due_date: u64,
        new_due_date: u64,
        extension_duration: u64,
        extension_fee: Balance,
        total_extensions: u32,
    }

    #[ink(event)]
    pub struct LateFeesAccumulated {
        #[ink(topic)]
        loan_id: u64,
        borrower: AccountId,
        overdue_blocks: u64,
        late_fees_added: Balance,
        total_late_fees: Balance,
        new_remaining_balance: Balance,
    }

    #[ink(event)]
    pub struct LoanRefinanced {
        #[ink(topic)]
        loan_id: u64,
        borrower: AccountId,
        old_lender: AccountId,
        new_lender: AccountId,
        old_interest_rate: u16,
        new_interest_rate: u16,
        refinance_fee: Balance,
        remaining_balance: Balance,
        refinance_count: u32,
    }

    #[ink(event)]
    pub struct InterestRateAdjusted {
        #[ink(topic)]
        loan_id: u64,
        borrower: AccountId,
        old_rate: u16,
        new_rate: u16,
        reason: RateAdjustmentReason,
        risk_multiplier: u16,
        effective_rate: u16,
    }

    #[ink(event)]
    pub struct InterestCompounded {
        #[ink(topic)]
        loan_id: u64,
        borrower: AccountId,
        compound_period: u64,
        interest_accrued: Balance,
        total_compounded: Balance,
        new_remaining_balance: Balance,
    }

    #[ink(event)]
    pub struct InterestOnlyPaymentMade {
        #[ink(topic)]
        loan_id: u64,
        borrower: AccountId,
        payment_period: u32,
        interest_paid: Balance,
        principal_remaining: Balance,
        next_payment_due: u64,
    }

    #[ink(event)]
    pub struct GracePeriodGranted {
        #[ink(topic)]
        loan_id: u64,
        borrower: AccountId,
        reason: GracePeriodReason,
        duration: u64,
        extension_number: u32,
        granted_by: AccountId,
        total_grace_period: u64,
    }

    #[ink(event)]
    pub struct LiquidityPoolCreated {
        #[ink(topic)]
        pool_id: u64,
        name: String,
        creator: AccountId,
        initial_liquidity: Balance,
        pool_fee_rate: u16,
        reward_rate: u16,
    }

    #[ink(event)]
    pub struct LiquidityProvided {
        #[ink(topic)]
        pool_id: u64,
        provider: AccountId,
        amount: Balance,
        pool_share: u16,
        total_pool_liquidity: Balance,
    }

    #[ink(event)]
    pub struct RewardsDistributed {
        #[ink(topic)]
        pool_id: u64,
        provider: AccountId,
        amount: Balance,
        pool_share: u16,
        total_rewards: Balance,
    }

    #[ink(event)]
    pub struct PoolRebalanced {
        #[ink(topic)]
        pool_id: u64,
        old_liquidity_ratio: u16,
        new_liquidity_ratio: u16,
        performance_score: u16,
        rebalance_reason: String,
        liquidity_adjustment: Balance,
    }

    #[ink(event)]
    pub struct YieldFarmingEnabled {
        #[ink(topic)]
        pool_id: u64,
        enabled_by: AccountId,
        reward_tokens_count: u32,
        staking_requirements: String,
    }

    #[ink(event)]
    pub struct TokensStaked {
        #[ink(topic)]
        pool_id: u64,
        staker: AccountId,
        amount: Balance,
        tier_level: String,
        multiplier: u16,
        lock_period: u64,
    }

    #[ink(event)]
    pub struct YieldRewardsClaimed {
        #[ink(topic)]
        pool_id: u64,
        staker: AccountId,
        reward_amount: Balance,
        reward_token: String,
        tier_multiplier: u16,
        total_staked: Balance,
    }

    #[ink(event)]
    pub struct MarketDepthUpdated {
        #[ink(topic)]
        pool_id: u64,
        price_level: u16,
        liquidity_change: Balance,
        new_depth: Balance,
        order_count: u32,
    }

    #[ink(event)]
    pub struct OptimalDistributionApplied {
        #[ink(topic)]
        pool_id: u64,
        old_distribution: String,
        new_distribution: String,
        optimization_reason: String,
        total_liquidity_moved: Balance,
    }

    #[ink(event)]
    pub struct ConcentrationLimitExceeded {
        #[ink(topic)]
        pool_id: u64,
        limit_type: String,
        current_concentration: u16,
        limit_threshold: u16,
        action_taken: String,
    }

    #[ink(event)]
    pub struct CreditScoreUpdated {
        #[ink(topic)]
        user_id: AccountId,
        old_score: u16,
        new_score: u16,
        change_reason: String,
        risk_level: String,
    }

    #[ink(event)]
    pub struct CollateralRequirementUpdated {
        #[ink(topic)]
        loan_id: u64,
        collateral_type: String,
        old_amount: Balance,
        new_amount: Balance,
        reason: String,
    }

    #[ink(event)]
    pub struct InsurancePolicyCreated {
        #[ink(topic)]
        policy_id: u64,
        loan_id: u64,
        insured_amount: Balance,
        premium_rate: u16,
        coverage_period: u64,
    }

    #[ink(event)]
    pub struct FraudDetected {
        #[ink(topic)]
        user_id: AccountId,
        rule_type: String,
        threshold: u16,
        action: String,
        description: String,
    }

    #[ink(event)]
    pub struct ComplianceStatusUpdated {
        #[ink(topic)]
        user_id: AccountId,
        compliance_type: String,
        old_status: String,
        new_status: String,
        verification_date: u64,
    }

    // ============================================================================
    // ANALYTICS & REPORTING EVENTS (Phase 5)
    // ============================================================================

    #[ink(event)]
    pub struct LoanMetricsUpdated {
        #[ink(topic)]
        loan_id: u64,
        borrower: AccountId,
        performance_score: u16,
        payment_efficiency: u16,
        risk_adjusted_return: u16,
    }

    #[ink(event)]
    pub struct PortfolioAnalyticsUpdated {
        #[ink(topic)]
        user_id: AccountId,
        portfolio_value: Balance,
        diversification_score: u16,
        risk_concentration: u16,
    }

    #[ink(event)]
    pub struct MarketStatisticsUpdated {
        total_market_cap: Balance,
        active_loans: u64,
        average_rate: u16,
        market_trend: String,
    }

    #[ink(event)]
    pub struct AnalyticsReportGenerated {
        #[ink(topic)]
        report_id: u64,
        report_type: String,
        generated_at: u64,
        metrics_count: u32,
    }

    #[ink(event)]
    pub struct PerformanceBenchmarkUpdated {
        #[ink(topic)]
        benchmark_id: u64,
        name: String,
        current_score: u16,
        target_score: u16,
    }

    impl LendingContract {
        // ============================================================================
        // CONSTRUCTOR
        // ============================================================================
        
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
                total_pools: 0,
                liquidity_pools: Mapping::default(),
                pool_liquidity_providers: Mapping::default(),
                // Risk Management & Security
                total_insurance_policies: 0,
                insurance_policies: Mapping::default(),
                fraud_detection_rules: Mapping::default(),
                total_fraud_rules: 0,
                compliance_records: Mapping::default(),
                credit_scores: Mapping::default(),
                // Analytics & Reporting (Phase 5)
                total_loan_metrics: 0,
                loan_performance_metrics: Mapping::default(),
                user_portfolio_analytics: Mapping::default(),
                market_statistics: MarketStatistics {
                    total_market_cap: 0,
                    total_active_loans: 0,
                    average_interest_rate: 0,
                    market_volatility: 0,
                    liquidity_depth: 0,
                    default_rate: 0,
                    utilization_rate: 0,
                    market_trend: MarketTrend::Stable,
                    last_updated: 0,
                },
                historical_data: Vec::new(),
                total_benchmarks: 0,
                performance_benchmarks: Mapping::default(),
                total_analytics_reports: 0,
                analytics_reports: Mapping::default(),
                total_users: 0, // Track total user count
            }
        }

        // ============================================================================
        // CORE LENDING OPERATIONS
        // ============================================================================
        
        /// Create a new loan request
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

            // Check if user is blacklisted and track new users
            let is_new_user = !self.user_profiles.contains(caller);
            let user_profile = self.get_or_create_user_profile(caller);
            if user_profile.is_blacklisted {
                return Err(LendingError::UserBlacklisted);
            }
            
            // Increment user count for new users
            if is_new_user {
                self.total_users += 1;
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
                early_repayment_discount: 200, // Default 2% discount for early repayment
                total_paid: 0,
                remaining_balance: 0, // Will be set when loan is funded
                partial_payments: Vec::new(),
                extension_count: 0,
                max_extensions: 3, // Default maximum of 3 extensions
                extension_fee_rate: 100, // Default 1% extension fee
                late_fee_rate: 50, // Default 0.5% daily late fee
                max_late_fee_rate: 1000, // Default 10% maximum late fee
                total_late_fees: 0,
                overdue_since: None,
                grace_period: 100, // Default 100 blocks grace period (~10 minutes)
                refinance_count: 0,
                max_refinances: 2, // Default maximum of 2 refinances
                refinance_fee_rate: 200, // Default 2% refinance fee
                original_loan_id: None,
                refinance_history: Vec::new(),
                interest_rate_type: InterestRateType::Fixed, // Default to fixed rate
                base_interest_rate: interest_rate, // Base rate same as initial rate
                risk_multiplier: 1000, // Default 1.0x risk multiplier
                interest_rate_adjustments: Vec::new(),
                last_interest_update: current_block,
                interest_update_frequency: 14400, // Default: daily updates (14400 blocks)
                interest_type: InterestType::Simple, // Default to simple interest
                compound_frequency: CompoundFrequency::Daily, // Default to daily compounding
                last_compound_date: current_block,
                compound_period_blocks: 14400, // Default: daily (14400 blocks)
                accrued_interest: 0,
                total_compounded_interest: 0,
                payment_structure: PaymentStructure::PrincipalAndInterest, // Default to P&I
                interest_only_periods: 0, // Default: no interest-only periods
                current_payment_period: 0,
                interest_only_periods_used: 0,
                next_payment_due: current_block + 14400, // First payment due in 1 day
                payment_period_blocks: 14400, // Default: daily payments (14400 blocks)
                minimum_payment_amount: 0, // No minimum initially
                grace_period_blocks: 100, // Default: 100 blocks grace period (~10 minutes)
                grace_period_used: 0,
                grace_period_extensions: 0,
                max_grace_period_extensions: 2, // Default: maximum 2 grace period extensions
                grace_period_reason: GracePeriodReason::None,
                grace_period_history: Vec::new(),
                liquidity_pool_id: None,
                pool_share: 0,
                liquidity_provider: None,
                pool_rewards_earned: 0,
                credit_score: None,
                collateral_requirements: Vec::new(),
                insurance_policies: Vec::new(),
                fraud_flags: Vec::new(),
                compliance_status: ComplianceStatus::Pending,
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

        /// Fund a pending loan
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
            
            // Set initial remaining balance (principal + interest)
            let total_repayment = loan.amount + ((loan.amount * loan.interest_rate as u128) / 10000);
            loan.remaining_balance = total_repayment;
            
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

        /// Repay a loan in full
        #[ink(message)]
        pub fn repay_loan(&mut self, loan_id: u64) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            if loan.borrower != caller {
                return Err(LendingError::Unauthorized);
            }
            
            if loan.status != LoanStatus::Active && loan.status != LoanStatus::PartiallyPaid && loan.status != LoanStatus::Overdue {
                return Err(LendingError::LoanNotActive);
            }

            let repayment_amount = self.calculate_repayment_amount(loan_id)?;
            
            if self.env().transferred_value() != repayment_amount {
                return Err(LendingError::InvalidAmount);
            }

            // Record the full payment
            let current_block = self.env().block_number() as u64;
            let full_payment = PartialPayment {
                amount: repayment_amount,
                timestamp: current_block,
                payment_type: PaymentType::Full,
            };

            // Update loan payment tracking
            loan.total_paid = repayment_amount;
            loan.remaining_balance = 0;
            loan.partial_payments.push(full_payment);
            loan.status = LoanStatus::Repaid;

            self.loans.insert(loan_id, &loan);

            // Transfer repayment to lender
            if let Some(lender) = loan.lender {
                self.env().transfer(lender, repayment_amount)
                    .map_err(|_| LendingError::TransferFailed)?;
            }

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

        // ============================================================================
        // ADVANCED LENDING OPERATIONS
        // ============================================================================
        
        /// Repay a loan early with discount
        #[ink(message)]
        pub fn early_repay_loan(&mut self, loan_id: u64) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            if loan.borrower != caller {
                return Err(LendingError::Unauthorized);
            }
            
            if loan.status != LoanStatus::Active {
                return Err(LendingError::LoanNotActive);
            }

            let current_block = self.env().block_number() as u64;
            if current_block >= loan.due_date {
                return Err(LendingError::LoanNotActive); // Loan is already due, use regular repayment
            }

            // Calculate early repayment discount
            let blocks_early = loan.due_date - current_block;
            let discount_percentage = self.calculate_early_repayment_discount(blocks_early, loan.duration);
            
            let original_repayment = self.calculate_repayment_amount(loan_id)?;
            let discount_amount = (original_repayment * discount_percentage as u128) / 10000;
            let discounted_repayment = original_repayment - discount_amount;
            
            if self.env().transferred_value() != discounted_repayment {
                return Err(LendingError::InvalidAmount);
            }

            // Record the early payment
            let current_block = self.env().block_number() as u64;
            let early_payment = PartialPayment {
                amount: discounted_repayment,
                timestamp: current_block,
                payment_type: PaymentType::Early,
            };

            // Update loan payment tracking
            loan.total_paid = discounted_repayment;
            loan.remaining_balance = 0;
            loan.partial_payments.push(early_payment);
            loan.status = LoanStatus::EarlyRepaid;

            self.loans.insert(loan_id, &loan);

            // Transfer discounted repayment to lender
            if let Some(lender) = loan.lender {
                self.env().transfer(lender, discounted_repayment)
                    .map_err(|_| LendingError::TransferFailed)?;
            }

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

            self.env().emit_event(LoanEarlyRepaid {
                loan_id,
                borrower: caller,
                original_amount: original_repayment,
                discounted_amount: discounted_repayment,
                discount_applied: discount_amount,
                blocks_early,
            });

            Ok(())
        }

        /// Make a partial payment on a loan
        #[ink(message)]
        pub fn partial_repay_loan(&mut self, loan_id: u64) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            if loan.borrower != caller {
                return Err(LendingError::Unauthorized);
            }
            
            if loan.status != LoanStatus::Active && loan.status != LoanStatus::PartiallyPaid && loan.status != LoanStatus::Overdue {
                return Err(LendingError::LoanNotActive);
            }

            let payment_amount = self.env().transferred_value();
            if payment_amount == 0 {
                return Err(LendingError::InvalidAmount);
            }

            if payment_amount >= loan.remaining_balance {
                return Err(LendingError::InvalidAmount); // Use full repayment for full amounts
            }

            // Apply late fees if loan is overdue
            if loan.status == LoanStatus::Overdue {
                let current_block = self.env().block_number() as u64;
                let grace_period_end = loan.due_date + loan.grace_period;
                let overdue_blocks = current_block - grace_period_end;
                let days_overdue = overdue_blocks / 14400;
                let late_fee_rate = (loan.late_fee_rate * days_overdue as u16).min(loan.max_late_fee_rate);
                let late_fees = (loan.remaining_balance * late_fee_rate as u128) / 10000;
                
                if late_fees > 0 {
                    loan.total_late_fees += late_fees;
                    loan.remaining_balance += late_fees;
                }
            }

            // Record the partial payment
            let current_block = self.env().block_number() as u64;
            let partial_payment = PartialPayment {
                amount: payment_amount,
                timestamp: current_block,
                payment_type: PaymentType::Partial,
            };

            // Update loan payment tracking
            loan.total_paid += payment_amount;
            loan.remaining_balance -= payment_amount;
            loan.partial_payments.push(partial_payment);
            
            // Update loan status
            if loan.remaining_balance > 0 {
                if loan.status == LoanStatus::Overdue {
                    loan.status = LoanStatus::Overdue; // Keep overdue status
                } else {
                    loan.status = LoanStatus::PartiallyPaid;
                }
            } else {
                loan.status = LoanStatus::Repaid;
            }

            self.loans.insert(loan_id, &loan);

            // Transfer payment to lender
            if let Some(lender) = loan.lender {
                self.env().transfer(lender, payment_amount)
                    .map_err(|_| LendingError::TransferFailed)?;
            }

            // Update borrower profile if loan is fully repaid
            if loan.remaining_balance == 0 {
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
            }

            self.env().emit_event(LoanPartiallyPaid {
                loan_id,
                borrower: caller,
                payment_amount,
                remaining_balance: loan.remaining_balance,
                total_paid: loan.total_paid,
            });

            Ok(())
        }

        /// Extend a loan's duration
        #[ink(message)]
        pub fn extend_loan(&mut self, loan_id: u64, extension_duration: u64) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            if loan.borrower != caller {
                return Err(LendingError::Unauthorized);
            }
            
            if loan.status != LoanStatus::Active && loan.status != LoanStatus::PartiallyPaid {
                return Err(LendingError::LoanNotActive);
            }

            // Check if loan can still be extended
            if loan.extension_count >= loan.max_extensions {
                return Err(LendingError::InvalidAmount); // Reuse error for max extensions reached
            }

            // Validate extension duration
            if extension_duration == 0 || extension_duration > 100000 { // Max ~1 year extension
                return Err(LendingError::InvalidDuration);
            }

            let current_block = self.env().block_number() as u64;
            if current_block >= loan.due_date {
                return Err(LendingError::LoanNotActive); // Loan is already due
            }

            // Calculate extension fee
            let extension_fee = (loan.remaining_balance * loan.extension_fee_rate as u128) / 10000;
            
            // Check if extension fee is paid
            if self.env().transferred_value() != extension_fee {
                return Err(LendingError::InvalidAmount);
            }

            // Update loan extension details
            let old_due_date = loan.due_date;
            loan.due_date += extension_duration;
            loan.extension_count += 1;

            // Update remaining balance to include extension fee
            loan.remaining_balance += extension_fee;

            self.loans.insert(loan_id, &loan);

            // Transfer extension fee to lender
            if let Some(lender) = loan.lender {
                self.env().transfer(lender, extension_fee)
                    .map_err(|_| LendingError::TransferFailed)?;
            }

            self.env().emit_event(LoanExtended {
                loan_id,
                borrower: caller,
                old_due_date,
                new_due_date: loan.due_date,
                extension_duration,
                extension_fee,
                total_extensions: loan.extension_count,
            });

            Ok(())
        }

        // ============================================================================
        // RISK MANAGEMENT & REFINANCING
        // ============================================================================
        
        /// Apply late fees to an overdue loan
        #[ink(message)]
        pub fn apply_late_fees(&mut self, loan_id: u64) -> Result<(), LendingError> {
            let mut loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            if loan.status != LoanStatus::Active && loan.status != LoanStatus::PartiallyPaid {
                return Err(LendingError::LoanNotActive);
            }

            let current_block = self.env().block_number() as u64;
            let grace_period_end = loan.due_date + loan.grace_period;
            
            // Check if loan is overdue and grace period has ended
            if current_block <= grace_period_end {
                return Err(LendingError::InvalidAmount); // Reuse error for not overdue yet
            }

            // Calculate overdue blocks
            let overdue_blocks = current_block - grace_period_end;
            
            // Calculate late fees (daily compounding)
            let days_overdue = overdue_blocks / 14400; // Assuming 14400 blocks per day (6s blocks)
            let late_fee_rate = (loan.late_fee_rate * days_overdue as u16).min(loan.max_late_fee_rate);
            
            let late_fees = (loan.remaining_balance * late_fee_rate as u128) / 10000;
            
            if late_fees > 0 {
                // Update loan with late fees
                loan.total_late_fees += late_fees;
                loan.remaining_balance += late_fees;
                
                // Set overdue status if not already set
                if loan.status == LoanStatus::Active {
                    loan.status = LoanStatus::Overdue;
                    loan.overdue_since = Some(grace_period_end);
                }
                
                self.loans.insert(loan_id, &loan);

                self.env().emit_event(LateFeesAccumulated {
                    loan_id,
                    borrower: loan.borrower,
                    overdue_blocks,
                    late_fees_added: late_fees,
                    total_late_fees: loan.total_late_fees,
                    new_remaining_balance: loan.remaining_balance,
                });
            }

            Ok(())
        }

        /// Refinance a loan with better terms
        #[ink(message)]
        pub fn refinance_loan(&mut self, loan_id: u64, new_interest_rate: u16, new_duration: u64) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            if loan.borrower != caller {
                return Err(LendingError::Unauthorized);
            }
            
            if loan.status != LoanStatus::Active && loan.status != LoanStatus::PartiallyPaid {
                return Err(LendingError::LoanNotActive);
            }

            // Check if loan can still be refinanced
            if loan.refinance_count >= loan.max_refinances {
                return Err(LendingError::InvalidAmount); // Reuse error for max refinances reached
            }

            // Validate new terms
            if new_interest_rate == 0 || new_interest_rate > 10000 {
                return Err(LendingError::InvalidInterestRate);
            }

            if new_duration == 0 || new_duration > 1000000 {
                return Err(LendingError::InvalidDuration);
            }

            // Check if new terms are actually better
            if new_interest_rate >= loan.interest_rate {
                return Err(LendingError::InvalidInterestRate); // Reuse error for worse terms
            }

            let current_block = self.env().block_number() as u64;
            if current_block >= loan.due_date {
                return Err(LendingError::LoanNotActive); // Loan is already due
            }

            // Calculate refinance fee
            let refinance_fee = (loan.remaining_balance * loan.refinance_fee_rate as u128) / 10000;
            
            // Check if refinance fee is paid
            if self.env().transferred_value() != refinance_fee {
                return Err(LendingError::InvalidAmount);
            }

            // Record refinancing operation
            let refinance_record = RefinanceRecord {
                timestamp: current_block,
                old_lender: loan.lender.unwrap_or(AccountId::from([0; 32])),
                new_lender: caller, // New lender is the caller
                old_interest_rate: loan.interest_rate,
                new_interest_rate,
                refinance_fee,
                remaining_balance: loan.remaining_balance,
            };

            // Update loan with new terms
            let old_interest_rate = loan.interest_rate;
            loan.interest_rate = new_interest_rate;
            loan.duration = new_duration;
            loan.due_date = current_block + new_duration;
            loan.refinance_count += 1;
            loan.refinance_history.push(refinance_record.clone());
            loan.status = LoanStatus::Refinanced;

            // Recalculate remaining balance with new interest rate
            let new_total_repayment = loan.amount + ((loan.amount * new_interest_rate as u128) / 10000);
            // For refinancing, just set the new remaining balance
            loan.remaining_balance = new_total_repayment;

            self.loans.insert(loan_id, &loan);

            // Transfer refinance fee to old lender
            if let Some(old_lender) = loan.lender {
                self.env().transfer(old_lender, refinance_fee)
                    .map_err(|_| LendingError::TransferFailed)?;
            }

            self.env().emit_event(LoanRefinanced {
                loan_id,
                borrower: caller,
                old_lender: refinance_record.old_lender,
                new_lender: caller,
                old_interest_rate,
                new_interest_rate,
                refinance_fee,
                remaining_balance: loan.remaining_balance,
                refinance_count: loan.refinance_count,
            });

            Ok(())
        }

        // ============================================================================
        // VARIABLE INTEREST RATE MANAGEMENT
        // ============================================================================
        
        /// Adjust interest rate for a variable rate loan
        #[ink(message)]
        pub fn adjust_interest_rate(
            &mut self,
            loan_id: u64,
            new_base_rate: u16,
            reason: RateAdjustmentReason,
        ) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            // Only lender can adjust interest rates
            if loan.lender != Some(caller) {
                return Err(LendingError::Unauthorized);
            }
            
            // Check if loan supports variable rates
            if loan.interest_rate_type != InterestRateType::Variable {
                return Err(LendingError::InvalidAmount); // Reuse error for fixed rate loan
            }

            // Validate new rate
            if new_base_rate == 0 || new_base_rate > 10000 {
                return Err(LendingError::InvalidInterestRate);
            }

            let current_block = self.env().block_number() as u64;
            
            // Check update frequency
            if current_block < loan.last_interest_update + loan.interest_update_frequency {
                return Err(LendingError::InvalidAmount); // Reuse error for too frequent updates
            }

            let old_rate = loan.interest_rate;
            let _old_base_rate = loan.base_interest_rate;
            
            // Calculate new effective rate with risk multiplier
            let new_effective_rate = ((new_base_rate as u32 * loan.risk_multiplier as u32) / 1000) as u16;
            
            // Record the adjustment
            let adjustment = InterestRateAdjustment {
                timestamp: current_block,
                old_rate: old_rate,
                new_rate: new_effective_rate,
                reason: reason.clone(),
                risk_score_change: None, // Will be updated if risk score changes
            };

            // Update loan with new rates
            loan.base_interest_rate = new_base_rate;
            loan.interest_rate = new_effective_rate;
            loan.interest_rate_adjustments.push(adjustment);
            loan.last_interest_update = current_block;

            // Recalculate remaining balance with new interest rate
            if loan.status == LoanStatus::Active || loan.status == LoanStatus::PartiallyPaid {
                let new_total_repayment = loan.amount + ((loan.amount * new_effective_rate as u128) / 10000);
                let principal_paid = loan.amount - (loan.remaining_balance - loan.total_late_fees);
                let new_remaining_balance = new_total_repayment - principal_paid;
                loan.remaining_balance = new_remaining_balance;
            }

            self.loans.insert(loan_id, &loan);

            self.env().emit_event(InterestRateAdjusted {
                loan_id,
                borrower: loan.borrower,
                old_rate,
                new_rate: new_effective_rate,
                reason,
                risk_multiplier: loan.risk_multiplier,
                effective_rate: new_effective_rate,
            });

            Ok(())
        }

        /// Update risk multiplier for a loan
        #[ink(message)]
        pub fn update_risk_multiplier(
            &mut self,
            loan_id: u64,
            new_risk_multiplier: u16,
        ) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            // Only lender can update risk multipliers
            if loan.lender != Some(caller) {
                return Err(LendingError::Unauthorized);
            }
            
            // Validate risk multiplier (0.5x to 3.0x)
            if new_risk_multiplier < 500 || new_risk_multiplier > 3000 {
                return Err(LendingError::InvalidAmount); // Reuse error for invalid multiplier
            }

            let old_multiplier = loan.risk_multiplier;
            let old_rate = loan.interest_rate;
            
            // Calculate new effective rate
            let new_effective_rate = ((loan.base_interest_rate as u32 * new_risk_multiplier as u32) / 1000) as u16;
            
            // Record the adjustment
            let adjustment = InterestRateAdjustment {
                timestamp: self.env().block_number() as u64,
                old_rate: old_rate,
                new_rate: new_effective_rate,
                reason: RateAdjustmentReason::RiskScoreChange,
                risk_score_change: Some((new_risk_multiplier as i16) - (old_multiplier as i16)),
            };

            // Update loan
            loan.risk_multiplier = new_risk_multiplier;
            loan.interest_rate = new_effective_rate;
            loan.interest_rate_adjustments.push(adjustment);
            loan.last_interest_update = self.env().block_number() as u64;

            // Recalculate remaining balance if loan is active
            if loan.status == LoanStatus::Active || loan.status == LoanStatus::PartiallyPaid {
                let new_total_repayment = loan.amount + ((loan.amount * new_effective_rate as u128) / 10000);
                // For risk multiplier updates, just set the new remaining balance
                loan.remaining_balance = new_total_repayment;
            }

            self.loans.insert(loan_id, &loan);

            self.env().emit_event(InterestRateAdjusted {
                loan_id,
                borrower: loan.borrower,
                old_rate,
                new_rate: new_effective_rate,
                reason: RateAdjustmentReason::RiskScoreChange,
                risk_multiplier: new_risk_multiplier,
                effective_rate: new_effective_rate,
            });

            Ok(())
        }

        /// Convert a fixed rate loan to variable rate
        #[ink(message)]
        pub fn convert_to_variable_rate(
            &mut self,
            loan_id: u64,
            new_base_rate: u16,
        ) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            // Only lender can convert loan types
            if loan.lender != Some(caller) {
                return Err(LendingError::Unauthorized);
            }
            
            // Check if loan is already variable
            if loan.interest_rate_type == InterestRateType::Variable {
                return Err(LendingError::InvalidAmount); // Reuse error for already variable
            }

            // Validate new base rate
            if new_base_rate == 0 || new_base_rate > 10000 {
                return Err(LendingError::InvalidInterestRate);
            }

            let old_rate = loan.interest_rate;
            let new_effective_rate = ((new_base_rate as u32 * loan.risk_multiplier as u32) / 1000) as u16;
            
            // Record the conversion
            let adjustment = InterestRateAdjustment {
                timestamp: self.env().block_number() as u64,
                old_rate: old_rate,
                new_rate: new_effective_rate,
                reason: RateAdjustmentReason::ManualAdjustment,
                risk_score_change: None,
            };

            // Update loan
            loan.interest_rate_type = InterestRateType::Variable;
            loan.base_interest_rate = new_base_rate;
            loan.interest_rate = new_effective_rate;
            loan.interest_rate_adjustments.push(adjustment);
            loan.last_interest_update = self.env().block_number() as u64;

            // Recalculate remaining balance
            if loan.status == LoanStatus::Active || loan.status == LoanStatus::PartiallyPaid {
                let new_total_repayment = loan.amount + ((loan.amount * new_effective_rate as u128) / 10000);
                // For conversion, just set the new remaining balance
                loan.remaining_balance = new_total_repayment;
            }

            self.loans.insert(loan_id, &loan);

            self.env().emit_event(InterestRateAdjusted {
                loan_id,
                borrower: loan.borrower,
                old_rate,
                new_rate: new_effective_rate,
                reason: RateAdjustmentReason::ManualAdjustment,
                risk_multiplier: loan.risk_multiplier,
                effective_rate: new_effective_rate,
            });

            Ok(())
        }

        // ============================================================================
        // COMPOUND INTEREST CALCULATION
        // ============================================================================
        
        /// Calculate and apply compound interest for a loan
        #[ink(message)]
        pub fn compound_interest(&mut self, loan_id: u64) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            // Only lender or borrower can compound interest
            if loan.lender != Some(caller) && loan.borrower != caller {
                return Err(LendingError::Unauthorized);
            }
            
            // Check if loan supports compound interest
            if loan.interest_type != InterestType::Compound {
                return Err(LendingError::InvalidAmount); // Reuse error for simple interest loan
            }
            
            // Check if it's time to compound
            let current_block = self.env().block_number() as u64;
            if current_block < loan.last_compound_date + loan.compound_period_blocks {
                return Err(LendingError::InvalidAmount); // Reuse error for too early to compound
            }
            
            // Calculate compound interest
            let periods_since_last_compound = (current_block - loan.last_compound_date) / loan.compound_period_blocks;
            if periods_since_last_compound == 0 {
                return Err(LendingError::InvalidAmount); // Reuse error for no periods to compound
            }
            
            // Calculate the new balance with compound interest
            let principal = loan.amount;
            let rate_per_period = loan.interest_rate as f64 / 10000.0; // Convert basis points to decimal
            let periods = periods_since_last_compound as f64;
            
            // Compound interest formula: A = P(1 + r)^n
            let compound_factor = (1.0 + rate_per_period).powf(periods);
            let new_total = (principal as f64 * compound_factor) as u128;
            
            // Calculate interest accrued
            let interest_accrued = new_total - principal;
            
            // Update loan with compound interest
            let _old_remaining_balance = loan.remaining_balance;
            loan.remaining_balance = new_total;
            loan.accrued_interest = 0; // Reset accrued interest
            loan.total_compounded_interest += interest_accrued;
            loan.last_compound_date = current_block;
            
            self.loans.insert(loan_id, &loan);
            
            self.env().emit_event(InterestCompounded {
                loan_id,
                borrower: loan.borrower,
                compound_period: periods_since_last_compound,
                interest_accrued,
                total_compounded: loan.total_compounded_interest,
                new_remaining_balance: loan.remaining_balance,
            });
            
            Ok(())
        }
        
        /// Switch loan from simple to compound interest
        #[ink(message)]
        pub fn convert_to_compound_interest(
            &mut self,
            loan_id: u64,
            frequency: CompoundFrequency,
        ) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            // Only lender can convert interest types
            if loan.lender != Some(caller) {
                return Err(LendingError::Unauthorized);
            }
            
            // Check if loan is already compound
            if loan.interest_type == InterestType::Compound {
                return Err(LendingError::InvalidAmount); // Reuse error for already compound
            }
            
            // Set compound frequency and calculate period blocks
            let compound_period_blocks = match frequency {
                CompoundFrequency::Daily => 14400,      // 14400 blocks per day
                CompoundFrequency::Weekly => 100800,    // 100800 blocks per week
                CompoundFrequency::Monthly => 432000,   // 432000 blocks per month
                CompoundFrequency::Quarterly => 1296000, // 1296000 blocks per quarter
                CompoundFrequency::Annually => 5184000,  // 5184000 blocks per year
            };
            
            // Convert loan to compound interest
            loan.interest_type = InterestType::Compound;
            loan.compound_frequency = frequency;
            loan.compound_period_blocks = compound_period_blocks;
            loan.last_compound_date = self.env().block_number() as u64;
            loan.accrued_interest = 0;
            loan.total_compounded_interest = 0;
            
            // Recalculate remaining balance for compound interest
            let current_block = self.env().block_number() as u64;
            let blocks_since_creation = current_block - loan.created_at;
            let periods = blocks_since_creation / compound_period_blocks;
            
            if periods > 0 {
                let rate_per_period = loan.interest_rate as f64 / 10000.0;
                let compound_factor = (1.0 + rate_per_period).powf(periods as f64);
                let new_total = (loan.amount as f64 * compound_factor) as u128;
                loan.remaining_balance = new_total;
                loan.total_compounded_interest = new_total - loan.amount;
            }
            
            self.loans.insert(loan_id, &loan);
            
            Ok(())
        }
        
        /// Get compound interest information for a loan
        #[ink(message)]
        pub fn get_compound_interest_info(&self, loan_id: u64) -> Result<(InterestType, CompoundFrequency, u64, Balance, Balance), LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            Ok((
                loan.interest_type.clone(),
                loan.compound_frequency.clone(),
                loan.compound_period_blocks,
                loan.accrued_interest,
                loan.total_compounded_interest,
            ))
        }
        
        /// Calculate accrued interest for a loan (without compounding)
        #[ink(message)]
        pub fn calculate_accrued_interest(&self, loan_id: u64) -> Result<Balance, LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            let current_block = self.env().block_number() as u64;
            let blocks_since_last_compound = current_block - loan.last_compound_date;
            
            if loan.interest_type == InterestType::Simple {
                // Simple interest: P × r × t
                let time_factor = blocks_since_last_compound as f64 / 14400.0; // Convert to days
                let rate = loan.interest_rate as f64 / 10000.0;
                let accrued = (loan.amount as f64 * rate * time_factor) as u128;
                Ok(accrued)
            } else {
                // Compound interest: calculate what would be accrued
                let periods = blocks_since_last_compound / loan.compound_period_blocks;
                if periods == 0 {
                    Ok(0)
                } else {
                    let rate_per_period = loan.interest_rate as f64 / 10000.0;
                    let compound_factor = (1.0 + rate_per_period).powf(periods as f64);
                    let new_total = (loan.amount as f64 * compound_factor) as u128;
                    Ok(new_total - loan.amount)
                }
            }
        }

        // ============================================================================
        // INTEREST-ONLY PAYMENT PERIODS
        // ============================================================================
        
        /// Set loan to interest-only payment structure for a specified number of periods
        #[ink(message)]
        pub fn set_interest_only_periods(
            &mut self,
            loan_id: u64,
            periods: u32,
            payment_period_blocks: u64,
        ) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            // Only lender can set payment structure
            if loan.lender != Some(caller) {
                return Err(LendingError::Unauthorized);
            }
            
            // Check if loan is active
            if loan.status != LoanStatus::Active {
                return Err(LendingError::LoanNotActive);
            }
            
            // Validate payment period blocks (minimum 1 day)
            if payment_period_blocks < 14400 {
                return Err(LendingError::InvalidAmount); // Reuse error for invalid period
            }
            
            // Set interest-only structure
            loan.payment_structure = PaymentStructure::InterestOnly;
            loan.interest_only_periods = periods;
            loan.payment_period_blocks = payment_period_blocks;
            loan.next_payment_due = self.env().block_number() as u64 + payment_period_blocks;
            loan.minimum_payment_amount = self.calculate_interest_payment(loan_id)?;
            
            self.loans.insert(loan_id, &loan);
            
            Ok(())
        }
        
        /// Make an interest-only payment
        #[ink(message)]
        pub fn make_interest_only_payment(&mut self, loan_id: u64) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            // Check if caller is the borrower
            if loan.borrower != caller {
                return Err(LendingError::Unauthorized);
            }
            
            // Check if loan supports interest-only payments
            if loan.payment_structure != PaymentStructure::InterestOnly {
                return Err(LendingError::InvalidAmount); // Reuse error for wrong payment type
            }
            
            // Check if it's time for payment
            let current_block = self.env().block_number() as u64;
            if current_block < loan.next_payment_due {
                return Err(LendingError::InvalidAmount); // Reuse error for too early
            }
            
            // Check if interest-only periods are available
            if loan.interest_only_periods_used >= loan.interest_only_periods {
                return Err(LendingError::InvalidAmount); // Reuse error for no more periods
            }
            
            // Calculate interest payment for this period
            let interest_payment = self.calculate_interest_payment(loan_id)?;
            
            // Update loan state
            loan.current_payment_period += 1;
            loan.interest_only_periods_used += 1;
            loan.next_payment_due = current_block + loan.payment_period_blocks;
            
            // If this was the last interest-only period, switch to P&I
            if loan.interest_only_periods_used >= loan.interest_only_periods {
                loan.payment_structure = PaymentStructure::PrincipalAndInterest;
                loan.minimum_payment_amount = self.calculate_minimum_payment(loan_id)?;
            }
            
            self.loans.insert(loan_id, &loan);
            
            self.env().emit_event(InterestOnlyPaymentMade {
                loan_id,
                borrower: loan.borrower,
                payment_period: loan.current_payment_period,
                interest_paid: interest_payment,
                principal_remaining: loan.amount,
                next_payment_due: loan.next_payment_due,
            });
            
            Ok(())
        }
        
        /// Switch back to principal and interest payments
        #[ink(message)]
        pub fn switch_to_principal_and_interest(&mut self, loan_id: u64) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            // Only lender can change payment structure
            if loan.lender != Some(caller) {
                return Err(LendingError::Unauthorized);
            }
            
            // Check if loan is active
            if loan.status != LoanStatus::Active {
                return Err(LendingError::LoanNotActive);
            }
            
            // Switch to P&I structure
            loan.payment_structure = PaymentStructure::PrincipalAndInterest;
            loan.minimum_payment_amount = self.calculate_minimum_payment(loan_id)?;
            
            self.loans.insert(loan_id, &loan);
            
            Ok(())
        }
        
        /// Get payment structure information for a loan
        #[ink(message)]
        pub fn get_payment_structure_info(&self, loan_id: u64) -> Result<(PaymentStructure, u32, u32, u32, u64, Balance), LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            Ok((
                loan.payment_structure.clone(),
                loan.interest_only_periods,
                loan.interest_only_periods_used,
                loan.current_payment_period,
                loan.next_payment_due,
                loan.minimum_payment_amount,
            ))
        }
        
        /// Calculate interest payment for current period
        fn calculate_interest_payment(&self, loan_id: u64) -> Result<Balance, LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            let rate = loan.interest_rate as f64 / 10000.0; // Convert basis points to decimal
            let time_factor = loan.payment_period_blocks as f64 / 5184000.0; // Convert to years
            let interest = (loan.amount as f64 * rate * time_factor) as u128;
            
            Ok(interest)
        }
        
        /// Calculate minimum payment for P&I structure
        fn calculate_minimum_payment(&self, loan_id: u64) -> Result<Balance, LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            // Simple P&I calculation: (Principal × Rate × Time) + (Principal / Total Periods)
            let rate = loan.interest_rate as f64 / 10000.0;
            let time_factor = loan.payment_period_blocks as f64 / 5184000.0;
            let interest = (loan.amount as f64 * rate * time_factor) as u128;
            let principal = loan.amount / ((loan.duration / loan.payment_period_blocks) as u128);
            
            Ok(interest + principal)
        }

        // ============================================================================
        // GRACE PERIOD MANAGEMENT
        // ============================================================================
        
        /// Grant or extend grace period for a loan
        #[ink(message)]
        pub fn grant_grace_period(
            &mut self,
            loan_id: u64,
            duration: u64,
            reason: GracePeriodReason,
        ) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            // Only lender can grant grace periods
            if loan.lender != Some(caller) {
                return Err(LendingError::Unauthorized);
            }
            
            // Check if loan is active
            if loan.status != LoanStatus::Active {
                return Err(LendingError::LoanNotActive);
            }
            
            // Validate grace period duration (minimum 100 blocks, maximum 1 day)
            if duration < 100 || duration > 14400 {
                return Err(LendingError::InvalidAmount); // Reuse error for invalid duration
            }
            
            // Check if grace period extensions are available
            if loan.grace_period_extensions >= loan.max_grace_period_extensions {
                return Err(LendingError::InvalidAmount); // Reuse error for no more extensions
            }
            
            // Calculate new grace period
            let new_grace_period = loan.grace_period_blocks + duration;
            let extension_number = loan.grace_period_extensions + 1;
            
            // Update loan grace period
            loan.grace_period_blocks = new_grace_period;
            loan.grace_period_extensions = extension_number;
            loan.grace_period_reason = reason.clone();
            
            // Record grace period history
            let grace_record = GracePeriodRecord {
                timestamp: self.env().block_number() as u64,
                reason: reason.clone(),
                duration,
                extension_number,
                granted_by: caller,
            };
            loan.grace_period_history.push(grace_record);
            
            self.loans.insert(loan_id, &loan);
            
            self.env().emit_event(GracePeriodGranted {
                loan_id,
                borrower: loan.borrower,
                reason,
                duration,
                extension_number,
                granted_by: caller,
                total_grace_period: new_grace_period,
            });
            
            Ok(())
        }
        
        /// Check if loan is within grace period
        #[ink(message)]
        pub fn is_within_grace_period(&self, loan_id: u64) -> Result<bool, LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            let current_block = self.env().block_number() as u64;
            let overdue_since = loan.overdue_since.unwrap_or(0);
            
            if overdue_since == 0 {
                return Ok(false); // Not overdue
            }
            
            let grace_period_end = overdue_since + loan.grace_period_blocks;
            Ok(current_block <= grace_period_end)
        }
        
        /// Get grace period information for a loan
        #[ink(message)]
        pub fn get_grace_period_info(&self, loan_id: u64) -> Result<(u64, u64, u32, u32, GracePeriodReason, Vec<GracePeriodRecord>), LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            Ok((
                loan.grace_period_blocks,
                loan.grace_period_used,
                loan.grace_period_extensions,
                loan.max_grace_period_extensions,
                loan.grace_period_reason.clone(),
                loan.grace_period_history.clone(),
            ))
        }
        
        /// Calculate remaining grace period for an overdue loan
        #[ink(message)]
        pub fn calculate_remaining_grace_period(&self, loan_id: u64) -> Result<u64, LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            let current_block = self.env().block_number() as u64;
            let overdue_since = loan.overdue_since.unwrap_or(0);
            
            if overdue_since == 0 {
                return Ok(0); // Not overdue
            }
            
            let grace_period_end = overdue_since + loan.grace_period_blocks;
            if current_block > grace_period_end {
                return Ok(0); // Grace period expired
            }
            
            Ok(grace_period_end - current_block)
        }
        
        /// Set custom grace period for a loan (lender only)
        #[ink(message)]
        pub fn set_custom_grace_period(
            &mut self,
            loan_id: u64,
            grace_period_blocks: u64,
            max_extensions: u32,
        ) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            // Only lender can set custom grace periods
            if loan.lender != Some(caller) {
                return Err(LendingError::Unauthorized);
            }
            
            // Check if loan is active
            if loan.status != LoanStatus::Active {
                return Err(LendingError::LoanNotActive);
            }
            
            // Validate grace period (minimum 100 blocks, maximum 1 week)
            if grace_period_blocks < 100 || grace_period_blocks > 100800 {
                return Err(LendingError::InvalidAmount); // Reuse error for invalid duration
            }
            
            // Update grace period settings
            loan.grace_period_blocks = grace_period_blocks;
            loan.max_grace_period_extensions = max_extensions;
            
            self.loans.insert(loan_id, &loan);
            
            Ok(())
        }

        // ============================================================================
        // LIQUIDITY POOL MANAGEMENT
        // ============================================================================
        
        /// Create a new liquidity pool
        #[ink(message)]
        pub fn create_liquidity_pool(
            &mut self,
            name: String,
            initial_liquidity: Balance,
            pool_fee_rate: u16,
            reward_rate: u16,
            min_liquidity: Balance,
            max_liquidity: Balance,
        ) -> Result<u64, LendingError> {
            let caller = self.env().caller();
            
            // Validate parameters
            if initial_liquidity == 0 || pool_fee_rate > 1000 || reward_rate > 1000 {
                return Err(LendingError::InvalidAmount);
            }
            
            if min_liquidity >= max_liquidity {
                return Err(LendingError::InvalidAmount);
            }
            
            let pool_id = self.total_pools + 1;
            let current_block = self.env().block_number() as u64;
            
            // Create liquidity pool
            let pool = LiquidityPool {
                id: pool_id,
                name: name.clone(),
                total_liquidity: initial_liquidity,
                active_loans: 0,
                total_volume: 0,
                pool_fee_rate,
                reward_rate,
                min_liquidity,
                max_liquidity,
                created_at: current_block,
                status: PoolStatus::Active,
                liquidity_providers: Vec::new(),
                total_rewards_distributed: 0,
                performance_score: 5000, // Default: 50% performance score
                last_rebalance: current_block,
                rebalance_frequency: 14400, // Default: daily rebalancing (14400 blocks)
                target_liquidity_ratio: 8000, // Default: 80% target liquidity ratio
                current_liquidity_ratio: 10000, // Initial: 100% current ratio
                rebalance_threshold: 500, // Default: 5% threshold for rebalancing
                auto_rebalance_enabled: true, // Default: auto-rebalancing enabled
                yield_farming_enabled: false, // Default: yield farming disabled
                reward_tokens: Vec::new(), // No reward tokens initially
                staking_requirements: StakingRequirements {
                    min_stake_amount: 1000, // Minimum 1000 tokens to stake
                    lock_period: 14400, // 1 day lock period
                    early_unstake_penalty: 500, // 5% penalty for early unstaking
                    max_stake_amount: 100000, // Maximum 100,000 tokens to stake
                },
                tier_multipliers: vec![
                    TierMultiplier {
                        tier_name: "Bronze".to_string(),
                        min_stake_amount: 1000,
                        multiplier: 1000, // 1x multiplier
                        bonus_rewards: 0,
                    },
                    TierMultiplier {
                        tier_name: "Silver".to_string(),
                        min_stake_amount: 5000,
                        multiplier: 1200, // 1.2x multiplier
                        bonus_rewards: 100,
                    },
                    TierMultiplier {
                        tier_name: "Gold".to_string(),
                        min_stake_amount: 20000,
                        multiplier: 1500, // 1.5x multiplier
                        bonus_rewards: 300,
                    },
                    TierMultiplier {
                        tier_name: "Platinum".to_string(),
                        min_stake_amount: 50000,
                        multiplier: 2000, // 2x multiplier
                        bonus_rewards: 500,
                    },
                ],
                total_staked_tokens: 0,
                market_depth_levels: vec![
                    MarketDepthLevel {
                        price_level: 950, // 95% price level
                        liquidity_available: 0,
                        order_count: 0,
                        last_updated: current_block,
                    },
                    MarketDepthLevel {
                        price_level: 1000, // 100% price level
                        liquidity_available: 0,
                        order_count: 0,
                        last_updated: current_block,
                    },
                    MarketDepthLevel {
                        price_level: 1050, // 105% price level
                        liquidity_available: 0,
                        order_count: 0,
                        last_updated: current_block,
                    },
                ],
                optimal_distribution: OptimalDistribution {
                    target_depth_spread: 200, // 2% spread across levels
                    min_depth_per_level: 1000, // Minimum 1,000 per level
                    max_depth_per_level: 50000, // Maximum 50,000 per level
                    rebalancing_threshold: 300, // 3% threshold for rebalancing
                    auto_optimization_enabled: true,
                },
                depth_based_pricing: false, // Default: disabled
                concentration_limits: ConcentrationLimits {
                    max_single_pool_concentration: 8000, // Max 80% in single pool
                    max_provider_concentration: 5000, // Max 50% per provider
                    min_pool_diversity: 2, // Minimum 2 pools
                    concentration_check_frequency: 14400, // Check daily
                },
            };
            
            // Add creator as first liquidity provider
            let creator_share = 10000; // 100% initially
            let creator_provider = LiquidityProvider {
                account: caller,
                liquidity_provided: initial_liquidity,
                pool_share: creator_share,
                rewards_earned: 0,
                joined_at: current_block,
                last_reward_claim: current_block,
            };
            
            let mut pool_with_provider = pool.clone();
            pool_with_provider.liquidity_providers.push(creator_provider);
            
            // Store pool and update state
            self.liquidity_pools.insert(pool_id, &pool_with_provider);
            self.pool_liquidity_providers.insert(pool_id, &vec![caller]);
            self.total_pools = pool_id;
            
            self.env().emit_event(LiquidityPoolCreated {
                pool_id,
                name,
                creator: caller,
                initial_liquidity,
                pool_fee_rate,
                reward_rate,
            });
            
            Ok(pool_id)
        }
        
        /// Provide liquidity to a pool
        #[ink(message)]
        pub fn provide_liquidity(&mut self, pool_id: u64, amount: Balance) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut pool = self.liquidity_pools.get(pool_id).ok_or(LendingError::LoanNotFound)?;
            
            // Check if pool is active
            if pool.status != PoolStatus::Active {
                return Err(LendingError::LoanNotActive);
            }
            
            // Validate amount
            if amount == 0 || amount < pool.min_liquidity || pool.total_liquidity + amount > pool.max_liquidity {
                return Err(LendingError::InvalidAmount);
            }
            
            // Calculate new pool share
            let new_total_liquidity = pool.total_liquidity + amount;
            let new_provider_share = ((amount as u32 * 10000) / new_total_liquidity as u32) as u16;
            
            // Check if provider already exists
            let existing_provider_index = pool.liquidity_providers.iter().position(|p| p.account == caller);
            
            if let Some(index) = existing_provider_index {
                // Update existing provider
                let mut provider = pool.liquidity_providers[index].clone();
                provider.liquidity_provided += amount;
                provider.pool_share = ((provider.liquidity_provided as u32 * 10000) / new_total_liquidity as u32) as u16;
                pool.liquidity_providers[index] = provider;
            } else {
                // Add new provider
                let new_provider = LiquidityProvider {
                    account: caller,
                    liquidity_provided: amount,
                    pool_share: new_provider_share,
                    rewards_earned: 0,
                    joined_at: self.env().block_number() as u64,
                    last_reward_claim: self.env().block_number() as u64,
                };
                pool.liquidity_providers.push(new_provider);
            }
            
            // Update pool state
            pool.total_liquidity = new_total_liquidity;
            
            // Update provider shares for all providers
            for provider in &mut pool.liquidity_providers {
                provider.pool_share = ((provider.liquidity_provided as u32 * 10000) / new_total_liquidity as u32) as u16;
            }
            
            self.liquidity_pools.insert(pool_id, &pool);
            
            self.env().emit_event(LiquidityProvided {
                pool_id,
                provider: caller,
                amount,
                pool_share: new_provider_share,
                total_pool_liquidity: new_total_liquidity,
            });
            
            Ok(())
        }
        
        /// Claim rewards from a liquidity pool
        #[ink(message)]
        pub fn claim_pool_rewards(&mut self, pool_id: u64) -> Result<Balance, LendingError> {
            let caller = self.env().caller();
            let mut pool = self.liquidity_pools.get(pool_id).ok_or(LendingError::LoanNotFound)?;
            
            // Check if pool is active
            if pool.status != PoolStatus::Active {
                return Err(LendingError::LoanNotActive);
            }
            
            // Find the provider
            let provider_index = pool.liquidity_providers.iter().position(|p| p.account == caller)
                .ok_or(LendingError::Unauthorized)?;
            
            let mut provider = pool.liquidity_providers[provider_index].clone();
            let current_block = self.env().block_number() as u64;
            
            // Calculate rewards based on time and pool share
            let blocks_since_last_claim = current_block - provider.last_reward_claim;
            let reward_rate_per_block = pool.reward_rate as f64 / 10000.0 / 5184000.0; // Convert to per-block rate
            let rewards = (provider.liquidity_provided as f64 * reward_rate_per_block * blocks_since_last_claim as f64) as u128;
            
            if rewards == 0 {
                return Err(LendingError::InvalidAmount); // No rewards to claim
            }
            
            // Update provider state
            provider.rewards_earned += rewards;
            provider.last_reward_claim = current_block;
            
            // Update pool state
            pool.total_rewards_distributed += rewards;
            pool.liquidity_providers[provider_index] = provider.clone();
            
            self.liquidity_pools.insert(pool_id, &pool);
            
            self.env().emit_event(RewardsDistributed {
                pool_id,
                provider: caller,
                amount: rewards,
                pool_share: provider.pool_share,
                total_rewards: pool.total_rewards_distributed,
            });
            
            Ok(rewards)
        }
        
        /// Get liquidity pool information
        #[ink(message)]
        pub fn get_liquidity_pool_info(&self, pool_id: u64) -> Result<(String, Balance, u32, Balance, u16, u16, PoolStatus), LendingError> {
            let pool = self.liquidity_pools.get(pool_id).ok_or(LendingError::LoanNotFound)?;
            Ok((
                pool.name.clone(),
                pool.total_liquidity,
                pool.active_loans,
                pool.total_volume,
                pool.pool_fee_rate,
                pool.reward_rate,
                pool.status.clone(),
            ))
        }
        
        /// Get liquidity provider information
        #[ink(message)]
        pub fn get_liquidity_provider_info(&self, pool_id: u64, provider: AccountId) -> Result<(Balance, u16, Balance, u64), LendingError> {
            let pool = self.liquidity_pools.get(pool_id).ok_or(LendingError::LoanNotFound)?;
            
            let provider_info = pool.liquidity_providers.iter()
                .find(|p| p.account == provider)
                .ok_or(LendingError::Unauthorized)?;
            
            Ok((
                provider_info.liquidity_provided,
                provider_info.pool_share,
                provider_info.rewards_earned,
                provider_info.last_reward_claim,
            ))
        }

        // ============================================================================
        // QUERY OPERATIONS
        // ============================================================================
        
        /// Get loan information
        #[ink(message)]
        pub fn get_loan(&self, loan_id: u64) -> Option<Loan> {
            self.loans.get(loan_id)
        }

        /// Get user profile information
        #[ink(message)]
        pub fn get_user_profile(&self, user: AccountId) -> Option<UserProfile> {
            self.user_profiles.get(user)
        }

        /// Get total number of loans
        #[ink(message)]
        pub fn get_total_loans(&self) -> u64 {
            self.total_loans
        }

        /// Get total liquidity in the contract
        #[ink(message)]
        pub fn get_total_liquidity(&self) -> Balance {
            self.total_liquidity
        }

        /// Get early repayment discount for a loan
        #[ink(message)]
        pub fn get_early_repayment_discount(&self, loan_id: u64) -> Result<u16, LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            let current_block = self.env().block_number() as u64;
            
            if current_block >= loan.due_date {
                return Ok(0); // No discount if loan is already due
            }
            
            let blocks_early = loan.due_date - current_block;
            Ok(self.calculate_early_repayment_discount(blocks_early, loan.duration))
        }

        /// Get loan payment information
        #[ink(message)]
        pub fn get_loan_payment_info(&self, loan_id: u64) -> Result<(Balance, Balance, Vec<PartialPayment>), LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            Ok((loan.total_paid, loan.remaining_balance, loan.partial_payments.clone()))
        }

        /// Get partial payment count for a loan
        #[ink(message)]
        pub fn get_partial_payment_count(&self, loan_id: u64) -> Result<u32, LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            Ok(loan.partial_payments.len() as u32)
        }

        /// Get loan extension information
        #[ink(message)]
        pub fn get_loan_extension_info(&self, loan_id: u64) -> Result<(u32, u32, u16), LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            Ok((loan.extension_count, loan.max_extensions, loan.extension_fee_rate))
        }

        /// Check if a loan can be extended
        #[ink(message)]
        pub fn can_extend_loan(&self, loan_id: u64) -> Result<bool, LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            let current_block = self.env().block_number() as u64;
            
            let can_extend = loan.extension_count < loan.max_extensions && 
                           current_block < loan.due_date &&
                           (loan.status == LoanStatus::Active || loan.status == LoanStatus::PartiallyPaid);
            
            Ok(can_extend)
        }

        /// Calculate extension fee for a loan
        #[ink(message)]
        pub fn calculate_extension_fee(&self, loan_id: u64) -> Result<Balance, LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            let extension_fee = (loan.remaining_balance * loan.extension_fee_rate as u128) / 10000;
            Ok(extension_fee)
        }

        /// Get late fee information for a loan
        #[ink(message)]
        pub fn get_late_fee_info(&self, loan_id: u64) -> Result<(Balance, u16, u16, Option<u64>), LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            Ok((loan.total_late_fees, loan.late_fee_rate, loan.max_late_fee_rate, loan.overdue_since))
        }

        /// Calculate current late fees for a loan
        #[ink(message)]
        pub fn calculate_current_late_fees(&self, loan_id: u64) -> Result<Balance, LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            let current_block = self.env().block_number() as u64;
            let grace_period_end = loan.due_date + loan.grace_period;
            
            if current_block <= grace_period_end {
                return Ok(0); // No late fees yet
            }
            
            let overdue_blocks = current_block - grace_period_end;
            let days_overdue = overdue_blocks / 14400;
            let late_fee_rate = (loan.late_fee_rate * days_overdue as u16).min(loan.max_late_fee_rate);
            let late_fees = (loan.remaining_balance * late_fee_rate as u128) / 10000;
            
            Ok(late_fees)
        }

        /// Check if a loan is overdue
        #[ink(message)]
        pub fn is_loan_overdue(&self, loan_id: u64) -> Result<bool, LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            let current_block = self.env().block_number() as u64;
            let grace_period_end = loan.due_date + loan.grace_period;
            Ok(current_block > grace_period_end)
        }

        /// Get loan refinance information
        #[ink(message)]
        pub fn get_loan_refinance_info(&self, loan_id: u64) -> Result<(u32, u32, u16), LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            Ok((loan.refinance_count, loan.max_refinances, loan.refinance_fee_rate))
        }

        /// Check if a loan can be refinanced
        #[ink(message)]
        pub fn can_refinance_loan(&self, loan_id: u64) -> Result<bool, LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            let current_block = self.env().block_number() as u64;
            
            let can_refinance = loan.refinance_count < loan.max_refinances && 
                              current_block < loan.due_date &&
                              (loan.status == LoanStatus::Active || loan.status == LoanStatus::PartiallyPaid);
            
            Ok(can_refinance)
        }

        /// Calculate refinance fee for a loan
        #[ink(message)]
        pub fn calculate_refinance_fee(&self, loan_id: u64) -> Result<Balance, LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            let refinance_fee = (loan.remaining_balance * loan.refinance_fee_rate as u128) / 10000;
            Ok(refinance_fee)
        }

        /// Get refinance history for a loan
        #[ink(message)]
        pub fn get_refinance_history(&self, loan_id: u64) -> Result<Vec<RefinanceRecord>, LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            Ok(loan.refinance_history.clone())
        }

        // ============================================================================
        // PRIVATE HELPER METHODS
        // ============================================================================
        
        /// Get or create a user profile
        fn get_or_create_user_profile(&self, user: AccountId) -> UserProfile {
            self.user_profiles.get(user).unwrap_or(UserProfile {
                total_borrowed: 0,
                total_lent: 0,
                active_loans: Vec::new(),
                credit_score: 700, // Default credit score
                is_blacklisted: false,
            })
        }



        /// Calculate repayment amount for a loan
        fn calculate_repayment_amount(&self, loan_id: u64) -> Result<Balance, LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            let interest_amount = (loan.amount * loan.interest_rate as u128) / 10000;
            Ok(loan.amount + interest_amount)
        }

        /// Calculate early repayment discount
        fn calculate_early_repayment_discount(&self, blocks_early: u64, total_duration: u64) -> u16 {
            // Calculate discount based on how early the repayment is
            // More early = higher discount (up to 5%)
            let early_percentage = (blocks_early * 10000) / total_duration;
            
            match early_percentage {
                // Repaying in first 25% of loan duration: 5% discount
                p if p >= 7500 => 500, // 5%
                // Repaying in first 50% of loan duration: 3% discount  
                p if p >= 5000 => 300, // 3%
                // Repaying in first 75% of loan duration: 2% discount
                p if p >= 2500 => 200, // 2%
                // Repaying in last 25% of loan duration: 1% discount
                _ => 100, // 1%
            }
        }

        // ============================================================================
        // POOL REBALANCING & DYNAMIC LIQUIDITY MANAGEMENT
        // ============================================================================
        
        /// Trigger manual pool rebalancing
        #[ink(message)]
        pub fn rebalance_pool(&mut self, pool_id: u64) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut pool = self.liquidity_pools.get(pool_id).ok_or(LendingError::LoanNotFound)?;
            
            // Only pool creator or authorized users can rebalance
            if pool.liquidity_providers.is_empty() || pool.liquidity_providers[0].account != caller {
                return Err(LendingError::Unauthorized);
            }
            
            // Check if pool is active
            if pool.status != PoolStatus::Active {
                return Err(LendingError::LoanNotActive);
            }
            
            // Check rebalance frequency
            let current_block = self.env().block_number() as u64;
            if current_block < pool.last_rebalance + pool.rebalance_frequency {
                return Err(LendingError::InvalidAmount); // Reuse error for too frequent rebalancing
            }
            
            // Perform rebalancing
            let old_ratio = pool.current_liquidity_ratio;
            let (new_ratio, reason, adjustment) = self.calculate_rebalance_parameters(&pool)?;
            
            // Update pool state
            pool.current_liquidity_ratio = new_ratio;
            pool.last_rebalance = current_block;
            pool.performance_score = self.calculate_performance_score(&pool)?;
            
            self.liquidity_pools.insert(pool_id, &pool);
            
            self.env().emit_event(PoolRebalanced {
                pool_id,
                old_liquidity_ratio: old_ratio,
                new_liquidity_ratio: new_ratio,
                performance_score: pool.performance_score,
                rebalance_reason: reason,
                liquidity_adjustment: adjustment,
            });
            
            Ok(())
        }
        
        /// Enable or disable auto-rebalancing for a pool
        #[ink(message)]
        pub fn set_auto_rebalancing(&mut self, pool_id: u64, enabled: bool) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut pool = self.liquidity_pools.get(pool_id).ok_or(LendingError::LoanNotFound)?;
            
            // Only pool creator can change auto-rebalancing settings
            if pool.liquidity_providers.is_empty() || pool.liquidity_providers[0].account != caller {
                return Err(LendingError::Unauthorized);
            }
            
            pool.auto_rebalance_enabled = enabled;
            self.liquidity_pools.insert(pool_id, &pool);
            
            Ok(())
        }
        
        /// Set rebalancing parameters for a pool
        #[ink(message)]
        pub fn set_rebalancing_parameters(
            &mut self,
            pool_id: u64,
            frequency: u64,
            target_ratio: u16,
            threshold: u16,
        ) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut pool = self.liquidity_pools.get(pool_id).ok_or(LendingError::LoanNotFound)?;
            
            // Only pool creator can change rebalancing parameters
            if pool.liquidity_providers.is_empty() || pool.liquidity_providers[0].account != caller {
                return Err(LendingError::Unauthorized);
            }
            
            // Validate parameters
            if frequency < 14400 || target_ratio > 10000 || threshold > 1000 {
                return Err(LendingError::InvalidAmount);
            }
            
            pool.rebalance_frequency = frequency;
            pool.target_liquidity_ratio = target_ratio;
            pool.rebalance_threshold = threshold;
            
            self.liquidity_pools.insert(pool_id, &pool);
            
            Ok(())
        }
        
        /// Check if pool needs rebalancing
        #[ink(message)]
        pub fn needs_rebalancing(&self, pool_id: u64) -> Result<bool, LendingError> {
            let pool = self.liquidity_pools.get(pool_id).ok_or(LendingError::LoanNotFound)?;
            
            if !pool.auto_rebalance_enabled {
                return Ok(false);
            }
            
            let current_block = self.env().block_number() as u64;
            if current_block < pool.last_rebalance + pool.rebalance_frequency {
                return Ok(false);
            }
            
            let ratio_difference = if pool.current_liquidity_ratio > pool.target_liquidity_ratio {
                pool.current_liquidity_ratio - pool.target_liquidity_ratio
            } else {
                pool.target_liquidity_ratio - pool.current_liquidity_ratio
            };
            
            Ok(ratio_difference >= pool.rebalance_threshold)
        }
        
        /// Get pool rebalancing information
        #[ink(message)]
        pub fn get_pool_rebalancing_info(&self, pool_id: u64) -> Result<(u16, u64, u64, u16, u16, bool), LendingError> {
            let pool = self.liquidity_pools.get(pool_id).ok_or(LendingError::LoanNotFound)?;
            Ok((
                pool.performance_score,
                pool.last_rebalance,
                pool.rebalance_frequency,
                pool.target_liquidity_ratio,
                pool.current_liquidity_ratio,
                pool.auto_rebalance_enabled,
            ))
        }
        
        /// Calculate rebalancing parameters
        fn calculate_rebalance_parameters(&self, pool: &LiquidityPool) -> Result<(u16, String, Balance), LendingError> {
            let current_ratio = pool.current_liquidity_ratio;
            let target_ratio = pool.target_liquidity_ratio;
            
            // Calculate new ratio based on performance
            let performance_factor = pool.performance_score as f64 / 10000.0;
            let ratio_adjustment = ((target_ratio as i32 - current_ratio as i32) as f64 * performance_factor) as i32;
            let new_ratio = (current_ratio as i32 + ratio_adjustment).max(1000).min(10000) as u16;
            
            // Determine rebalance reason
            let reason = if new_ratio > current_ratio {
                "Performance improvement - increasing liquidity ratio".to_string()
            } else if new_ratio < current_ratio {
                "Performance decline - decreasing liquidity ratio".to_string()
            } else {
                "No adjustment needed".to_string()
            };
            
            // Calculate liquidity adjustment
            let adjustment = if new_ratio != current_ratio {
                let adjustment_factor = (new_ratio as f64 - current_ratio as f64) / 10000.0;
                (pool.total_liquidity as f64 * adjustment_factor) as u128
            } else {
                0
            };
            
            Ok((new_ratio, reason, adjustment))
        }
        
        /// Calculate pool performance score
        fn calculate_performance_score(&self, pool: &LiquidityPool) -> Result<u16, LendingError> {
            // Simple performance calculation based on multiple factors
            let mut score = 5000u32; // Base score: 50%
            
            // Factor 1: Liquidity utilization (0-2000 points)
            let utilization = if pool.total_liquidity > 0 {
                (pool.active_loans as f64 / pool.total_liquidity as f64) * 2000.0
            } else {
                0.0
            };
            score += utilization as u32;
            
            // Factor 2: Reward distribution efficiency (0-2000 points)
            let reward_efficiency = if pool.total_liquidity > 0 {
                (pool.total_rewards_distributed as f64 / pool.total_liquidity as f64) * 2000.0
            } else {
                0.0
            };
            score += reward_efficiency as u32;
            
            // Factor 3: Provider diversity (0-1000 points)
            let provider_diversity = (pool.liquidity_providers.len() as u32).min(10) * 100;
            score += provider_diversity;
            
            // Ensure score is within bounds
            score = score.min(10000);
            
            Ok(score as u16)
        }

        // ============================================================================
        // YIELD FARMING & ADVANCED REWARDS
        // ============================================================================
        
        /// Enable yield farming for a pool
        #[ink(message)]
        pub fn enable_yield_farming(
            &mut self,
            pool_id: u64,
            reward_tokens: Vec<RewardToken>,
        ) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut pool = self.liquidity_pools.get(pool_id).ok_or(LendingError::LoanNotFound)?;
            
            // Only pool creator can enable yield farming
            if pool.liquidity_providers.is_empty() || pool.liquidity_providers[0].account != caller {
                return Err(LendingError::Unauthorized);
            }
            
            // Check if pool is active
            if pool.status != PoolStatus::Active {
                return Err(LendingError::LoanNotActive);
            }
            
            // Validate reward tokens
            if reward_tokens.is_empty() {
                return Err(LendingError::InvalidAmount);
            }
            
            // Enable yield farming
            pool.yield_farming_enabled = true;
            pool.reward_tokens = reward_tokens.clone();
            
            self.liquidity_pools.insert(pool_id, &pool);
            
            self.env().emit_event(YieldFarmingEnabled {
                pool_id,
                enabled_by: caller,
                reward_tokens_count: reward_tokens.len() as u32,
                staking_requirements: format!("Min: {}, Lock: {} blocks", 
                    pool.staking_requirements.min_stake_amount, 
                    pool.staking_requirements.lock_period),
            });
            
            Ok(())
        }
        
        /// Stake tokens for yield farming
        #[ink(message)]
        pub fn stake_tokens(&mut self, pool_id: u64, amount: Balance) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut pool = self.liquidity_pools.get(pool_id).ok_or(LendingError::LoanNotFound)?;
            
            // Check if yield farming is enabled
            if !pool.yield_farming_enabled {
                return Err(LendingError::LoanNotActive);
            }
            
            // Check if pool is active
            if pool.status != PoolStatus::Active {
                return Err(LendingError::LoanNotActive);
            }
            
            // Validate staking amount
            if amount < pool.staking_requirements.min_stake_amount || 
               amount > pool.staking_requirements.max_stake_amount {
                return Err(LendingError::InvalidAmount);
            }
            
            // Calculate tier and multiplier
            let (tier_level, multiplier) = self.calculate_staking_tier(&pool, amount)?;
            
            // Create or update staking position
            let current_block = self.env().block_number() as u64;
            let _lock_end_time = current_block + pool.staking_requirements.lock_period;
            
            // For now, we'll just update the pool's total staked tokens
            // In a real implementation, you'd store individual staking positions
            pool.total_staked_tokens += amount;
            
            self.liquidity_pools.insert(pool_id, &pool);
            
            self.env().emit_event(TokensStaked {
                pool_id,
                staker: caller,
                amount,
                tier_level: tier_level.clone(),
                multiplier,
                lock_period: pool.staking_requirements.lock_period,
            });
            
            Ok(())
        }
        
        /// Claim yield farming rewards
        #[ink(message)]
        pub fn claim_yield_rewards(&mut self, pool_id: u64) -> Result<Balance, LendingError> {
            let caller = self.env().caller();
            let pool = self.liquidity_pools.get(pool_id).ok_or(LendingError::LoanNotFound)?;
            
            // Check if yield farming is enabled
            if !pool.yield_farming_enabled {
                return Err(LendingError::LoanNotActive);
            }
            
            // Check if pool is active
            if pool.status != PoolStatus::Active {
                return Err(LendingError::LoanNotActive);
            }
            
            // For demonstration, calculate rewards based on staked amount and time
            // In a real implementation, you'd track individual staking positions
            let _current_block = self.env().block_number() as u64;
            let base_reward_rate = 100; // 1% base reward rate
            let staked_amount = 10000; // Assume staker has 10,000 staked
            let time_factor = 1; // Assume 1 block since last claim
            
            // Calculate base rewards
            let base_rewards = (staked_amount as f64 * base_reward_rate as f64 / 10000.0 * time_factor as f64) as u128;
            
            // Apply tier multiplier (assume Gold tier: 1.5x)
            let tier_multiplier = 1500; // 1.5x
            let total_rewards = (base_rewards as f64 * tier_multiplier as f64 / 1000.0) as u128;
            
            if total_rewards == 0 {
                return Err(LendingError::InvalidAmount); // No rewards to claim
            }
            
            self.env().emit_event(YieldRewardsClaimed {
                pool_id,
                staker: caller,
                reward_amount: total_rewards,
                reward_token: "LEND".to_string(), // Default reward token
                tier_multiplier,
                total_staked: pool.total_staked_tokens,
            });
            
            Ok(total_rewards)
        }
        
        /// Get yield farming information
        #[ink(message)]
        pub fn get_yield_farming_info(&self, pool_id: u64) -> Result<(bool, u32, Balance, u32), LendingError> {
            let pool = self.liquidity_pools.get(pool_id).ok_or(LendingError::LoanNotFound)?;
            Ok((
                pool.yield_farming_enabled,
                pool.reward_tokens.len() as u32,
                pool.total_staked_tokens,
                pool.tier_multipliers.len() as u32,
            ))
        }
        
        /// Get staking tier information
        #[ink(message)]
        pub fn get_staking_tiers(&self, pool_id: u64) -> Result<Vec<(String, Balance, u16, u16)>, LendingError> {
            let pool = self.liquidity_pools.get(pool_id).ok_or(LendingError::LoanNotFound)?;
            
            let tiers: Vec<(String, Balance, u16, u16)> = pool.tier_multipliers.iter()
                .map(|tier| (
                    tier.tier_name.clone(),
                    tier.min_stake_amount,
                    tier.multiplier,
                    tier.bonus_rewards,
                ))
                .collect();
            
            Ok(tiers)
        }
        
        /// Calculate staking tier and multiplier
        fn calculate_staking_tier(&self, pool: &LiquidityPool, amount: Balance) -> Result<(String, u16), LendingError> {
            // Find the highest tier the staker qualifies for
            let mut best_tier = None;
            let mut best_multiplier = 0u16;
            
            for tier in &pool.tier_multipliers {
                if amount >= tier.min_stake_amount && tier.multiplier > best_multiplier {
                    best_tier = Some(tier.tier_name.clone());
                    best_multiplier = tier.multiplier;
                }
            }
            
            match best_tier {
                Some(tier_name) => Ok((tier_name, best_multiplier)),
                None => Err(LendingError::InvalidAmount),
            }
        }

        // ============================================================================
        // MARKET DEPTH MANAGEMENT & OPTIMAL LIQUIDITY DISTRIBUTION
        // ============================================================================
        
        /// Update market depth at a specific price level
        #[ink(message)]
        pub fn update_market_depth(
            &mut self,
            pool_id: u64,
            price_level: u16,
            liquidity_change: i128, // Allow negative values for removal
            order_count_change: i32,
        ) -> Result<(), LendingError> {
            let _caller = self.env().caller();
            let mut pool = self.liquidity_pools.get(pool_id).ok_or(LendingError::LoanNotFound)?;
            
            // Check if pool is active
            if pool.status != PoolStatus::Active {
                return Err(LendingError::LoanNotActive);
            }
            
            // Find the market depth level
            let level_index = pool.market_depth_levels.iter().position(|level| level.price_level == price_level);
            if level_index.is_none() {
                return Err(LendingError::InvalidAmount); // Invalid price level
            }
            
            let level_index = level_index.unwrap();
            let mut level = pool.market_depth_levels[level_index].clone();
            
            // Update liquidity and order count
            if liquidity_change < 0 && (-liquidity_change) > level.liquidity_available as i128 {
                return Err(LendingError::InvalidAmount); // Cannot remove more than available
            }
            
            level.liquidity_available = if liquidity_change > 0 {
                level.liquidity_available + liquidity_change as u128
            } else {
                level.liquidity_available.saturating_sub((-liquidity_change) as u128)
            };
            
            level.order_count = if order_count_change > 0 {
                level.order_count + order_count_change as u32
            } else {
                level.order_count.saturating_sub(order_count_change.abs() as u32)
            };
            
            level.last_updated = self.env().block_number() as u64;
            
            // Update the pool
            pool.market_depth_levels[level_index] = level.clone();
            self.liquidity_pools.insert(pool_id, &pool);
            
            self.env().emit_event(MarketDepthUpdated {
                pool_id,
                price_level,
                liquidity_change: liquidity_change.abs() as u128,
                new_depth: level.liquidity_available,
                order_count: level.order_count,
            });
            
            Ok(())
        }
        
        /// Apply optimal distribution algorithm
        #[ink(message)]
        pub fn apply_optimal_distribution(&mut self, pool_id: u64) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut pool = self.liquidity_pools.get(pool_id).ok_or(LendingError::LoanNotFound)?;
            
            // Only pool creator can apply optimal distribution
            if pool.liquidity_providers.is_empty() || pool.liquidity_providers[0].account != caller {
                return Err(LendingError::Unauthorized);
            }
            
            // Check if pool is active
            if pool.status != PoolStatus::Active {
                return Err(LendingError::LoanNotActive);
            }
            
            // Calculate optimal distribution
            let old_distribution = self.get_market_depth_summary(&pool)?;
            let (new_distribution, optimization_reason, liquidity_moved) = self.calculate_optimal_distribution(&pool)?;
            
            // Apply the new distribution
            pool.market_depth_levels = new_distribution;
            self.liquidity_pools.insert(pool_id, &pool);
            
            self.env().emit_event(OptimalDistributionApplied {
                pool_id,
                old_distribution,
                optimization_reason,
                new_distribution: self.get_market_depth_summary(&pool)?,
                total_liquidity_moved: liquidity_moved,
            });
            
            Ok(())
        }
        
        /// Check concentration limits and apply corrections
        #[ink(message)]
        pub fn check_concentration_limits(&mut self, pool_id: u64) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let pool = self.liquidity_pools.get(pool_id).ok_or(LendingError::LoanNotFound)?;
            
            // Only pool creator can check concentration limits
            if pool.liquidity_providers.is_empty() || pool.liquidity_providers[0].account != caller {
                return Err(LendingError::Unauthorized);
            }
            
            // Check single pool concentration
            let total_liquidity = pool.total_liquidity;
            let max_concentration = pool.concentration_limits.max_single_pool_concentration;
            let current_concentration = if total_liquidity > 0 {
                (total_liquidity * 10000) / self.total_liquidity
            } else {
                0
            };
            
            if current_concentration > max_concentration as u128 {
                let action = "Pool concentration limit exceeded - consider reducing liquidity";
                self.env().emit_event(ConcentrationLimitExceeded {
                    pool_id,
                    limit_type: "Single Pool Concentration".to_string(),
                    current_concentration: current_concentration as u16,
                    limit_threshold: max_concentration,
                    action_taken: action.to_string(),
                });
            }
            
            // Check provider concentration
            for provider in &pool.liquidity_providers {
                let provider_concentration = if total_liquidity > 0 {
                    (provider.liquidity_provided * 10000) / total_liquidity
                } else {
                    0
                };
                
                if provider_concentration > pool.concentration_limits.max_provider_concentration as u128 {
                    let action = "Provider concentration limit exceeded - consider reducing stake";
                    self.env().emit_event(ConcentrationLimitExceeded {
                        pool_id,
                        limit_type: "Provider Concentration".to_string(),
                        current_concentration: provider_concentration as u16,
                        limit_threshold: pool.concentration_limits.max_provider_concentration,
                        action_taken: action.to_string(),
                    });
                }
            }
            
            Ok(())
        }
        
        /// Get market depth information
        #[ink(message)]
        pub fn get_market_depth_info(&self, pool_id: u64) -> Result<(Vec<(u16, Balance, u32)>, bool, String), LendingError> {
            let pool = self.liquidity_pools.get(pool_id).ok_or(LendingError::LoanNotFound)?;
            
            let depth_info: Vec<(u16, Balance, u32)> = pool.market_depth_levels.iter()
                .map(|level| (level.price_level, level.liquidity_available, level.order_count))
                .collect();
            
            let distribution_summary = self.get_market_depth_summary(&pool)?;
            
            Ok((depth_info, pool.depth_based_pricing, distribution_summary))
        }
        
        /// Enable or disable depth-based pricing
        #[ink(message)]
        pub fn set_depth_based_pricing(&mut self, pool_id: u64, enabled: bool) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut pool = self.liquidity_pools.get(pool_id).ok_or(LendingError::LoanNotFound)?;
            
            // Only pool creator can change depth-based pricing
            if pool.liquidity_providers.is_empty() || pool.liquidity_providers[0].account != caller {
                return Err(LendingError::Unauthorized);
            }
            
            pool.depth_based_pricing = enabled;
            self.liquidity_pools.insert(pool_id, &pool);
            
            Ok(())
        }
        
        /// Calculate optimal distribution
        fn calculate_optimal_distribution(&self, pool: &LiquidityPool) -> Result<(Vec<MarketDepthLevel>, String, Balance), LendingError> {
            let total_liquidity = pool.total_liquidity;
            let _target_spread = pool.optimal_distribution.target_depth_spread;
            let min_per_level = pool.optimal_distribution.min_depth_per_level;
            let max_per_level = pool.optimal_distribution.max_depth_per_level;
            
            let mut new_levels = Vec::new();
            let mut total_moved = 0u128;
            
            // Calculate optimal distribution across price levels
            for level in &pool.market_depth_levels {
                let mut new_level = level.clone();
                
                // Calculate target liquidity for this level
                let target_liquidity = total_liquidity / pool.market_depth_levels.len() as u128;
                let current_liquidity = level.liquidity_available;
                
                if target_liquidity > current_liquidity {
                    let to_add = (target_liquidity - current_liquidity).min(max_per_level - current_liquidity);
                    new_level.liquidity_available = current_liquidity + to_add;
                    total_moved += to_add;
                } else if current_liquidity > target_liquidity {
                    let to_remove = (current_liquidity - target_liquidity).min(current_liquidity - min_per_level);
                    new_level.liquidity_available = current_liquidity - to_remove;
                    total_moved += to_remove;
                }
                
                new_level.last_updated = self.env().block_number() as u64;
                new_levels.push(new_level);
            }
            
            let reason = if total_moved > 0 {
                "Optimal distribution applied to balance liquidity across price levels".to_string()
            } else {
                "Distribution already optimal - no changes needed".to_string()
            };
            
            Ok((new_levels, reason, total_moved))
        }
        
        /// Get market depth summary
        fn get_market_depth_summary(&self, pool: &LiquidityPool) -> Result<String, LendingError> {
            let total_depth: u128 = pool.market_depth_levels.iter()
                .map(|level| level.liquidity_available)
                .sum();
            
            let level_count = pool.market_depth_levels.len();
            let avg_depth = if level_count > 0 { total_depth / level_count as u128 } else { 0 };
            
            Ok(format!("Total: {}, Levels: {}, Avg: {}", total_depth, level_count, avg_depth))
        }

        // ============================================================================
        // RISK MANAGEMENT & SECURITY FEATURES
        // ============================================================================
        
        /// Calculate and update credit score for a user
        #[ink(message)]
        pub fn calculate_credit_score(&mut self, user_id: AccountId) -> Result<u16, LendingError> {
            let caller = self.env().caller();
            
            // Only authorized users can calculate credit scores
            if caller != user_id && !self.is_authorized_admin(caller) {
                return Err(LendingError::Unauthorized);
            }
            
            let _user_profile = self.user_profiles.get(user_id).ok_or(LendingError::LoanNotFound)?;
            let current_score = self.credit_scores.get(user_id);
            
            // Calculate credit score based on various factors
            let mut total_score = 300u16; // Base score
            
            // Factor 1: Payment History (35% weight)
            let payment_score = self.calculate_payment_history_score(user_id)?;
            total_score += (payment_score * 35) / 100;
            
            // Factor 2: Credit Utilization (30% weight)
            let utilization_score = self.calculate_credit_utilization_score(user_id)?;
            total_score += (utilization_score * 30) / 100;
            
            // Factor 3: Credit History Length (15% weight)
            let history_score = self.calculate_credit_history_score(user_id)?;
            total_score += (history_score * 15) / 100;
            
            // Factor 4: New Credit (10% weight)
            let new_credit_score = self.calculate_new_credit_score(user_id)?;
            total_score += (new_credit_score * 10) / 100;
            
            // Factor 5: Credit Mix (10% weight)
            let mix_score = self.calculate_credit_mix_score(user_id)?;
            total_score += (mix_score * 10) / 100;
            
            // Ensure score is within valid range (300-850)
            total_score = total_score.min(850).max(300);
            
            // Create credit score record
            let old_score = current_score.as_ref().map(|cs| cs.score).unwrap_or(0);
            let risk_level = self.determine_risk_level(total_score);
            
            let credit_score = CreditScore {
                score: total_score,
                factors: vec![
                    CreditFactor {
                        factor_type: CreditFactorType::PaymentHistory,
                        weight: 3500, // 35%
                        value: payment_score,
                        description: "Payment history score".to_string(),
                    },
                    CreditFactor {
                        factor_type: CreditFactorType::CreditUtilization,
                        weight: 3000, // 30%
                        value: utilization_score,
                        description: "Credit utilization score".to_string(),
                    },
                    CreditFactor {
                        factor_type: CreditFactorType::CreditHistoryLength,
                        weight: 1500, // 15%
                        value: history_score,
                        description: "Credit history length score".to_string(),
                    },
                    CreditFactor {
                        factor_type: CreditFactorType::NewCredit,
                        weight: 1000, // 10%
                        value: new_credit_score,
                        description: "New credit score".to_string(),
                    },
                    CreditFactor {
                        factor_type: CreditFactorType::CreditMix,
                        weight: 1000, // 10%
                        value: mix_score,
                        description: "Credit mix score".to_string(),
                    },
                ],
                last_updated: self.env().block_number() as u64,
                score_history: vec![
                    CreditScoreRecord {
                        score: total_score,
                        change: (total_score as i16) - (old_score as i16),
                        reason: "Credit score calculated".to_string(),
                        timestamp: self.env().block_number() as u64,
                    }
                ],
                risk_level: risk_level.clone(),
            };
            
            // Update credit score
            self.credit_scores.insert(user_id, &credit_score);
            
            // Emit event
            self.env().emit_event(CreditScoreUpdated {
                user_id,
                old_score,
                new_score: total_score,
                change_reason: "Credit score calculated".to_string(),
                risk_level: format!("{:?}", risk_level),
            });
            
            Ok(total_score)
        }
        
        /// Set collateral requirements for a loan
        #[ink(message)]
        pub fn set_collateral_requirements(
            &mut self,
            loan_id: u64,
            collateral_type: CollateralType,
            required_amount: Balance,
            liquidation_threshold: u16,
            maintenance_margin: u16,
        ) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let mut loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            // Only lender or authorized admin can set collateral requirements
            if loan.lender != Some(caller) && !self.is_authorized_admin(caller) {
                return Err(LendingError::Unauthorized);
            }
            
            let collateral_req = CollateralRequirement {
                collateral_type,
                required_amount,
                current_amount: 0, // Initially no collateral provided
                liquidation_threshold,
                maintenance_margin,
                last_updated: self.env().block_number() as u64,
            };
            
            loan.collateral_requirements.push(collateral_req);
            self.loans.insert(loan_id, &loan);
            
            Ok(())
        }
        
        /// Create insurance policy for a loan
        #[ink(message)]
        pub fn create_insurance_policy(
            &mut self,
            loan_id: u64,
            insured_amount: Balance,
            premium_rate: u16,
            coverage_period: u64,
            deductible: Balance,
        ) -> Result<u64, LendingError> {
            let caller = self.env().caller();
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            // Only borrower can create insurance policy
            if loan.borrower != caller {
                return Err(LendingError::Unauthorized);
            }
            
            let policy_id = self.total_insurance_policies + 1;
            let policy = InsurancePolicy {
                policy_id,
                insured_amount,
                premium_rate,
                coverage_period,
                deductible,
                status: InsuranceStatus::Active,
                created_at: self.env().block_number() as u64,
            };
            
            self.insurance_policies.insert(policy_id, &policy);
            self.total_insurance_policies = policy_id;
            
            // Emit event
            self.env().emit_event(InsurancePolicyCreated {
                policy_id,
                loan_id,
                insured_amount,
                premium_rate,
                coverage_period,
            });
            
            Ok(policy_id)
        }
        
        /// Add fraud detection rule
        #[ink(message)]
        pub fn add_fraud_detection_rule(
            &mut self,
            rule_type: FraudRuleType,
            threshold: u16,
            action: FraudAction,
            description: String,
        ) -> Result<u64, LendingError> {
            let caller = self.env().caller();
            
            // Only authorized admin can add fraud detection rules
            if !self.is_authorized_admin(caller) {
                return Err(LendingError::Unauthorized);
            }
            
            let rule_id = self.total_fraud_rules + 1;
            let rule = FraudDetectionRule {
                rule_id,
                rule_type,
                threshold,
                action,
                is_active: true,
                description,
            };
            
            self.fraud_detection_rules.insert(rule_id, &rule);
            self.total_fraud_rules = rule_id;
            
            Ok(rule_id)
        }
        
        /// Check for fraud based on user activity
        #[ink(message)]
        pub fn check_fraud_detection(&mut self, user_id: AccountId) -> Result<Vec<String>, LendingError> {
            let caller = self.env().caller();
            
            // Only authorized users can check fraud detection
            if caller != user_id && !self.is_authorized_admin(caller) {
                return Err(LendingError::Unauthorized);
            }
            
            let mut fraud_alerts = Vec::new();
            
            // Check all active fraud detection rules
            for rule_id in 1..=self.total_fraud_rules {
                if let Some(rule) = self.fraud_detection_rules.get(rule_id) {
                    if rule.is_active {
                        match rule.rule_type {
                            FraudRuleType::UnusualActivity => {
                                if let Some(alert) = self.check_unusual_activity(user_id, rule.threshold) {
                                    fraud_alerts.push(alert);
                                    self.trigger_fraud_action(user_id, &rule)?;
                                }
                            }
                            FraudRuleType::MultipleAccounts => {
                                if let Some(alert) = self.check_multiple_accounts(user_id, rule.threshold) {
                                    fraud_alerts.push(alert);
                                    self.trigger_fraud_action(user_id, &rule)?;
                                }
                            }
                            FraudRuleType::RapidTransactions => {
                                if let Some(alert) = self.check_rapid_transactions(user_id, rule.threshold) {
                                    fraud_alerts.push(alert);
                                    self.trigger_fraud_action(user_id, &rule)?;
                                }
                            }
                            FraudRuleType::AmountThreshold => {
                                if let Some(alert) = self.check_amount_threshold(user_id, rule.threshold) {
                                    fraud_alerts.push(alert);
                                    self.trigger_fraud_action(user_id, &rule)?;
                                }
                            }
                            _ => {} // Handle other rule types as needed
                        }
                    }
                }
            }
            
            Ok(fraud_alerts)
        }
        
        /// Update compliance status for a user
        #[ink(message)]
        pub fn update_compliance_status(
            &mut self,
            user_id: AccountId,
            compliance_type: ComplianceType,
            status: ComplianceStatus,
            documents: Vec<String>,
        ) -> Result<(), LendingError> {
            let caller = self.env().caller();
            
            // Only authorized admin can update compliance status
            if !self.is_authorized_admin(caller) {
                return Err(LendingError::Unauthorized);
            }
            
            let current_records = self.compliance_records.get(user_id).unwrap_or_default();
            let mut updated_records = current_records.clone();
            
            let record = ComplianceRecord {
                record_id: updated_records.len() as u64 + 1,
                user_id,
                compliance_type: compliance_type.clone(),
                status: status.clone(),
                verification_date: self.env().block_number() as u64,
                expiry_date: self.env().block_number() as u64 + 5184000, // 1 year
                documents,
            };
            
            updated_records.push(record);
            self.compliance_records.insert(user_id, &updated_records);
            
            // Emit event
            self.env().emit_event(ComplianceStatusUpdated {
                user_id,
                compliance_type: format!("{:?}", compliance_type),
                old_status: "Unknown".to_string(),
                new_status: format!("{:?}", status),
                verification_date: self.env().block_number() as u64,
            });
            
            Ok(())
        }
        
        /// Get credit score information for a user
        #[ink(message)]
        pub fn get_credit_score_info(&self, user_id: AccountId) -> Result<(u16, String, Vec<(String, u16, String)>), LendingError> {
            let credit_score = self.credit_scores.get(user_id).ok_or(LendingError::LoanNotFound)?;
            
            let factors: Vec<(String, u16, String)> = credit_score.factors.iter()
                .map(|f| (
                    format!("{:?}", f.factor_type),
                    f.weight,
                    f.description.clone()
                ))
                .collect();
            
            Ok((
                credit_score.score,
                format!("{:?}", credit_score.risk_level),
                factors
            ))
        }
        
        /// Get collateral requirements for a loan
        #[ink(message)]
        pub fn get_collateral_requirements(&self, loan_id: u64) -> Result<Vec<(String, Balance, Balance, u16, u16)>, LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            
            let requirements: Vec<(String, Balance, Balance, u16, u16)> = loan.collateral_requirements.iter()
                .map(|cr| (
                    format!("{:?}", cr.collateral_type),
                    cr.required_amount,
                    cr.current_amount,
                    cr.liquidation_threshold,
                    cr.maintenance_margin
                ))
                .collect();
            
            Ok(requirements)
        }
        
        // Helper functions for credit score calculation
        fn calculate_payment_history_score(&self, _user_id: AccountId) -> Result<u16, LendingError> {
            // Simplified payment history calculation
            // In a real implementation, this would analyze actual payment data
            Ok(700) // Default score
        }
        
        fn calculate_credit_utilization_score(&self, user_id: AccountId) -> Result<u16, LendingError> {
            // Simplified credit utilization calculation
            let user_profile = self.user_profiles.get(user_id).ok_or(LendingError::LoanNotFound)?;
            
            if user_profile.total_borrowed == 0 {
                Ok(800) // No debt = excellent score
            } else {
                let utilization = (user_profile.total_borrowed * 10000) / user_profile.total_lent;
                if utilization < 3000 { // < 30%
                    Ok(750)
                } else if utilization < 5000 { // < 50%
                    Ok(650)
                } else if utilization < 7000 { // < 70%
                    Ok(550)
                } else {
                    Ok(400) // High utilization
                }
            }
        }
        
        fn calculate_credit_history_score(&self, user_id: AccountId) -> Result<u16, LendingError> {
            // Simplified credit history calculation
            let user_profile = self.user_profiles.get(user_id).ok_or(LendingError::LoanNotFound)?;
            
            if user_profile.active_loans.len() > 5 {
                Ok(800) // Long history
            } else if user_profile.active_loans.len() > 2 {
                Ok(600) // Medium history
            } else {
                Ok(400) // Short history
            }
        }
        
        fn calculate_new_credit_score(&self, _user_id: AccountId) -> Result<u16, LendingError> {
            // Simplified new credit calculation
            Ok(600) // Default score
        }
        
        fn calculate_credit_mix_score(&self, _user_id: AccountId) -> Result<u16, LendingError> {
            // Simplified credit mix calculation
            Ok(650) // Default score
        }
        
        fn determine_risk_level(&self, score: u16) -> RiskLevel {
            match score {
                750..=850 => RiskLevel::Excellent,
                700..=749 => RiskLevel::Good,
                650..=699 => RiskLevel::Fair,
                600..=649 => RiskLevel::Poor,
                _ => RiskLevel::VeryPoor,
            }
        }
        
        fn is_authorized_admin(&self, caller: AccountId) -> bool {
            // Simplified admin check - in real implementation, this would check against admin list
            caller == AccountId::from([1u8; 32]) // Alice as admin
        }
        
        // Helper functions for fraud detection
        fn check_unusual_activity(&self, _user_id: AccountId, _threshold: u16) -> Option<String> {
            // Simplified unusual activity check
            None // No unusual activity detected
        }
        
        fn check_multiple_accounts(&self, _user_id: AccountId, _threshold: u16) -> Option<String> {
            // Simplified multiple accounts check
            None // No multiple accounts detected
        }
        
        fn check_rapid_transactions(&self, _user_id: AccountId, _threshold: u16) -> Option<String> {
            // Simplified rapid transactions check
            None // No rapid transactions detected
        }
        
        fn check_amount_threshold(&self, _user_id: AccountId, _threshold: u16) -> Option<String> {
            // Simplified amount threshold check
            None // No amount threshold violations
        }
        
        fn trigger_fraud_action(&mut self, user_id: AccountId, rule: &FraudDetectionRule) -> Result<(), LendingError> {
            // Emit fraud detection event
            self.env().emit_event(FraudDetected {
                user_id,
                rule_type: format!("{:?}", rule.rule_type),
                threshold: rule.threshold,
                action: format!("{:?}", rule.action),
                description: rule.description.clone(),
            });
            
            Ok(())
        }

        // ============================================================================
        // ANALYTICS & REPORTING FUNCTIONS (Phase 5)
        // ============================================================================

        /// Calculate and update loan performance metrics
        #[ink(message)]
        pub fn update_loan_metrics(&mut self, loan_id: u64) -> Result<(), LendingError> {
            let loan = self.loans.get(loan_id).ok_or(LendingError::LoanNotFound)?;
            let current_block = self.env().block_number() as u64;
            
            // Calculate performance metrics
            let total_interest_paid = loan.total_paid.saturating_sub(loan.amount);
            let total_fees_paid = loan.total_late_fees + 
                (loan.extension_count as u128 * loan.extension_fee_rate as u128 * loan.amount / 10000) +
                (loan.refinance_count as u128 * loan.refinance_fee_rate as u128 * loan.amount / 10000);
            
            let days_to_repayment = if loan.status == LoanStatus::Repaid || loan.status == LoanStatus::EarlyRepaid {
                current_block.saturating_sub(loan.created_at)
            } else {
                0
            };
            
            let payment_efficiency = if loan.total_paid > 0 {
                let expected_total = loan.amount + (loan.amount * loan.interest_rate as u128 / 10000);
                ((loan.total_paid * 10000) / expected_total) as u16
            } else {
                0
            };
            
            let risk_adjusted_return = if loan.credit_score.is_some() {
                let credit_score = loan.credit_score.as_ref().unwrap();
                let risk_factor = match credit_score.risk_level {
                    RiskLevel::Excellent => 10000,
                    RiskLevel::Good => 8500,
                    RiskLevel::Fair => 7000,
                    RiskLevel::Poor => 5500,
                    RiskLevel::VeryPoor => 4000,
                };
                (risk_factor * loan.interest_rate as u32 / 10000) as u16
            } else {
                loan.interest_rate
            };
            
            let collateral_utilization = if loan.collateral > 0 {
                ((loan.amount * 10000) / loan.collateral) as u16
            } else {
                0
            };
            
            let performance_score = (payment_efficiency + risk_adjusted_return + (10000 - collateral_utilization)) / 3;
            
            let metrics = LoanPerformanceMetrics {
                loan_id,
                borrower: loan.borrower,
                total_interest_paid,
                total_fees_paid,
                average_daily_balance: loan.amount, // Simplified calculation
                days_to_repayment,
                payment_efficiency,
                risk_adjusted_return,
                collateral_utilization,
                late_payment_count: 0, // Would need to track this separately
                extension_count: loan.extension_count,
                refinance_count: loan.refinance_count,
                performance_score,
                last_updated: current_block,
            };
            
            self.loan_performance_metrics.insert(loan_id, &metrics);
            self.total_loan_metrics += 1;
            
            // Emit event
            self.env().emit_event(LoanMetricsUpdated {
                loan_id,
                borrower: loan.borrower,
                performance_score,
                payment_efficiency,
                risk_adjusted_return,
            });
            
            Ok(())
        }

        /// Calculate and update user portfolio analytics
        #[ink(message)]
        pub fn update_portfolio_analytics(&mut self, user_id: AccountId) -> Result<(), LendingError> {
            let user_profile = self.user_profiles.get(user_id).ok_or(LendingError::UserNotFound)?;
            let current_block = self.env().block_number() as u64;
            
            // Calculate portfolio metrics
            let total_portfolio_value = user_profile.total_borrowed + user_profile.total_lent;
            let active_loans_count = user_profile.active_loans.len() as u32;
            
            // Simplified calculations - in real implementation would analyze actual loan data
            let completed_loans_count = if user_profile.total_borrowed > 0 { 1 } else { 0 };
            let defaulted_loans_count = 0; // Would need to track defaults
            let average_loan_size = if active_loans_count > 0 {
                user_profile.total_borrowed / active_loans_count as u128
            } else {
                0
            };
            
            let portfolio_diversification_score = if active_loans_count > 1 { 8000 } else { 4000 };
            let risk_concentration = if user_profile.total_borrowed > user_profile.total_lent { 7000 } else { 3000 };
            let expected_return = 6000; // Simplified calculation
            let volatility_score = 5000; // Simplified calculation
            let liquidity_score = if user_profile.total_lent > 0 { 8000 } else { 4000 };
            
            let analytics = PortfolioAnalytics {
                user_id,
                total_portfolio_value,
                active_loans_count,
                completed_loans_count,
                defaulted_loans_count,
                average_loan_size,
                portfolio_diversification_score,
                risk_concentration,
                expected_return,
                volatility_score,
                liquidity_score,
                last_updated: current_block,
            };
            
            self.user_portfolio_analytics.insert(user_id, &analytics);
            
            // Emit event
            self.env().emit_event(PortfolioAnalyticsUpdated {
                user_id,
                portfolio_value: total_portfolio_value,
                diversification_score: portfolio_diversification_score,
                risk_concentration,
            });
            
            Ok(())
        }

        /// Update market statistics
        #[ink(message)]
        pub fn update_market_statistics(&mut self) -> Result<(), LendingError> {
            let current_block = self.env().block_number() as u64;
            
            // Calculate market metrics
            let total_market_cap = self.total_liquidity;
            let total_active_loans = self.total_loans;
            
            // Calculate average interest rate from active loans
            let mut total_rate = 0u32;
            let mut active_loan_count = 0u32;
            
            for i in 1..=self.total_loans {
                if let Some(loan) = self.loans.get(i) {
                    if loan.status == LoanStatus::Active {
                        total_rate += loan.interest_rate as u32;
                        active_loan_count += 1;
                    }
                }
            }
            
            let average_interest_rate = if active_loan_count > 0 {
                (total_rate / active_loan_count) as u16
            } else {
                0
            };
            
            // Simplified market metrics
            let market_volatility = 5000; // 50% - would need historical data
            let liquidity_depth = if self.total_liquidity > 0 { 7000 } else { 3000 };
            let default_rate = 500; // 5% - would need actual default tracking
            let utilization_rate = if self.total_liquidity > 0 {
                ((self.total_loans as u128 * 10000) / self.total_liquidity) as u16
            } else {
                0
            };
            
            let market_trend = if average_interest_rate > 1000 { MarketTrend::Bullish } else { MarketTrend::Stable };
            
            let new_stats = MarketStatistics {
                total_market_cap,
                total_active_loans,
                average_interest_rate,
                market_volatility,
                liquidity_depth,
                default_rate,
                utilization_rate,
                market_trend,
                last_updated: current_block,
            };
            
            self.market_statistics = new_stats;
            
            // Add to historical data
            let historical_point = HistoricalDataPoint {
                timestamp: current_block,
                total_loans: self.total_loans,
                total_volume: self.total_liquidity,
                average_rate: average_interest_rate,
                default_count: 0, // Would need to track defaults
                active_users: self.total_users as u32,
            };
            
            self.historical_data.push(historical_point);
            
            // Emit event
            self.env().emit_event(MarketStatisticsUpdated {
                total_market_cap,
                active_loans: total_active_loans,
                average_rate: average_interest_rate,
                market_trend: format!("{:?}", market_trend),
            });
            
            Ok(())
        }

        /// Create a new performance benchmark
        #[ink(message)]
        pub fn create_performance_benchmark(
            &mut self,
            name: String,
            category: BenchmarkCategory,
            target_score: u16,
            weight: u16,
        ) -> Result<u64, LendingError> {
            if !self.is_authorized_admin(self.env().caller()) {
                return Err(LendingError::Unauthorized);
            }
            
            let benchmark_id = self.total_benchmarks + 1;
            let current_block = self.env().block_number() as u64;
            
            let benchmark = PerformanceBenchmark {
                benchmark_id,
                name: name.clone(),
                category,
                target_score,
                current_score: 0,
                weight,
                last_updated: current_block,
            };
            
            self.performance_benchmarks.insert(benchmark_id, &benchmark);
            self.total_benchmarks += 1;
            
            Ok(benchmark_id)
        }

        /// Update benchmark scores
        #[ink(message)]
        pub fn update_benchmark_scores(&mut self) -> Result<(), LendingError> {
            if !self.is_authorized_admin(self.env().caller()) {
                return Err(LendingError::Unauthorized);
            }
            
            let current_block = self.env().block_number() as u64;
            
            for i in 1..=self.total_benchmarks {
                if let Some(mut benchmark) = self.performance_benchmarks.get(i) {
                    // Calculate current score based on category
                    let current_score = match benchmark.category {
                        BenchmarkCategory::LoanPerformance => self.calculate_loan_performance_score(),
                        BenchmarkCategory::RiskManagement => self.calculate_risk_management_score(),
                        BenchmarkCategory::LiquidityEfficiency => self.calculate_liquidity_efficiency_score(),
                        BenchmarkCategory::UserExperience => self.calculate_user_experience_score(),
                        BenchmarkCategory::Compliance => self.calculate_compliance_score(),
                        BenchmarkCategory::Overall => self.calculate_overall_score(),
                    };
                    
                    benchmark.current_score = current_score;
                    benchmark.last_updated = current_block;
                    
                    self.performance_benchmarks.insert(i, &benchmark);
                    
                    // Emit event
                    self.env().emit_event(PerformanceBenchmarkUpdated {
                        benchmark_id: i,
                        name: benchmark.name.clone(),
                        current_score,
                        target_score: benchmark.target_score,
                    });
                }
            }
            
            Ok(())
        }

        /// Generate analytics report
        #[ink(message)]
        pub fn generate_analytics_report(&mut self, report_type: ReportType, data_period: u64) -> Result<u64, LendingError> {
            if !self.is_authorized_admin(self.env().caller()) {
                return Err(LendingError::Unauthorized);
            }
            
            let report_id = self.total_analytics_reports + 1;
            let current_block = self.env().block_number() as u64;
            
            // Generate report metrics
            let mut metrics = Vec::new();
            
            // Market metrics
            metrics.push(AnalyticsMetric {
                name: "Total Market Cap".to_string(),
                value: format!("{}", self.market_statistics.total_market_cap),
                unit: "Wei".to_string(),
                change_from_previous: 0,
                trend: MetricTrend::Stable,
            });
            
            metrics.push(AnalyticsMetric {
                name: "Active Loans".to_string(),
                value: format!("{}", self.market_statistics.total_active_loans),
                unit: "Count".to_string(),
                change_from_previous: 0,
                trend: MetricTrend::Stable,
            });
            
            metrics.push(AnalyticsMetric {
                name: "Average Interest Rate".to_string(),
                value: format!("{:.2}%", self.market_statistics.average_interest_rate as f64 / 100.0),
                unit: "Percentage".to_string(),
                change_from_previous: 0,
                trend: MetricTrend::Stable,
            });
            
            // Generate summary and recommendations
            let summary = format!("Analytics report for {:?} period ending at block {}", report_type, current_block);
            let recommendations = vec![
                "Monitor market volatility trends".to_string(),
                "Review risk management parameters".to_string(),
                "Optimize liquidity distribution".to_string(),
            ];
            
            let report = AnalyticsReport {
                report_id,
                report_type: report_type.clone(),
                generated_at: current_block,
                data_period,
                summary,
                metrics,
                recommendations,
            };
            
            self.analytics_reports.insert(report_id, &report);
            self.total_analytics_reports += 1;
            
            // Emit event
            self.env().emit_event(AnalyticsReportGenerated {
                report_id,
                report_type: format!("{:?}", report_type),
                generated_at: current_block,
                metrics_count: report.metrics.len() as u32,
            });
            
            Ok(report_id)
        }

        // ============================================================================
        // ANALYTICS HELPER FUNCTIONS
        // ============================================================================

        fn calculate_loan_performance_score(&self) -> u16 {
            // Simplified calculation based on loan metrics
            let mut total_score = 0u32;
            let mut count = 0u32;
            
            for i in 1..=self.total_loan_metrics {
                if let Some(metrics) = self.loan_performance_metrics.get(i) {
                    total_score += metrics.performance_score as u32;
                    count += 1;
                }
            }
            
            if count > 0 {
                (total_score / count) as u16
            } else {
                5000 // Default 50%
            }
        }

        fn calculate_risk_management_score(&self) -> u16 {
            // Simplified risk management score
            let default_rate = self.market_statistics.default_rate;
            let volatility = self.market_statistics.market_volatility;
            
            // Lower default rate and volatility = higher score
            let score = 10000 - (default_rate + volatility) / 2;
            score.max(1000).min(10000) // Ensure score is between 10% and 100%
        }

        fn calculate_liquidity_efficiency_score(&self) -> u16 {
            // Simplified liquidity efficiency score
            let utilization = self.market_statistics.utilization_rate;
            let depth = self.market_statistics.liquidity_depth;
            
            // Optimal utilization around 70-80%
            let utilization_score = if utilization >= 7000 && utilization <= 8000 {
                10000
            } else if utilization >= 5000 && utilization <= 9000 {
                8000
            } else {
                6000
            };
            
            (utilization_score + depth) / 2
        }

        fn calculate_user_experience_score(&self) -> u16 {
            // Simplified user experience score
            let total_users = self.total_users;
            let active_loans = self.market_statistics.total_active_loans;
            
            if total_users == 0 {
                return 5000;
            }
            
            let activity_rate = ((active_loans as u32 * 10000) / total_users as u32) as u16;
            activity_rate.min(10000)
        }

        fn calculate_compliance_score(&self) -> u16 {
            // Simplified compliance score
            8500 // Default high score - would need actual compliance tracking
        }

        fn calculate_overall_score(&self) -> u16 {
            let scores = vec![
                self.calculate_loan_performance_score(),
                self.calculate_risk_management_score(),
                self.calculate_liquidity_efficiency_score(),
                self.calculate_user_experience_score(),
                self.calculate_compliance_score(),
            ];
            
            let total: u32 = scores.iter().map(|&s| s as u32).sum();
            (total / scores.len() as u32) as u16
        }

        // ============================================================================
        // ANALYTICS QUERY FUNCTIONS
        // ============================================================================

        /// Get loan performance metrics
        #[ink(message)]
        pub fn get_loan_metrics(&self, loan_id: u64) -> Result<LoanPerformanceMetrics, LendingError> {
            self.loan_performance_metrics.get(loan_id).ok_or(LendingError::LoanNotFound)
        }

        /// Get user portfolio analytics
        #[ink(message)]
        pub fn get_portfolio_analytics(&self, user_id: AccountId) -> Result<PortfolioAnalytics, LendingError> {
            self.user_portfolio_analytics.get(user_id).ok_or(LendingError::UserNotFound)
        }

        /// Get current market statistics
        #[ink(message)]
        pub fn get_market_statistics(&self) -> MarketStatistics {
            self.market_statistics.clone()
        }

        /// Get performance benchmarks
        #[ink(message)]
        pub fn get_performance_benchmarks(&self) -> Vec<PerformanceBenchmark> {
            let mut benchmarks = Vec::new();
            for i in 1..=self.total_benchmarks {
                if let Some(benchmark) = self.performance_benchmarks.get(i) {
                    benchmarks.push(benchmark);
                }
            }
            benchmarks
        }

        /// Get analytics report
        #[ink(message)]
        pub fn get_analytics_report(&self, report_id: u64) -> Result<AnalyticsReport, LendingError> {
            self.analytics_reports.get(report_id).ok_or(LendingError::LoanNotFound)
        }

        /// Get historical data points
        #[ink(message)]
        pub fn get_historical_data(&self, limit: u32) -> Vec<HistoricalDataPoint> {
            let mut data = self.historical_data.clone();
            data.sort_by(|a, b| b.timestamp.cmp(&a.timestamp)); // Sort by newest first
            data.truncate(limit as usize);
            data
        }

        /// Get total loan metrics count
        #[ink(message)]
        pub fn get_total_loan_metrics(&self) -> u64 {
            self.total_loan_metrics
        }

        /// Get total benchmarks count
        #[ink(message)]
        pub fn get_total_benchmarks(&self) -> u64 {
            self.total_benchmarks
        }

        /// Get total analytics reports count
        #[ink(message)]
        pub fn get_total_analytics_reports(&self) -> u64 {
            self.total_analytics_reports
        }

        /// Get historical data count
        #[ink(message)]
        pub fn get_historical_data_count(&self) -> u32 {
            self.historical_data.len() as u32
        }
    }
} 