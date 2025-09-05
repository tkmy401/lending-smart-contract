use ink::storage::Mapping;
use ink_prelude::vec::Vec;

use crate::types::{
    Loan, LoanStatus, UserProfile, PartialPayment, PaymentType, RefinanceRecord,
    InterestRateType, InterestRateAdjustment, RateAdjustmentReason, InterestType, CompoundFrequency, PaymentStructure,
    GracePeriodReason, GracePeriodRecord, LiquidityPool, PoolStatus, LiquidityProvider, RewardToken, StakingRequirements, TierMultiplier,
    MarketDepthLevel, OptimalDistribution, ConcentrationLimits, CollateralType, CollateralRequirement, InsurancePolicy, InsuranceStatus, FraudDetectionRule, FraudRuleType, FraudAction, ComplianceRecord, ComplianceStatus, ComplianceType, CreditScore, CreditFactor, CreditFactorType, CreditScoreRecord, RiskLevel,
    MarketStatistics, MarketTrend, LoanPerformanceMetrics, PortfolioAnalytics, HistoricalDataPoint, PerformanceBenchmark, BenchmarkCategory, AnalyticsReport, ReportType, AnalyticsMetric, MetricTrend,
    FlashLoan, FlashLoanStatus, CrossChainBridge, BridgeStatus, CrossChainTransfer, TransferStatus, NFTCollateral, NFTMetadata, StakingPool, StakingPosition, LiquidityMining, LiquidityMiningPosition,
    GovernanceToken, GovernanceProposal, ProposalType, ProposalStatus, Vote, VoteChoice, Treasury, TreasuryTransaction, MultiSignatureWallet, MultiSigTransaction, DAOConfiguration, GovernanceSnapshot,
    BatchOperation, BatchOperationType, BatchItem, BatchStatus, BatchItemStatus, StorageOptimization, StorageOptimizationType, OptimizationStatus, UpgradeableContract, ContractUpgrade, GasOptimization, GasOptimizationType, ParallelProcessing, ParallelProcessType, ParallelOperation, ParallelProcessStatus, ParallelOperationStatus, PerformanceMetrics, PerformanceRating,
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
        // DeFi Integration (Phase 6)
        total_flash_loans: u64,
        flash_loans: Mapping<u64, FlashLoan>,
        total_nft_collateral: u64,
        nft_collateral: Mapping<u64, NFTCollateral>,
        total_cross_chain_bridges: u64,
        cross_chain_bridges: Mapping<u64, CrossChainBridge>,
        total_cross_chain_transfers: u64,
        cross_chain_transfers: Mapping<u64, CrossChainTransfer>,
        total_staking_pools: u64,
        staking_pools: Mapping<u64, StakingPool>,
        total_staking_positions: u64,
        staking_positions: Mapping<u64, StakingPosition>,
        user_staking_positions: Mapping<AccountId, Vec<u64>>,
        total_liquidity_mining_campaigns: u64,
        liquidity_mining_campaigns: Mapping<u64, LiquidityMining>,
        total_liquidity_mining_positions: u64,
        liquidity_mining_positions: Mapping<u64, LiquidityMiningPosition>,
        user_liquidity_mining_positions: Mapping<AccountId, Vec<u64>>,
        // Governance & DAO (Phase 7)
        total_governance_tokens: u64,
        governance_tokens: Mapping<u64, GovernanceToken>,
        total_proposals: u64,
        governance_proposals: Mapping<u64, GovernanceProposal>,
        total_votes: u64,
        votes: Mapping<u64, Vote>,
        user_votes: Mapping<AccountId, Vec<u64>>,
        total_treasuries: u64,
        treasuries: Mapping<u64, Treasury>,
        total_treasury_transactions: u64,
        treasury_transactions: Mapping<u64, TreasuryTransaction>,
        total_multi_sig_wallets: u64,
        multi_sig_wallets: Mapping<u64, MultiSignatureWallet>,
        total_multi_sig_transactions: u64,
        multi_sig_transactions: Mapping<u64, MultiSigTransaction>,
        total_daos: u64,
        dao_configurations: Mapping<u64, DAOConfiguration>,
        total_governance_snapshots: u64,
        governance_snapshots: Mapping<u64, GovernanceSnapshot>,
        user_governance_tokens: Mapping<AccountId, Balance>,
        user_voting_power: Mapping<AccountId, Balance>,
        // Performance & Gas Optimization (Phase 8)
        total_batch_operations: u64,
        batch_operations: Mapping<u64, BatchOperation>,
        total_storage_optimizations: u64,
        storage_optimizations: Mapping<u64, StorageOptimization>,
        total_upgradeable_contracts: u64,
        upgradeable_contracts: Mapping<u64, UpgradeableContract>,
        total_gas_optimizations: u64,
        gas_optimizations: Mapping<u64, GasOptimization>,
        total_parallel_processes: u64,
        parallel_processes: Mapping<u64, ParallelProcessing>,
        total_performance_metrics: u64,
        performance_metrics: Mapping<u64, PerformanceMetrics>,
        batch_operation_queue: Vec<u64>,
        optimization_queue: Vec<u64>,
        gas_usage_tracker: Mapping<String, u64>, // Function name -> gas usage
        storage_usage_tracker: Mapping<String, u64>, // Data structure -> storage usage
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

    // ============================================================================
    // DEFI INTEGRATION EVENTS (Phase 6)
    // ============================================================================

    #[ink(event)]
    pub struct FlashLoanExecuted {
        #[ink(topic)]
        flash_loan_id: u64,
        borrower: AccountId,
        asset: AccountId,
        amount: Balance,
        fee_amount: Balance,
        callback_target: AccountId,
    }

    #[ink(event)]
    pub struct FlashLoanRepaid {
        #[ink(topic)]
        flash_loan_id: u64,
        borrower: AccountId,
        asset: AccountId,
        amount: Balance,
        fee_amount: Balance,
        total_repay_amount: Balance,
    }

    #[ink(event)]
    pub struct NFTCollateralAdded {
        #[ink(topic)]
        nft_id: u64,
        owner: AccountId,
        contract_address: AccountId,
        token_id: u64,
        valuation: Balance,
        rarity_score: u16,
    }

    #[ink(event)]
    pub struct CrossChainBridgeCreated {
        #[ink(topic)]
        bridge_id: u64,
        source_chain: u32,
        target_chain: u32,
        source_asset: AccountId,
        target_asset: AccountId,
        bridge_fee: Balance,
    }

    #[ink(event)]
    pub struct CrossChainTransferInitiated {
        #[ink(topic)]
        transfer_id: u64,
        user: AccountId,
        source_chain: u32,
        target_chain: u32,
        amount: Balance,
        bridge_fee: Balance,
    }

    #[ink(event)]
    pub struct StakingPoolCreated {
        #[ink(topic)]
        pool_id: u64,
        token: AccountId,
        reward_rate: u16,
        min_stake: Balance,
        max_stake: Balance,
    }

    #[ink(event)]
    pub struct StakingPositionOpened {
        #[ink(topic)]
        position_id: u64,
        user: AccountId,
        pool_id: u64,
        amount: Balance,
        lock_period: u64,
        multiplier: u16,
    }

    #[ink(event)]
    pub struct LiquidityMiningCampaignCreated {
        #[ink(topic)]
        campaign_id: u64,
        name: String,
        reward_token: AccountId,
        total_rewards: Balance,
        start_block: u64,
        end_block: u64,
    }

    #[ink(event)]
    pub struct LiquidityMiningPositionOpened {
        #[ink(topic)]
        position_id: u64,
        user: AccountId,
        campaign_id: u64,
        staked_amount: Balance,
        multiplier: u16,
    }

    // ============================================================================
    // GOVERNANCE & DAO EVENTS (Phase 7)
    // ============================================================================

    #[ink(event)]
    pub struct GovernanceTokenCreated {
        #[ink(topic)]
        token_id: u64,
        name: String,
        symbol: String,
        total_supply: Balance,
        min_stake_for_voting: Balance,
        min_stake_for_proposal: Balance,
    }

    #[ink(event)]
    pub struct GovernanceProposalCreated {
        #[ink(topic)]
        proposal_id: u64,
        creator: AccountId,
        title: String,
        proposal_type: ProposalType,
        voting_start: u64,
        voting_end: u64,
        quorum: Balance,
        threshold: u16,
    }

    #[ink(event)]
    pub struct VoteCast {
        #[ink(topic)]
        proposal_id: u64,
        voter: AccountId,
        vote_choice: VoteChoice,
        voting_power: Balance,
        reason: Option<String>,
    }

    #[ink(event)]
    pub struct ProposalExecuted {
        #[ink(topic)]
        proposal_id: u64,
        executor: AccountId,
        executed_at: u64,
    }

    #[ink(event)]
    pub struct TreasuryCreated {
        #[ink(topic)]
        treasury_id: u64,
        name: String,
        daily_spend_limit: Balance,
        monthly_spend_limit: Balance,
        required_signatures: u32,
    }

    #[ink(event)]
    pub struct TreasuryTransactionProposed {
        #[ink(topic)]
        transaction_id: u64,
        treasury_id: u64,
        proposer: AccountId,
        recipient: AccountId,
        amount: Balance,
        purpose: String,
    }

    #[ink(event)]
    pub struct MultiSignatureWalletCreated {
        #[ink(topic)]
        wallet_id: u64,
        name: String,
        owners_count: u32,
        required_signatures: u32,
        daily_limit: Balance,
    }

    #[ink(event)]
    pub struct DAOCreated {
        #[ink(topic)]
        dao_id: u64,
        name: String,
        governance_token: AccountId,
        treasury: u64,
        multi_sig_wallet: u64,
        proposal_creation_threshold: Balance,
    }

    #[ink(event)]
    pub struct GovernanceSnapshotCreated {
        #[ink(topic)]
        snapshot_id: u64,
        proposal_id: u64,
        total_voting_power: Balance,
        total_participants: u32,
    }

    // ============================================================================
    // PERFORMANCE & GAS OPTIMIZATION EVENTS (Phase 8)
    // ============================================================================

    #[ink(event)]
    pub struct BatchOperationCreated {
        #[ink(topic)]
        batch_id: u64,
        operation_type: BatchOperationType,
        total_operations: u32,
        estimated_gas: u64,
        created_by: AccountId,
    }

    #[ink(event)]
    pub struct BatchOperationCompleted {
        #[ink(topic)]
        batch_id: u64,
        total_gas_used: u64,
        success_count: u32,
        error_count: u32,
        total_cost: Balance,
    }

    #[ink(event)]
    pub struct StorageOptimizationProposed {
        #[ink(topic)]
        optimization_id: u64,
        optimization_type: StorageOptimizationType,
        target_contract: AccountId,
        estimated_savings: u64,
        proposed_by: AccountId,
    }

    #[ink(event)]
    pub struct StorageOptimizationApplied {
        #[ink(topic)]
        optimization_id: u64,
        gas_savings: u64,
        cost_savings: Balance,
        applied_by: AccountId,
    }

    #[ink(event)]
    pub struct ContractUpgradeInitiated {
        #[ink(topic)]
        contract_id: u64,
        from_version: String,
        to_version: String,
        implementation_address: AccountId,
        upgrade_delay: u64,
    }

    #[ink(event)]
    pub struct ContractUpgradeCompleted {
        #[ink(topic)]
        contract_id: u64,
        new_version: String,
        gas_used: u64,
        cost: Balance,
        executed_by: AccountId,
    }

    #[ink(event)]
    pub struct GasOptimizationApplied {
        #[ink(topic)]
        optimization_id: u64,
        function_name: String,
        gas_savings: u64,
        optimization_type: GasOptimizationType,
        applied_by: AccountId,
    }

    #[ink(event)]
    pub struct ParallelProcessStarted {
        #[ink(topic)]
        process_id: u64,
        process_type: ParallelProcessType,
        total_operations: u32,
        estimated_gas: u64,
        started_by: AccountId,
    }

    #[ink(event)]
    pub struct ParallelProcessCompleted {
        #[ink(topic)]
        process_id: u64,
        completed_operations: u32,
        failed_operations: u32,
        total_gas_used: u64,
        execution_time: u64,
    }

    #[ink(event)]
    pub struct PerformanceMetricsUpdated {
        #[ink(topic)]
        metrics_id: u64,
        contract_address: AccountId,
        optimization_score: u16,
        performance_rating: PerformanceRating,
        total_gas_used: u64,
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
                // DeFi Integration (Phase 6)
                total_flash_loans: 0,
                flash_loans: Mapping::default(),
                total_nft_collateral: 0,
                nft_collateral: Mapping::default(),
                total_cross_chain_bridges: 0,
                cross_chain_bridges: Mapping::default(),
                total_cross_chain_transfers: 0,
                cross_chain_transfers: Mapping::default(),
                total_staking_pools: 0,
                staking_pools: Mapping::default(),
                total_staking_positions: 0,
                staking_positions: Mapping::default(),
                user_staking_positions: Mapping::default(),
                total_liquidity_mining_campaigns: 0,
                liquidity_mining_campaigns: Mapping::default(),
                total_liquidity_mining_positions: 0,
                liquidity_mining_positions: Mapping::default(),
                user_liquidity_mining_positions: Mapping::default(),
                // Governance & DAO (Phase 7)
                total_governance_tokens: 0,
                governance_tokens: Mapping::default(),
                total_proposals: 0,
                governance_proposals: Mapping::default(),
                total_votes: 0,
                votes: Mapping::default(),
                user_votes: Mapping::default(),
                total_treasuries: 0,
                treasuries: Mapping::default(),
                total_treasury_transactions: 0,
                treasury_transactions: Mapping::default(),
                total_multi_sig_wallets: 0,
                multi_sig_wallets: Mapping::default(),
                total_multi_sig_transactions: 0,
                multi_sig_transactions: Mapping::default(),
                total_daos: 0,
                dao_configurations: Mapping::default(),
                total_governance_snapshots: 0,
                governance_snapshots: Mapping::default(),
                user_governance_tokens: Mapping::default(),
                user_voting_power: Mapping::default(),
                // Performance & Gas Optimization (Phase 8)
                total_batch_operations: 0,
                batch_operations: Mapping::default(),
                total_storage_optimizations: 0,
                storage_optimizations: Mapping::default(),
                total_upgradeable_contracts: 0,
                upgradeable_contracts: Mapping::default(),
                total_gas_optimizations: 0,
                gas_optimizations: Mapping::default(),
                total_parallel_processes: 0,
                parallel_processes: Mapping::default(),
                total_performance_metrics: 0,
                performance_metrics: Mapping::default(),
                batch_operation_queue: Vec::new(),
                optimization_queue: Vec::new(),
                gas_usage_tracker: Mapping::default(),
                storage_usage_tracker: Mapping::default(),
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

        // ============================================================================
        // DEFI INTEGRATION FUNCTIONS (Phase 6)
        // ============================================================================

        /// Execute a flash loan
        #[ink(message)]
        pub fn execute_flash_loan(
            &mut self,
            asset: AccountId,
            amount: Balance,
            callback_data: Vec<u8>,
            callback_target: AccountId,
        ) -> Result<u64, LendingError> {
            let caller = self.env().caller();
            let current_block = self.env().block_number() as u64;
            
            // Validate parameters
            if amount == 0 {
                return Err(LendingError::InvalidAmount);
            }
            
            // Check if user is blacklisted
            let user_profile = self.get_or_create_user_profile(caller);
            if user_profile.is_blacklisted {
                return Err(LendingError::UserBlacklisted);
            }
            
            // Flash loan fee (typically 0.09% = 9 basis points)
            let fee_rate = 9;
            let fee_amount = (amount * fee_rate as u128) / 10000;
            let total_repay_amount = amount + fee_amount;
            
            let flash_loan_id = self.total_flash_loans + 1;
            
            let flash_loan = FlashLoan {
                id: flash_loan_id,
                borrower: caller,
                asset,
                amount,
                fee_rate,
                fee_amount,
                total_repay_amount,
                status: FlashLoanStatus::Pending,
                created_at: current_block,
                executed_at: None,
                repaid_at: None,
                callback_data,
                callback_target,
            };
            
            self.flash_loans.insert(flash_loan_id, &flash_loan);
            self.total_flash_loans += 1;
            
            // Emit event
            self.env().emit_event(FlashLoanExecuted {
                flash_loan_id,
                borrower: caller,
                asset,
                amount,
                fee_amount,
                callback_target,
            });
            
            Ok(flash_loan_id)
        }

        /// Repay a flash loan
        #[ink(message)]
        pub fn repay_flash_loan(&mut self, flash_loan_id: u64) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let current_block = self.env().block_number() as u64;
            
            let mut flash_loan = self.flash_loans.get(flash_loan_id)
                .ok_or(LendingError::LoanNotFound)?;
            
            if flash_loan.borrower != caller {
                return Err(LendingError::Unauthorized);
            }
            
            if flash_loan.status != FlashLoanStatus::Executed {
                return Err(LendingError::LoanNotActive);
            }
            
            // Update flash loan status
            flash_loan.status = FlashLoanStatus::Repaid;
            flash_loan.repaid_at = Some(current_block);
            
            self.flash_loans.insert(flash_loan_id, &flash_loan);
            
            // Emit event
            self.env().emit_event(FlashLoanRepaid {
                flash_loan_id,
                borrower: caller,
                asset: flash_loan.asset,
                amount: flash_loan.amount,
                fee_amount: flash_loan.fee_amount,
                total_repay_amount: flash_loan.total_repay_amount,
            });
            
            Ok(())
        }

        /// Add NFT as collateral
        #[ink(message)]
        pub fn add_nft_collateral(
            &mut self,
            contract_address: AccountId,
            token_id: u64,
            token_uri: String,
            name: String,
            symbol: String,
            decimals: u8,
            total_supply: u128,
            valuation: Balance,
            rarity_score: u16,
            market_demand: u16,
        ) -> Result<u64, LendingError> {
            let caller = self.env().caller();
            let current_block = self.env().block_number() as u64;
            
            // Validate parameters
            if valuation == 0 || rarity_score > 10000 || market_demand > 10000 {
                return Err(LendingError::InvalidAmount);
            }
            
            let nft_id = self.total_nft_collateral + 1;
            
            let metadata = NFTMetadata {
                token_id,
                contract_address,
                token_uri,
                name,
                symbol,
                decimals,
                total_supply,
            };
            
            let nft_collateral = NFTCollateral {
                nft_id,
                metadata,
                valuation,
                liquidation_threshold: 8000, // 80% default
                maintenance_margin: 12000, // 120% default
                is_verified: true, // Simplified verification
                floor_price: valuation,
                rarity_score,
                market_demand,
                last_valuation_update: current_block,
            };
            
            self.nft_collateral.insert(nft_id, &nft_collateral);
            self.total_nft_collateral += 1;
            
            // Emit event
            self.env().emit_event(NFTCollateralAdded {
                nft_id,
                owner: caller,
                contract_address,
                token_id,
                valuation,
                rarity_score,
            });
            
            Ok(nft_id)
        }

        /// Create a cross-chain bridge
        #[ink(message)]
        pub fn create_cross_chain_bridge(
            &mut self,
            source_chain: u32,
            target_chain: u32,
            source_asset: AccountId,
            target_asset: AccountId,
            bridge_fee: Balance,
            min_transfer: Balance,
            max_transfer: Balance,
        ) -> Result<u64, LendingError> {
            if !self.is_authorized_admin(self.env().caller()) {
                return Err(LendingError::Unauthorized);
            }
            
            let bridge_id = self.total_cross_chain_bridges + 1;
            let current_block = self.env().block_number() as u64;
            
            let bridge = CrossChainBridge {
                bridge_id,
                source_chain,
                target_chain,
                source_asset,
                target_asset,
                bridge_fee,
                min_transfer,
                max_transfer,
                status: BridgeStatus::Active,
                total_volume: 0,
                total_fees_collected: 0,
                last_updated: current_block,
            };
            
            self.cross_chain_bridges.insert(bridge_id, &bridge);
            self.total_cross_chain_bridges += 1;
            
            // Emit event
            self.env().emit_event(CrossChainBridgeCreated {
                bridge_id,
                source_chain,
                target_chain,
                source_asset,
                target_asset,
                bridge_fee,
            });
            
            Ok(bridge_id)
        }

        /// Initiate cross-chain transfer
        #[ink(message)]
        pub fn initiate_cross_chain_transfer(
            &mut self,
            bridge_id: u64,
            target_chain: u32,
            amount: Balance,
        ) -> Result<u64, LendingError> {
            let caller = self.env().caller();
            let current_block = self.env().block_number() as u64;
            
            let bridge = self.cross_chain_bridges.get(bridge_id)
                .ok_or(LendingError::LoanNotFound)?;
            
            if bridge.status != BridgeStatus::Active {
                return Err(LendingError::LoanNotActive);
            }
            
            if amount < bridge.min_transfer || amount > bridge.max_transfer {
                return Err(LendingError::InvalidAmount);
            }
            
            let transfer_id = self.total_cross_chain_transfers + 1;
            
            let transfer = CrossChainTransfer {
                transfer_id,
                user: caller,
                source_chain: 1, // Current chain (simplified)
                target_chain,
                amount,
                bridge_fee: bridge.bridge_fee,
                status: TransferStatus::Pending,
                created_at: current_block,
                completed_at: None,
                transaction_hash: format!("0x{:x}", transfer_id), // Simplified hash
            };
            
            self.cross_chain_transfers.insert(transfer_id, &transfer);
            self.total_cross_chain_transfers += 1;
            
            // Emit event
            self.env().emit_event(CrossChainTransferInitiated {
                transfer_id,
                user: caller,
                source_chain: 1,
                target_chain,
                amount,
                bridge_fee: bridge.bridge_fee,
            });
            
            Ok(transfer_id)
        }

        /// Create a staking pool
        #[ink(message)]
        pub fn create_staking_pool(
            &mut self,
            token: AccountId,
            reward_rate: u16,
            lock_periods: Vec<u64>,
            multipliers: Vec<u16>,
            early_unstake_penalties: Vec<u16>,
            min_stake: Balance,
            max_stake: Balance,
        ) -> Result<u64, LendingError> {
            if !self.is_authorized_admin(self.env().caller()) {
                return Err(LendingError::Unauthorized);
            }
            
            let pool_id = self.total_staking_pools + 1;
            let current_block = self.env().block_number() as u64;
            
            let pool = StakingPool {
                pool_id,
                token,
                total_staked: 0,
                reward_rate,
                lock_periods,
                multipliers,
                early_unstake_penalties,
                min_stake,
                max_stake,
                total_rewards_distributed: 0,
                last_reward_update: current_block,
                is_active: true,
            };
            
            self.staking_pools.insert(pool_id, &pool);
            self.total_staking_pools += 1;
            
            // Emit event
            self.env().emit_event(StakingPoolCreated {
                pool_id,
                token,
                reward_rate,
                min_stake,
                max_stake,
            });
            
            Ok(pool_id)
        }

        /// Open a staking position
        #[ink(message)]
        pub fn open_staking_position(
            &mut self,
            pool_id: u64,
            amount: Balance,
            lock_period_index: u32,
        ) -> Result<u64, LendingError> {
            let caller = self.env().caller();
            let current_block = self.env().block_number() as u64;
            
            let pool = self.staking_pools.get(pool_id)
                .ok_or(LendingError::LoanNotFound)?;
            
            if !pool.is_active {
                return Err(LendingError::LoanNotActive);
            }
            
            if amount < pool.min_stake || amount > pool.max_stake {
                return Err(LendingError::InvalidAmount);
            }
            
            if lock_period_index as usize >= pool.lock_periods.len() {
                return Err(LendingError::InvalidAmount);
            }
            
            let lock_period = pool.lock_periods[lock_period_index as usize];
            let multiplier = pool.multipliers[lock_period_index as usize];
            let _early_unstake_penalty = pool.early_unstake_penalties[lock_period_index as usize];
            
            let position_id = self.total_staking_positions + 1;
            let unlock_time = current_block + lock_period;
            
            let position = StakingPosition {
                staker: caller,
                staked_amount: amount,
                staked_at: current_block,
                last_reward_claim: current_block,
                total_rewards_earned: 0,
                tier_level: format!("Tier_{}", lock_period_index),
                multiplier,
                is_locked: true,
                lock_end_time: unlock_time,
            };
            
            self.staking_positions.insert(position_id, &position);
            self.total_staking_positions += 1;
            
            // Update user positions
            let mut user_positions = self.user_staking_positions.get(caller).unwrap_or(Vec::new());
            user_positions.push(position_id);
            self.user_staking_positions.insert(caller, &user_positions);
            
            // Emit event
            self.env().emit_event(StakingPositionOpened {
                position_id,
                user: caller,
                pool_id,
                amount,
                lock_period: lock_period,
                multiplier,
            });
            
            Ok(position_id)
        }

        /// Create a liquidity mining campaign
        #[ink(message)]
        pub fn create_liquidity_mining_campaign(
            &mut self,
            name: String,
            description: String,
            reward_token: AccountId,
            total_rewards: Balance,
            start_block: u64,
            end_block: u64,
            reward_rate: u16,
            min_stake: Balance,
            max_stake: Balance,
            staking_requirements: Vec<AccountId>,
            bonus_multipliers: Vec<u16>,
        ) -> Result<u64, LendingError> {
            if !self.is_authorized_admin(self.env().caller()) {
                return Err(LendingError::Unauthorized);
            }
            
            let campaign_id = self.total_liquidity_mining_campaigns + 1;
            let _current_block = self.env().block_number() as u64;
            
            let campaign = LiquidityMining {
                campaign_id,
                name: name.clone(),
                description,
                reward_token,
                total_rewards,
                distributed_rewards: 0,
                start_block,
                end_block,
                reward_rate,
                min_stake,
                max_stake,
                staking_requirements,
                bonus_multipliers,
                is_active: true,
                participants_count: 0,
                total_staked: 0,
            };
            
            // Update last reward update to current block
            // Note: current_block is used implicitly in the campaign creation
            
            self.liquidity_mining_campaigns.insert(campaign_id, &campaign);
            self.total_liquidity_mining_campaigns += 1;
            
            // Emit event
            self.env().emit_event(LiquidityMiningCampaignCreated {
                campaign_id,
                name,
                reward_token,
                total_rewards,
                start_block,
                end_block,
            });
            
            Ok(campaign_id)
        }

        /// Open a liquidity mining position
        #[ink(message)]
        pub fn open_liquidity_mining_position(
            &mut self,
            campaign_id: u64,
            amount: Balance,
        ) -> Result<u64, LendingError> {
            let caller = self.env().caller();
            let current_block = self.env().block_number() as u64;
            
            let campaign = self.liquidity_mining_campaigns.get(campaign_id)
                .ok_or(LendingError::LoanNotFound)?;
            
            if !campaign.is_active {
                return Err(LendingError::LoanNotActive);
            }
            
            if current_block < campaign.start_block || current_block > campaign.end_block {
                return Err(LendingError::InvalidAmount);
            }
            
            if amount < campaign.min_stake || amount > campaign.max_stake {
                return Err(LendingError::InvalidAmount);
            }
            
            let position_id = self.total_liquidity_mining_positions + 1;
            let multiplier = if campaign.bonus_multipliers.is_empty() {
                1000 // 1x default
            } else {
                campaign.bonus_multipliers[0] // Use first multiplier
            };
            
            let position = LiquidityMiningPosition {
                position_id,
                user: caller,
                campaign_id,
                staked_amount: amount,
                staked_at: current_block,
                rewards_earned: 0,
                last_claim: current_block,
                multiplier,
                is_active: true,
            };
            
            self.liquidity_mining_positions.insert(position_id, &position);
            self.total_liquidity_mining_positions += 1;
            
            // Update user positions
            let mut user_positions = self.user_liquidity_mining_positions.get(caller).unwrap_or(Vec::new());
            user_positions.push(position_id);
            self.user_liquidity_mining_positions.insert(caller, &user_positions);
            
            // Emit event
            self.env().emit_event(LiquidityMiningPositionOpened {
                position_id,
                user: caller,
                campaign_id,
                staked_amount: amount,
                multiplier,
            });
            
            Ok(position_id)
        }

        // ============================================================================
        // DEFI QUERY FUNCTIONS
        // ============================================================================

        /// Get flash loan information
        #[ink(message)]
        pub fn get_flash_loan(&self, flash_loan_id: u64) -> Result<FlashLoan, LendingError> {
            self.flash_loans.get(flash_loan_id).ok_or(LendingError::LoanNotFound)
        }

        /// Get NFT collateral information
        #[ink(message)]
        pub fn get_nft_collateral(&self, nft_id: u64) -> Result<NFTCollateral, LendingError> {
            self.nft_collateral.get(nft_id).ok_or(LendingError::LoanNotFound)
        }

        /// Get cross-chain bridge information
        #[ink(message)]
        pub fn get_cross_chain_bridge(&self, bridge_id: u64) -> Result<CrossChainBridge, LendingError> {
            self.cross_chain_bridges.get(bridge_id).ok_or(LendingError::LoanNotFound)
        }

        /// Get cross-chain transfer information
        #[ink(message)]
        pub fn get_cross_chain_transfer(&self, transfer_id: u64) -> Result<CrossChainTransfer, LendingError> {
            self.cross_chain_transfers.get(transfer_id).ok_or(LendingError::LoanNotFound)
        }

        /// Get staking pool information
        #[ink(message)]
        pub fn get_staking_pool(&self, pool_id: u64) -> Result<StakingPool, LendingError> {
            self.staking_pools.get(pool_id).ok_or(LendingError::LoanNotFound)
        }

        /// Get staking position information
        #[ink(message)]
        pub fn get_staking_position(&self, position_id: u64) -> Result<StakingPosition, LendingError> {
            self.staking_positions.get(position_id).ok_or(LendingError::LoanNotFound)
        }

        /// Get liquidity mining campaign information
        #[ink(message)]
        pub fn get_liquidity_mining_campaign(&self, campaign_id: u64) -> Result<LiquidityMining, LendingError> {
            self.liquidity_mining_campaigns.get(campaign_id).ok_or(LendingError::LoanNotFound)
        }

        /// Get liquidity mining position information
        #[ink(message)]
        pub fn get_liquidity_mining_position(&self, position_id: u64) -> Result<LiquidityMiningPosition, LendingError> {
            self.liquidity_mining_positions.get(position_id).ok_or(LendingError::LoanNotFound)
        }

        /// Get user staking positions
        #[ink(message)]
        pub fn get_user_staking_positions(&self, user: AccountId) -> Vec<u64> {
            self.user_staking_positions.get(user).unwrap_or(Vec::new())
        }

        /// Get user liquidity mining positions
        #[ink(message)]
        pub fn get_user_liquidity_mining_positions(&self, user: AccountId) -> Vec<u64> {
            self.user_liquidity_mining_positions.get(user).unwrap_or(Vec::new())
        }

        /// Get DeFi statistics
        #[ink(message)]
        pub fn get_defi_statistics(&self) -> (u64, u64, u64, u64, u64, u64) {
            (
                self.total_flash_loans,
                self.total_nft_collateral,
                self.total_cross_chain_bridges,
                self.total_cross_chain_transfers,
                self.total_staking_pools,
                self.total_liquidity_mining_campaigns,
            )
        }

        // ============================================================================
        // GOVERNANCE & DAO FUNCTIONS (Phase 7)
        // ============================================================================

        /// Create a governance token
        #[ink(message)]
        pub fn create_governance_token(
            &mut self,
            name: String,
            symbol: String,
            total_supply: Balance,
            decimals: u8,
            min_stake_for_voting: Balance,
            min_stake_for_proposal: Balance,
            voting_power_multiplier: u16,
            staking_lock_period: u64,
        ) -> Result<u64, LendingError> {
            if !self.is_authorized_admin(self.env().caller()) {
                return Err(LendingError::Unauthorized);
            }
            
            let token_id = self.total_governance_tokens + 1;
            let _current_block = self.env().block_number() as u64;
            
            let token = GovernanceToken {
                token_id,
                name: name.clone(),
                symbol: symbol.clone(),
                total_supply,
                circulating_supply: 0,
                decimals,
                min_stake_for_voting,
                min_stake_for_proposal,
                voting_power_multiplier,
                staking_lock_period,
                is_active: true,
            };
            
            self.governance_tokens.insert(token_id, &token);
            self.total_governance_tokens += 1;
            
            // Emit event
            self.env().emit_event(GovernanceTokenCreated {
                token_id,
                name,
                symbol,
                total_supply,
                min_stake_for_voting,
                min_stake_for_proposal,
            });
            
            Ok(token_id)
        }

        /// Mint governance tokens to a user
        #[ink(message)]
        pub fn mint_governance_tokens(
            &mut self,
            token_id: u64,
            recipient: AccountId,
            amount: Balance,
        ) -> Result<(), LendingError> {
            if !self.is_authorized_admin(self.env().caller()) {
                return Err(LendingError::Unauthorized);
            }
            
            let mut token = self.governance_tokens.get(token_id)
                .ok_or(LendingError::LoanNotFound)?;
            
            if !token.is_active {
                return Err(LendingError::LoanNotActive);
            }
            
            if token.circulating_supply + amount > token.total_supply {
                return Err(LendingError::InvalidAmount);
            }
            
            // Update token supply
            token.circulating_supply += amount;
            self.governance_tokens.insert(token_id, &token);
            
            // Update user balance
            let current_balance = self.user_governance_tokens.get(recipient).unwrap_or(0);
            self.user_governance_tokens.insert(recipient, &(current_balance + amount));
            
            // Update voting power
            let voting_power = (amount * token.voting_power_multiplier as u128) / 1000;
            let current_voting_power = self.user_voting_power.get(recipient).unwrap_or(0);
            self.user_voting_power.insert(recipient, &(current_voting_power + voting_power));
            
            Ok(())
        }

        /// Create a governance proposal
        #[ink(message)]
        pub fn create_governance_proposal(
            &mut self,
            title: String,
            description: String,
            proposal_type: ProposalType,
            target_contract: Option<AccountId>,
            target_function: Option<String>,
            parameters: Vec<u8>,
            value: Balance,
            voting_period: u64,
            execution_delay: u64,
            quorum: Balance,
            threshold: u16,
        ) -> Result<u64, LendingError> {
            let caller = self.env().caller();
            let current_block = self.env().block_number() as u64;
            
            // Check if user has enough governance tokens to create proposal
            let user_balance = self.user_governance_tokens.get(caller).unwrap_or(0);
            let token = self.governance_tokens.get(1).ok_or(LendingError::LoanNotFound)?; // Assume first token
            
            if user_balance < token.min_stake_for_proposal {
                return Err(LendingError::InvalidAmount);
            }
            
            let proposal_id = self.total_proposals + 1;
            let voting_start = current_block;
            let voting_end = current_block + voting_period;
            
            let proposal = GovernanceProposal {
                proposal_id,
                creator: caller,
                title: title.clone(),
                description,
                proposal_type,
                target_contract,
                target_function,
                parameters,
                value,
                voting_start,
                voting_end,
                execution_delay,
                quorum,
                threshold,
                status: ProposalStatus::Active,
                total_votes_for: 0,
                total_votes_against: 0,
                total_votes_abstain: 0,
                executed_at: None,
                executed_by: None,
            };
            
            self.governance_proposals.insert(proposal_id, &proposal);
            self.total_proposals += 1;
            
            // Emit event
            self.env().emit_event(GovernanceProposalCreated {
                proposal_id,
                creator: caller,
                title,
                proposal_type,
                voting_start,
                voting_end,
                quorum,
                threshold,
            });
            
            Ok(proposal_id)
        }

        /// Cast a vote on a governance proposal
        #[ink(message)]
        pub fn cast_vote(
            &mut self,
            proposal_id: u64,
            vote_choice: VoteChoice,
            reason: Option<String>,
        ) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let current_block = self.env().block_number() as u64;
            
            let proposal = self.governance_proposals.get(proposal_id)
                .ok_or(LendingError::LoanNotFound)?;
            
            if proposal.status != ProposalStatus::Active {
                return Err(LendingError::LoanNotActive);
            }
            
            if current_block < proposal.voting_start || current_block > proposal.voting_end {
                return Err(LendingError::InvalidAmount);
            }
            
            // Check if user has already voted
            let user_votes = self.user_votes.get(caller).unwrap_or(Vec::new());
            for vote_id in &user_votes {
                let vote = self.votes.get(*vote_id).unwrap_or_else(|| Vote {
                    voter: AccountId::from([0u8; 32]),
                    proposal_id: 0,
                    vote_choice: VoteChoice::For,
                    voting_power: 0,
                    voted_at: 0,
                    reason: None,
                });
                if vote.proposal_id == proposal_id {
                    return Err(LendingError::InvalidAmount); // Already voted
                }
            }
            
            // Get user's voting power
            let voting_power = self.user_voting_power.get(caller).unwrap_or(0);
            if voting_power == 0 {
                return Err(LendingError::InvalidAmount);
            }
            
            let vote_id = self.total_votes + 1;
            
            let vote = Vote {
                voter: caller,
                proposal_id,
                vote_choice,
                voting_power,
                voted_at: current_block,
                reason: reason.clone(),
            };
            
            self.votes.insert(vote_id, &vote);
            self.total_votes += 1;
            
            // Update user votes
            let mut user_votes = self.user_votes.get(caller).unwrap_or(Vec::new());
            user_votes.push(vote_id);
            self.user_votes.insert(caller, &user_votes);
            
            // Update proposal vote counts
            let mut proposal = self.governance_proposals.get(proposal_id).unwrap();
            match vote_choice {
                VoteChoice::For => proposal.total_votes_for += voting_power,
                VoteChoice::Against => proposal.total_votes_against += voting_power,
                VoteChoice::Abstain => proposal.total_votes_abstain += voting_power,
            }
            
            // Check if proposal should be approved/rejected
            let total_votes = proposal.total_votes_for + proposal.total_votes_against + proposal.total_votes_abstain;
            if total_votes >= proposal.quorum {
                let approval_percentage = (proposal.total_votes_for * 10000) / total_votes;
                if approval_percentage >= proposal.threshold.into() {
                    proposal.status = ProposalStatus::Approved;
                } else {
                    proposal.status = ProposalStatus::Rejected;
                }
            }
            
            self.governance_proposals.insert(proposal_id, &proposal);
            
            // Emit event
            self.env().emit_event(VoteCast {
                proposal_id,
                voter: caller,
                vote_choice,
                voting_power,
                reason,
            });
            
            Ok(())
        }

        /// Execute an approved governance proposal
        #[ink(message)]
        pub fn execute_proposal(&mut self, proposal_id: u64) -> Result<(), LendingError> {
            let caller = self.env().caller();
            let current_block = self.env().block_number() as u64;
            
            let mut proposal = self.governance_proposals.get(proposal_id)
                .ok_or(LendingError::LoanNotFound)?;
            
            if proposal.status != ProposalStatus::Approved {
                return Err(LendingError::LoanNotActive);
            }
            
            if current_block < proposal.voting_end + proposal.execution_delay {
                return Err(LendingError::InvalidAmount); // Execution delay not met
            }
            
            // Execute the proposal based on type
            match proposal.proposal_type {
                ProposalType::ParameterChange => {
                    // Handle parameter changes
                    // This would typically call internal functions to update parameters
                },
                ProposalType::FunctionCall => {
                    // Handle function calls
                    // This would typically call external contracts
                },
                ProposalType::TreasurySpend => {
                    // Handle treasury spending
                    // This would typically create a treasury transaction
                },
                ProposalType::EmergencyAction => {
                    // Handle emergency actions
                    // This would typically have special execution logic
                },
                ProposalType::GovernanceUpdate => {
                    // Handle governance updates
                    // This would typically update governance parameters
                },
                ProposalType::ContractUpgrade => {
                    // Handle contract upgrades
                    // This would typically trigger upgrade logic
                },
            }
            
            // Mark proposal as executed
            proposal.status = ProposalStatus::Executed;
            proposal.executed_at = Some(current_block);
            proposal.executed_by = Some(caller);
            
            self.governance_proposals.insert(proposal_id, &proposal);
            
            // Emit event
            self.env().emit_event(ProposalExecuted {
                proposal_id,
                executor: caller,
                executed_at: current_block,
            });
            
            Ok(())
        }

        /// Create a treasury
        #[ink(message)]
        pub fn create_treasury(
            &mut self,
            name: String,
            description: String,
            daily_spend_limit: Balance,
            monthly_spend_limit: Balance,
            required_signatures: u32,
            authorized_spenders: Vec<AccountId>,
        ) -> Result<u64, LendingError> {
            if !self.is_authorized_admin(self.env().caller()) {
                return Err(LendingError::Unauthorized);
            }
            
            let treasury_id = self.total_treasuries + 1;
            let current_block = self.env().block_number() as u64;
            
            let treasury = Treasury {
                treasury_id,
                name: name.clone(),
                description,
                total_balance: 0,
                daily_spend_limit,
                monthly_spend_limit,
                required_signatures,
                authorized_spenders,
                pending_transactions: Vec::new(),
                transaction_history: Vec::new(),
                last_updated: current_block,
            };
            
            self.treasuries.insert(treasury_id, &treasury);
            self.total_treasuries += 1;
            
            // Emit event
            self.env().emit_event(TreasuryCreated {
                treasury_id,
                name,
                daily_spend_limit,
                monthly_spend_limit,
                required_signatures,
            });
            
            Ok(treasury_id)
        }

        /// Create a multi-signature wallet
        #[ink(message)]
        pub fn create_multi_signature_wallet(
            &mut self,
            name: String,
            description: String,
            owners: Vec<AccountId>,
            required_signatures: u32,
            daily_limit: Balance,
        ) -> Result<u64, LendingError> {
            if !self.is_authorized_admin(self.env().caller()) {
                return Err(LendingError::Unauthorized);
            }
            
            if owners.is_empty() || required_signatures == 0 || required_signatures > owners.len() as u32 {
                return Err(LendingError::InvalidAmount);
            }
            
            let wallet_id = self.total_multi_sig_wallets + 1;
            let current_block = self.env().block_number() as u64;
            
            let wallet = MultiSignatureWallet {
                wallet_id,
                name: name.clone(),
                description,
                owners,
                required_signatures,
                daily_limit,
                total_balance: 0,
                pending_transactions: Vec::new(),
                transaction_history: Vec::new(),
                is_active: true,
                created_at: current_block,
            };
            
            self.multi_sig_wallets.insert(wallet_id, &wallet);
            self.total_multi_sig_wallets += 1;
            
            // Emit event
            self.env().emit_event(MultiSignatureWalletCreated {
                wallet_id,
                name,
                owners_count: wallet.owners.len() as u32,
                required_signatures,
                daily_limit,
            });
            
            Ok(wallet_id)
        }

        /// Create a DAO configuration
        #[ink(message)]
        pub fn create_dao(
            &mut self,
            name: String,
            description: String,
            governance_token: AccountId,
            treasury: u64,
            multi_sig_wallet: u64,
            proposal_creation_threshold: Balance,
            voting_period: u64,
            execution_delay: u64,
            quorum_percentage: u16,
            approval_threshold: u16,
            emergency_threshold: u16,
            max_active_proposals: u32,
        ) -> Result<u64, LendingError> {
            if !self.is_authorized_admin(self.env().caller()) {
                return Err(LendingError::Unauthorized);
            }
            
            let dao_id = self.total_daos + 1;
            let current_block = self.env().block_number() as u64;
            
            let dao = DAOConfiguration {
                dao_id,
                name: name.clone(),
                description,
                governance_token,
                treasury,
                multi_sig_wallet,
                proposal_creation_threshold,
                voting_period,
                execution_delay,
                quorum_percentage,
                approval_threshold,
                emergency_threshold,
                max_active_proposals,
                is_active: true,
                created_at: current_block,
            };
            
            self.dao_configurations.insert(dao_id, &dao);
            self.total_daos += 1;
            
            // Emit event
            self.env().emit_event(DAOCreated {
                dao_id,
                name,
                governance_token,
                treasury,
                multi_sig_wallet,
                proposal_creation_threshold,
            });
            
            Ok(dao_id)
        }

        // ============================================================================
        // GOVERNANCE QUERY FUNCTIONS
        // ============================================================================

        /// Get governance token information
        #[ink(message)]
        pub fn get_governance_token(&self, token_id: u64) -> Result<GovernanceToken, LendingError> {
            self.governance_tokens.get(token_id).ok_or(LendingError::LoanNotFound)
        }

        /// Get governance proposal information
        #[ink(message)]
        pub fn get_governance_proposal(&self, proposal_id: u64) -> Result<GovernanceProposal, LendingError> {
            self.governance_proposals.get(proposal_id).ok_or(LendingError::LoanNotFound)
        }

        /// Get user's governance token balance
        #[ink(message)]
        pub fn get_user_governance_tokens(&self, user: AccountId) -> Balance {
            self.user_governance_tokens.get(user).unwrap_or(0)
        }

        /// Get user's voting power
        #[ink(message)]
        pub fn get_user_voting_power(&self, user: AccountId) -> Balance {
            self.user_voting_power.get(user).unwrap_or(0)
        }

        /// Get treasury information
        #[ink(message)]
        pub fn get_treasury(&self, treasury_id: u64) -> Result<Treasury, LendingError> {
            self.treasuries.get(treasury_id).ok_or(LendingError::LoanNotFound)
        }

        /// Get multi-signature wallet information
        #[ink(message)]
        pub fn get_multi_signature_wallet(&self, wallet_id: u64) -> Result<MultiSignatureWallet, LendingError> {
            self.multi_sig_wallets.get(wallet_id).ok_or(LendingError::LoanNotFound)
        }

        /// Get DAO configuration
        #[ink(message)]
        pub fn get_dao_configuration(&self, dao_id: u64) -> Result<DAOConfiguration, LendingError> {
            self.dao_configurations.get(dao_id).ok_or(LendingError::LoanNotFound)
        }

        /// Get governance statistics
        #[ink(message)]
        pub fn get_governance_statistics(&self) -> (u64, u64, u64, u64, u64, u64) {
            (
                self.total_governance_tokens,
                self.total_proposals,
                self.total_votes,
                self.total_treasuries,
                self.total_multi_sig_wallets,
                self.total_daos,
            )
        }

        /// Get active proposals
        #[ink(message)]
        pub fn get_active_proposals(&self) -> Vec<u64> {
            let mut active_proposals = Vec::new();
            for i in 1..=self.total_proposals {
                if let Some(proposal) = self.governance_proposals.get(i) {
                    if proposal.status == ProposalStatus::Active {
                        active_proposals.push(i);
                    }
                }
            }
            active_proposals
        }

        // ============================================================================
        // PERFORMANCE & GAS OPTIMIZATION FUNCTIONS (Phase 8)
        // ============================================================================

        /// Create a batch operation for gas-efficient bulk processing
        #[ink(message)]
        pub fn create_batch_operation(
            &mut self,
            operation_type: BatchOperationType,
            operations: Vec<Vec<u8>>, // Encoded operation data
        ) -> Result<u64, LendingError> {
            if !self.is_authorized_admin(self.env().caller()) {
                return Err(LendingError::Unauthorized);
            }

            let batch_id = self.total_batch_operations + 1;
            let current_block = self.env().block_number() as u64;
            let caller = self.env().caller();

            let mut batch_items = Vec::new();
            let mut total_gas_estimate = 0;

            for (index, operation_data) in operations.iter().enumerate() {
                let item_id = (index as u64) + 1;
                let gas_estimate = operation_data.len() as u64 * 100; // Rough estimate
                total_gas_estimate += gas_estimate;

                let batch_item = BatchItem {
                    item_id,
                    operation_data: operation_data.clone(),
                    gas_estimate,
                    status: BatchItemStatus::Pending,
                    error_message: None,
                    executed_at: None,
                };
                batch_items.push(batch_item);
            }

            let batch_operation = BatchOperation {
                batch_id,
                operation_type,
                operations: batch_items,
                total_gas_used: 0,
                total_cost: 0,
                status: BatchStatus::Pending,
                created_at: current_block,
                completed_at: None,
                error_count: 0,
                success_count: 0,
            };

            self.batch_operations.insert(batch_id, &batch_operation);
            self.total_batch_operations += 1;
            self.batch_operation_queue.push(batch_id);

            // Emit event
            self.env().emit_event(BatchOperationCreated {
                batch_id,
                operation_type,
                total_operations: operations.len() as u32,
                estimated_gas: total_gas_estimate,
                created_by: caller,
            });

            Ok(batch_id)
        }

        /// Execute a batch operation
        #[ink(message)]
        pub fn execute_batch_operation(&mut self, batch_id: u64) -> Result<(), LendingError> {
            if !self.is_authorized_admin(self.env().caller()) {
                return Err(LendingError::Unauthorized);
            }

            let mut batch = self.batch_operations.get(batch_id)
                .ok_or(LendingError::LoanNotFound)?;

            if batch.status != BatchStatus::Pending {
                return Err(LendingError::InvalidStatus);
            }

            batch.status = BatchStatus::Processing;
            self.batch_operations.insert(batch_id, &batch);

            let mut success_count = 0;
            let mut error_count = 0;
            let mut total_gas_used = 0;
            let current_block = self.env().block_number() as u64;

            // Process each operation in the batch
            for item in &mut batch.operations {
                match self.process_batch_item(&item.operation_data, batch.operation_type) {
                    Ok(_) => {
                        item.status = BatchItemStatus::Executed;
                        item.executed_at = Some(current_block);
                        success_count += 1;
                        total_gas_used += item.gas_estimate;
                    }
                    Err(e) => {
                        item.status = BatchItemStatus::Failed;
                        item.error_message = Some(format!("{:?}", e));
                        error_count += 1;
                    }
                }
            }

            // Update batch status
            batch.status = if error_count == 0 {
                BatchStatus::Completed
            } else if success_count > 0 {
                BatchStatus::PartiallyCompleted
            } else {
                BatchStatus::Failed
            };
            batch.total_gas_used = total_gas_used;
            batch.success_count = success_count;
            batch.error_count = error_count;
            batch.completed_at = Some(current_block);
            batch.total_cost = (total_gas_used as u128) * 1; // 1 wei per gas unit

            self.batch_operations.insert(batch_id, &batch);

            // Emit event
            self.env().emit_event(BatchOperationCompleted {
                batch_id,
                total_gas_used,
                success_count,
                error_count,
                total_cost: batch.total_cost,
            });

            Ok(())
        }

        /// Propose a storage optimization
        #[ink(message)]
        pub fn propose_storage_optimization(
            &mut self,
            optimization_type: StorageOptimizationType,
            target_contract: AccountId,
            estimated_savings: u64,
        ) -> Result<u64, LendingError> {
            if !self.is_authorized_admin(self.env().caller()) {
                return Err(LendingError::Unauthorized);
            }

            let optimization_id = self.total_storage_optimizations + 1;
            let current_block = self.env().block_number() as u64;
            let caller = self.env().caller();

            let optimization = StorageOptimization {
                optimization_id,
                optimization_type,
                target_contract,
                old_storage_size: 0, // Will be calculated when applied
                new_storage_size: 0,
                gas_savings: estimated_savings,
                cost_savings: (estimated_savings as u128) * 1, // 1 wei per gas unit
                status: OptimizationStatus::Proposed,
                created_at: current_block,
                applied_at: None,
            };

            self.storage_optimizations.insert(optimization_id, &optimization);
            self.total_storage_optimizations += 1;
            self.optimization_queue.push(optimization_id);

            // Emit event
            self.env().emit_event(StorageOptimizationProposed {
                optimization_id,
                optimization_type,
                target_contract,
                estimated_savings,
                proposed_by: caller,
            });

            Ok(optimization_id)
        }

        /// Apply a storage optimization
        #[ink(message)]
        pub fn apply_storage_optimization(&mut self, optimization_id: u64) -> Result<(), LendingError> {
            if !self.is_authorized_admin(self.env().caller()) {
                return Err(LendingError::Unauthorized);
            }

            let mut optimization = self.storage_optimizations.get(optimization_id)
                .ok_or(LendingError::LoanNotFound)?;

            if optimization.status != OptimizationStatus::Proposed {
                return Err(LendingError::InvalidStatus);
            }

            let current_block = self.env().block_number() as u64;
            let caller = self.env().caller();

            // Calculate current storage size
            optimization.old_storage_size = self.calculate_storage_size();
            
            // Apply optimization based on type
            match optimization.optimization_type {
                StorageOptimizationType::DataCompression => {
                    // Simulate data compression
                    optimization.new_storage_size = optimization.old_storage_size * 80 / 100; // 20% reduction
                }
                StorageOptimizationType::StructureOptimization => {
                    // Simulate structure optimization
                    optimization.new_storage_size = optimization.old_storage_size * 85 / 100; // 15% reduction
                }
                StorageOptimizationType::UnusedDataRemoval => {
                    // Simulate unused data removal
                    optimization.new_storage_size = optimization.old_storage_size * 90 / 100; // 10% reduction
                }
                StorageOptimizationType::IndexOptimization => {
                    // Simulate index optimization
                    optimization.new_storage_size = optimization.old_storage_size * 95 / 100; // 5% reduction
                }
                StorageOptimizationType::CacheImplementation => {
                    // Simulate cache implementation
                    optimization.new_storage_size = optimization.old_storage_size * 88 / 100; // 12% reduction
                }
            }

            optimization.gas_savings = optimization.old_storage_size - optimization.new_storage_size;
            optimization.cost_savings = (optimization.gas_savings as u128) * 1; // 1 wei per gas unit
            optimization.status = OptimizationStatus::Applied;
            optimization.applied_at = Some(current_block);

            self.storage_optimizations.insert(optimization_id, &optimization);

            // Emit event
            self.env().emit_event(StorageOptimizationApplied {
                optimization_id,
                gas_savings: optimization.gas_savings,
                cost_savings: optimization.cost_savings,
                applied_by: caller,
            });

            Ok(())
        }

        /// Create an upgradeable contract
        #[ink(message)]
        pub fn create_upgradeable_contract(
            &mut self,
            current_version: String,
            upgrade_proxy: AccountId,
            implementation_address: AccountId,
            upgrade_delay: u64,
        ) -> Result<u64, LendingError> {
            if !self.is_authorized_admin(self.env().caller()) {
                return Err(LendingError::Unauthorized);
            }

            let contract_id = self.total_upgradeable_contracts + 1;
            let current_block = self.env().block_number() as u64;
            let caller = self.env().caller();

            let contract = UpgradeableContract {
                contract_id,
                current_version,
                upgrade_proxy,
                implementation_address,
                admin_address: caller,
                upgrade_history: Vec::new(),
                is_upgradeable: true,
                upgrade_delay,
                created_at: current_block,
            };

            self.upgradeable_contracts.insert(contract_id, &contract);
            self.total_upgradeable_contracts += 1;

            Ok(contract_id)
        }

        /// Initiate a contract upgrade
        #[ink(message)]
        pub fn initiate_contract_upgrade(
            &mut self,
            contract_id: u64,
            new_version: String,
            new_implementation: AccountId,
            upgrade_reason: String,
        ) -> Result<(), LendingError> {
            if !self.is_authorized_admin(self.env().caller()) {
                return Err(LendingError::Unauthorized);
            }

            let mut contract = self.upgradeable_contracts.get(contract_id)
                .ok_or(LendingError::LoanNotFound)?;

            if !contract.is_upgradeable {
                return Err(LendingError::InvalidStatus);
            }

            let _current_block = self.env().block_number() as u64;
            let caller = self.env().caller();

            // Create upgrade record
            let upgrade = ContractUpgrade {
                upgrade_id: contract.upgrade_history.len() as u64 + 1,
                from_version: contract.current_version.clone(),
                to_version: new_version.clone(),
                implementation_address: new_implementation,
                upgrade_reason,
                gas_used: 0, // Will be set when completed
                cost: 0, // Will be set when completed
                executed_by: caller,
                executed_at: 0, // Will be set when completed
            };

            contract.upgrade_history.push(upgrade);
            contract.current_version = new_version.clone();
            contract.implementation_address = new_implementation;

            self.upgradeable_contracts.insert(contract_id, &contract);

            // Emit event
            self.env().emit_event(ContractUpgradeInitiated {
                contract_id,
                from_version: contract.upgrade_history.last().unwrap().from_version.clone(),
                to_version: new_version,
                implementation_address: new_implementation,
                upgrade_delay: contract.upgrade_delay,
            });

            Ok(())
        }

        /// Apply gas optimization to a function
        #[ink(message)]
        pub fn apply_gas_optimization(
            &mut self,
            function_name: String,
            optimization_type: GasOptimizationType,
            estimated_savings: u64,
        ) -> Result<u64, LendingError> {
            if !self.is_authorized_admin(self.env().caller()) {
                return Err(LendingError::Unauthorized);
            }

            let optimization_id = self.total_gas_optimizations + 1;
            let current_block = self.env().block_number() as u64;
            let caller = self.env().caller();

            let old_gas_usage = self.gas_usage_tracker.get(&function_name).unwrap_or(0);
            let new_gas_usage = if old_gas_usage > estimated_savings {
                old_gas_usage - estimated_savings
            } else {
                0
            };

            let optimization = GasOptimization {
                optimization_id,
                function_name: function_name.clone(),
                old_gas_usage,
                new_gas_usage,
                gas_savings: estimated_savings,
                optimization_type,
                status: OptimizationStatus::Applied,
                created_at: current_block,
                applied_at: Some(current_block),
            };

            self.gas_optimizations.insert(optimization_id, &optimization);
            self.total_gas_optimizations += 1;

            // Update gas usage tracker
            self.gas_usage_tracker.insert(&function_name, &new_gas_usage);

            // Emit event
            self.env().emit_event(GasOptimizationApplied {
                optimization_id,
                function_name,
                gas_savings: estimated_savings,
                optimization_type,
                applied_by: caller,
            });

            Ok(optimization_id)
        }

        /// Start a parallel processing operation
        #[ink(message)]
        pub fn start_parallel_process(
            &mut self,
            process_type: ParallelProcessType,
            operations: Vec<Vec<u8>>, // Encoded operation data
        ) -> Result<u64, LendingError> {
            if !self.is_authorized_admin(self.env().caller()) {
                return Err(LendingError::Unauthorized);
            }

            let process_id = self.total_parallel_processes + 1;
            let current_block = self.env().block_number() as u64;
            let caller = self.env().caller();

            let mut parallel_operations = Vec::new();
            let mut total_gas_estimate = 0;

            for (index, operation_data) in operations.iter().enumerate() {
                let operation_id = (index as u64) + 1;
                let gas_estimate = operation_data.len() as u64 * 100; // Rough estimate
                total_gas_estimate += gas_estimate;

                let parallel_operation = ParallelOperation {
                    operation_id,
                    operation_type: format!("Operation_{}", operation_id),
                    input_data: operation_data.clone(),
                    output_data: None,
                    gas_used: 0,
                    status: ParallelOperationStatus::Pending,
                    started_at: 0,
                    completed_at: None,
                    error_message: None,
                };
                parallel_operations.push(parallel_operation);
            }

            let parallel_process = ParallelProcessing {
                process_id,
                process_type,
                concurrent_operations: parallel_operations,
                total_operations: operations.len() as u32,
                completed_operations: 0,
                failed_operations: 0,
                gas_used: 0,
                execution_time: 0,
                status: ParallelProcessStatus::Running,
                created_at: current_block,
                completed_at: None,
            };

            self.parallel_processes.insert(process_id, &parallel_process);
            self.total_parallel_processes += 1;

            // Emit event
            self.env().emit_event(ParallelProcessStarted {
                process_id,
                process_type,
                total_operations: operations.len() as u32,
                estimated_gas: total_gas_estimate,
                started_by: caller,
            });

            Ok(process_id)
        }

        /// Complete a parallel processing operation
        #[ink(message)]
        pub fn complete_parallel_process(&mut self, process_id: u64) -> Result<(), LendingError> {
            if !self.is_authorized_admin(self.env().caller()) {
                return Err(LendingError::Unauthorized);
            }

            let mut process = self.parallel_processes.get(process_id)
                .ok_or(LendingError::LoanNotFound)?;

            if process.status != ParallelProcessStatus::Running {
                return Err(LendingError::InvalidStatus);
            }

            let current_block = self.env().block_number() as u64;
            let mut completed_count = 0;
            let mut failed_count = 0;
            let mut total_gas_used = 0;

            // Process each operation
            for operation in &mut process.concurrent_operations {
                operation.started_at = current_block;
                
                match self.process_parallel_operation(&operation.input_data, process.process_type) {
                    Ok(output_data) => {
                        operation.status = ParallelOperationStatus::Completed;
                        operation.output_data = Some(output_data);
                        operation.completed_at = Some(current_block);
                        operation.gas_used = operation.input_data.len() as u64 * 100;
                        completed_count += 1;
                        total_gas_used += operation.gas_used;
                    }
                    Err(_) => {
                        operation.status = ParallelOperationStatus::Failed;
                        operation.error_message = Some("Operation failed".to_string());
                        failed_count += 1;
                    }
                }
            }

            process.status = ParallelProcessStatus::Completed;
            process.completed_operations = completed_count;
            process.failed_operations = failed_count;
            process.gas_used = total_gas_used;
            process.execution_time = current_block - process.created_at;
            process.completed_at = Some(current_block);

            self.parallel_processes.insert(process_id, &process);

            // Emit event
            self.env().emit_event(ParallelProcessCompleted {
                process_id,
                completed_operations: completed_count,
                failed_operations: failed_count,
                total_gas_used,
                execution_time: process.execution_time,
            });

            Ok(())
        }

        /// Update performance metrics
        #[ink(message)]
        pub fn update_performance_metrics(&mut self) -> Result<u64, LendingError> {
            if !self.is_authorized_admin(self.env().caller()) {
                return Err(LendingError::Unauthorized);
            }

            let metrics_id = self.total_performance_metrics + 1;
            let current_block = self.env().block_number() as u64;
            let contract_address = self.env().account_id();

            // Calculate performance metrics
            let total_gas_used = self.calculate_total_gas_usage();
            let total_transactions = self.total_loans + self.total_batch_operations + self.total_parallel_processes;
            let successful_transactions = self.total_loans; // Simplified
            let failed_transactions = total_transactions - successful_transactions;
            let storage_size = self.calculate_storage_size();
            
            let optimization_score = self.calculate_optimization_score();
            let performance_rating = self.calculate_performance_rating(optimization_score);

            let metrics = PerformanceMetrics {
                metrics_id,
                contract_address,
                total_gas_used,
                average_gas_per_operation: if total_transactions > 0 {
                    total_gas_used / total_transactions
                } else {
                    0
                },
                total_transactions,
                successful_transactions,
                failed_transactions,
                storage_size,
                optimization_score,
                performance_rating,
                last_updated: current_block,
            };

            self.performance_metrics.insert(metrics_id, &metrics);
            self.total_performance_metrics += 1;

            // Emit event
            self.env().emit_event(PerformanceMetricsUpdated {
                metrics_id,
                contract_address,
                optimization_score,
                performance_rating,
                total_gas_used,
            });

            Ok(metrics_id)
        }

        // ============================================================================
        // PERFORMANCE OPTIMIZATION HELPER FUNCTIONS
        // ============================================================================

        /// Process a batch item
        fn process_batch_item(&self, operation_data: &[u8], operation_type: BatchOperationType) -> Result<(), LendingError> {
            // Simulate processing based on operation type
            match operation_type {
                BatchOperationType::LoanCreation => {
                    // Simulate loan creation
                    if operation_data.len() < 4 {
                        return Err(LendingError::InvalidAmount);
                    }
                }
                BatchOperationType::LoanRepayment => {
                    // Simulate loan repayment
                    if operation_data.len() < 4 {
                        return Err(LendingError::InvalidAmount);
                    }
                }
                BatchOperationType::UserRegistration => {
                    // Simulate user registration
                    if operation_data.len() < 32 {
                        return Err(LendingError::InvalidAmount);
                    }
                }
                BatchOperationType::CollateralManagement => {
                    // Simulate collateral management
                    if operation_data.len() < 4 {
                        return Err(LendingError::InvalidAmount);
                    }
                }
                BatchOperationType::LiquidityProvision => {
                    // Simulate liquidity provision
                    if operation_data.len() < 4 {
                        return Err(LendingError::InvalidAmount);
                    }
                }
                BatchOperationType::GovernanceVoting => {
                    // Simulate governance voting
                    if operation_data.len() < 4 {
                        return Err(LendingError::InvalidAmount);
                    }
                }
                BatchOperationType::TreasuryOperations => {
                    // Simulate treasury operations
                    if operation_data.len() < 4 {
                        return Err(LendingError::InvalidAmount);
                    }
                }
                BatchOperationType::MultiSigTransactions => {
                    // Simulate multi-sig transactions
                    if operation_data.len() < 4 {
                        return Err(LendingError::InvalidAmount);
                    }
                }
            }
            Ok(())
        }

        /// Process a parallel operation
        fn process_parallel_operation(&self, operation_data: &[u8], process_type: ParallelProcessType) -> Result<Vec<u8>, LendingError> {
            // Simulate parallel processing based on process type
            match process_type {
                ParallelProcessType::BatchLoanProcessing => {
                    // Simulate batch loan processing
                    if operation_data.len() < 4 {
                        return Err(LendingError::InvalidAmount);
                    }
                    Ok(vec![1, 2, 3, 4]) // Simulated output
                }
                ParallelProcessType::ConcurrentUserOperations => {
                    // Simulate concurrent user operations
                    if operation_data.len() < 4 {
                        return Err(LendingError::InvalidAmount);
                    }
                    Ok(vec![5, 6, 7, 8]) // Simulated output
                }
                ParallelProcessType::ParallelAnalytics => {
                    // Simulate parallel analytics
                    if operation_data.len() < 4 {
                        return Err(LendingError::InvalidAmount);
                    }
                    Ok(vec![9, 10, 11, 12]) // Simulated output
                }
                ParallelProcessType::MultiPoolOperations => {
                    // Simulate multi-pool operations
                    if operation_data.len() < 4 {
                        return Err(LendingError::InvalidAmount);
                    }
                    Ok(vec![13, 14, 15, 16]) // Simulated output
                }
                ParallelProcessType::GovernanceBatchProcessing => {
                    // Simulate governance batch processing
                    if operation_data.len() < 4 {
                        return Err(LendingError::InvalidAmount);
                    }
                    Ok(vec![17, 18, 19, 20]) // Simulated output
                }
            }
        }

        /// Calculate total gas usage
        fn calculate_total_gas_usage(&self) -> u64 {
            // Simplified calculation since Mapping doesn't have iter()
            // In a real implementation, you would track this separately
            self.total_gas_optimizations * 1000 // Rough estimate
        }

        /// Calculate storage size
        fn calculate_storage_size(&self) -> u64 {
            // Simplified storage size calculation
            let base_size = 1000; // Base contract size
            let loan_size = self.total_loans * 100; // Approximate size per loan
            let user_size = self.total_users * 50; // Approximate size per user
            let pool_size = self.total_pools * 80; // Approximate size per pool
            let governance_size = self.total_proposals * 60; // Approximate size per proposal
            let batch_size = self.total_batch_operations * 40; // Approximate size per batch
            let optimization_size = self.total_storage_optimizations * 30; // Approximate size per optimization
            
            base_size + loan_size + user_size + pool_size + governance_size + batch_size + optimization_size
        }

        /// Calculate optimization score
        fn calculate_optimization_score(&self) -> u16 {
            let mut score = 500; // Base score of 50%
            
            // Add points for optimizations
            score += self.total_storage_optimizations as u16 * 50; // 5% per optimization
            score += self.total_gas_optimizations as u16 * 30; // 3% per gas optimization
            score += self.total_batch_operations as u16 * 20; // 2% per batch operation
            
            // Cap at 1000 (100%)
            if score > 1000 {
                score = 1000;
            }
            
            score
        }

        /// Calculate performance rating
        fn calculate_performance_rating(&self, optimization_score: u16) -> PerformanceRating {
            match optimization_score {
                900..=1000 => PerformanceRating::Excellent,
                700..=899 => PerformanceRating::Good,
                500..=699 => PerformanceRating::Average,
                300..=499 => PerformanceRating::Poor,
                _ => PerformanceRating::Critical,
            }
        }

        // ============================================================================
        // PERFORMANCE OPTIMIZATION QUERY FUNCTIONS
        // ============================================================================

        /// Get batch operation information
        #[ink(message)]
        pub fn get_batch_operation(&self, batch_id: u64) -> Result<BatchOperation, LendingError> {
            self.batch_operations.get(batch_id).ok_or(LendingError::LoanNotFound)
        }

        /// Get storage optimization information
        #[ink(message)]
        pub fn get_storage_optimization(&self, optimization_id: u64) -> Result<StorageOptimization, LendingError> {
            self.storage_optimizations.get(optimization_id).ok_or(LendingError::LoanNotFound)
        }

        /// Get upgradeable contract information
        #[ink(message)]
        pub fn get_upgradeable_contract(&self, contract_id: u64) -> Result<UpgradeableContract, LendingError> {
            self.upgradeable_contracts.get(contract_id).ok_or(LendingError::LoanNotFound)
        }

        /// Get gas optimization information
        #[ink(message)]
        pub fn get_gas_optimization(&self, optimization_id: u64) -> Result<GasOptimization, LendingError> {
            self.gas_optimizations.get(optimization_id).ok_or(LendingError::LoanNotFound)
        }

        /// Get parallel process information
        #[ink(message)]
        pub fn get_parallel_process(&self, process_id: u64) -> Result<ParallelProcessing, LendingError> {
            self.parallel_processes.get(process_id).ok_or(LendingError::LoanNotFound)
        }

        /// Get performance metrics
        #[ink(message)]
        pub fn get_performance_metrics(&self, metrics_id: u64) -> Result<PerformanceMetrics, LendingError> {
            self.performance_metrics.get(metrics_id).ok_or(LendingError::LoanNotFound)
        }

        /// Get performance statistics
        #[ink(message)]
        pub fn get_performance_statistics(&self) -> (u64, u64, u64, u64, u64, u64) {
            (
                self.total_batch_operations,
                self.total_storage_optimizations,
                self.total_upgradeable_contracts,
                self.total_gas_optimizations,
                self.total_parallel_processes,
                self.total_performance_metrics,
            )
        }

        /// Get batch operation queue
        #[ink(message)]
        pub fn get_batch_operation_queue(&self) -> Vec<u64> {
            self.batch_operation_queue.clone()
        }

        /// Get optimization queue
        #[ink(message)]
        pub fn get_optimization_queue(&self) -> Vec<u64> {
            self.optimization_queue.clone()
        }

        /// Get gas usage for a specific function
        #[ink(message)]
        pub fn get_function_gas_usage(&self, function_name: String) -> u64 {
            self.gas_usage_tracker.get(&function_name).unwrap_or(0)
        }

        /// Get storage usage for a specific data structure
        #[ink(message)]
        pub fn get_storage_usage(&self, data_structure: String) -> u64 {
            self.storage_usage_tracker.get(&data_structure).unwrap_or(0)
        }
    }
} 