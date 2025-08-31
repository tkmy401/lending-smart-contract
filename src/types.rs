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
    pub interest_rate_type: InterestRateType, // Fixed or variable interest rate
    pub base_interest_rate: u16, // Base interest rate for variable loans
    pub risk_multiplier: u16, // Risk-based multiplier (1000 = 1.0x, 1200 = 1.2x)
    pub interest_rate_adjustments: Vec<InterestRateAdjustment>, // History of rate changes
    pub last_interest_update: u64, // Block number of last interest rate update
    pub interest_update_frequency: u64, // How often interest rates can be updated (blocks)
    pub interest_type: InterestType, // Simple or compound interest
    pub compound_frequency: CompoundFrequency, // How often interest compounds
    pub last_compound_date: u64, // Block number of last compound calculation
    pub compound_period_blocks: u64, // Blocks per compound period
    pub accrued_interest: Balance, // Interest accrued since last compound
    pub total_compounded_interest: Balance, // Total interest from compounding
    pub payment_structure: PaymentStructure, // Type of payment structure
    pub interest_only_periods: u32, // Total interest-only periods allowed
    pub current_payment_period: u32, // Current payment period number
    pub interest_only_periods_used: u32, // Interest-only periods already used
    pub next_payment_due: u64, // Block number when next payment is due
    pub payment_period_blocks: u64, // Blocks per payment period
    pub minimum_payment_amount: Balance, // Minimum payment required per period
    pub grace_period_blocks: u64, // Grace period in blocks (configurable)
    pub grace_period_used: u64, // How much of grace period has been used
    pub grace_period_extensions: u32, // Number of grace period extensions used
    pub max_grace_period_extensions: u32, // Maximum grace period extensions allowed
    pub grace_period_reason: GracePeriodReason, // Reason for grace period
    pub grace_period_history: Vec<GracePeriodRecord>, // History of grace period usage
    pub liquidity_pool_id: Option<u64>, // Associated liquidity pool
    pub pool_share: u16, // Share of the pool (basis points)
    pub liquidity_provider: Option<AccountId>, // Who provided the liquidity
    pub pool_rewards_earned: Balance, // Rewards earned from pool participation
    pub credit_score: Option<CreditScore>, // Borrower's credit score
    pub collateral_requirements: Vec<CollateralRequirement>, // Collateral requirements
    pub insurance_policies: Vec<InsurancePolicy>, // Insurance coverage
    pub fraud_flags: Vec<FraudDetectionRule>, // Fraud detection flags
    pub compliance_status: ComplianceStatus, // Compliance verification status
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
pub enum InterestRateType {
    Fixed,      // Fixed interest rate throughout loan term
    Variable,   // Variable interest rate that can change
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct InterestRateAdjustment {
    pub timestamp: u64, // Block number when adjustment occurred
    pub old_rate: u16,  // Previous interest rate
    pub new_rate: u16,  // New interest rate
    pub reason: RateAdjustmentReason, // Reason for the adjustment
    pub risk_score_change: Option<i16>, // Change in risk score if applicable
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum RateAdjustmentReason {
    MarketConditions,    // General market conditions
    RiskScoreChange,     // Borrower risk score changed
    CreditRatingUpdate,  // Credit rating updated
    MarketVolatility,    // High market volatility
    ManualAdjustment,    // Manual adjustment by lender
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum PaymentType {
    Partial,
    Full,
    Early,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum InterestType {
    Simple,     // Simple interest (principal × rate × time)
    Compound,   // Compound interest (principal × (1 + rate)^time)
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum CompoundFrequency {
    Daily,      // Compound every day (14400 blocks)
    Weekly,     // Compound every week (100800 blocks)
    Monthly,    // Compound every month (432000 blocks)
    Quarterly,  // Compound every quarter (1296000 blocks)
    Annually,   // Compound every year (5184000 blocks)
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum PaymentStructure {
    PrincipalAndInterest,
    InterestOnly,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum GracePeriodReason {
    None,                    // No grace period
    FirstTimeBorrower,       // New borrower benefit
    GoodPaymentHistory,      // Reward for good payment history
    MarketConditions,        // Market volatility considerations
    LenderDiscretion,        // Lender's discretionary grace period
    EmergencyCircumstances,  // Special circumstances
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct GracePeriodRecord {
    pub timestamp: u64,           // When grace period was granted
    pub reason: GracePeriodReason, // Reason for grace period
    pub duration: u64,            // Duration in blocks
    pub extension_number: u32,    // Which extension this was
    pub granted_by: AccountId,    // Who granted the grace period
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct LiquidityPool {
    pub id: u64,
    pub name: String,
    pub total_liquidity: Balance,
    pub active_loans: u32,
    pub total_volume: Balance,
    pub pool_fee_rate: u16, // Pool fee in basis points
    pub reward_rate: u16,   // Reward rate in basis points
    pub min_liquidity: Balance,
    pub max_liquidity: Balance,
    pub created_at: u64,
    pub status: PoolStatus,
    pub liquidity_providers: Vec<LiquidityProvider>,
    pub total_rewards_distributed: Balance,
    pub performance_score: u16, // Pool performance score (0-10000)
    pub last_rebalance: u64, // Block number of last rebalance
    pub rebalance_frequency: u64, // How often to rebalance (blocks)
    pub target_liquidity_ratio: u16, // Target liquidity ratio (basis points)
    pub current_liquidity_ratio: u16, // Current liquidity ratio (basis points)
    pub rebalance_threshold: u16, // Threshold for triggering rebalance
    pub auto_rebalance_enabled: bool, // Whether auto-rebalancing is enabled
    pub yield_farming_enabled: bool, // Whether yield farming is enabled
    pub reward_tokens: Vec<RewardToken>, // Supported reward tokens
    pub staking_requirements: StakingRequirements, // Staking requirements for yield farming
    pub tier_multipliers: Vec<TierMultiplier>, // Reward tier multipliers
    pub total_staked_tokens: Balance, // Total tokens staked for yield farming
    pub market_depth_levels: Vec<MarketDepthLevel>, // Market depth at different price levels
    pub optimal_distribution: OptimalDistribution, // Optimal liquidity distribution settings
    pub depth_based_pricing: bool, // Whether depth-based pricing is enabled
    pub concentration_limits: ConcentrationLimits, // Limits to prevent over-concentration
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct LiquidityProvider {
    pub account: AccountId,
    pub liquidity_provided: Balance,
    pub pool_share: u16, // Share in basis points
    pub rewards_earned: Balance,
    pub joined_at: u64,
    pub last_reward_claim: u64,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum PoolStatus {
    Active,
    Paused,
    Closed,
    Emergency,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct RewardToken {
    pub token_address: AccountId,
    pub symbol: String,
    pub decimals: u8,
    pub reward_rate: u16, // Reward rate in basis points
    pub total_distributed: Balance,
    pub is_active: bool,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct StakingRequirements {
    pub min_stake_amount: Balance,
    pub lock_period: u64, // Minimum staking period in blocks
    pub early_unstake_penalty: u16, // Penalty for early unstaking (basis points)
    pub max_stake_amount: Balance,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct TierMultiplier {
    pub tier_name: String,
    pub min_stake_amount: Balance,
    pub multiplier: u16, // Multiplier in basis points (1000 = 1x)
    pub bonus_rewards: u16, // Additional bonus rewards (basis points)
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct StakingPosition {
    pub staker: AccountId,
    pub staked_amount: Balance,
    pub staked_at: u64,
    pub last_reward_claim: u64,
    pub total_rewards_earned: Balance,
    pub tier_level: String,
    pub multiplier: u16,
    pub is_locked: bool,
    pub lock_end_time: u64,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct MarketDepthLevel {
    pub price_level: u16, // Price level in basis points (1000 = 100%)
    pub liquidity_available: Balance, // Available liquidity at this price level
    pub order_count: u32, // Number of orders at this price level
    pub last_updated: u64, // Block number when this level was last updated
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct OptimalDistribution {
    pub target_depth_spread: u16, // Target spread across depth levels (basis points)
    pub min_depth_per_level: Balance, // Minimum liquidity per depth level
    pub max_depth_per_level: Balance, // Maximum liquidity per depth level
    pub rebalancing_threshold: u16, // Threshold for triggering rebalancing
    pub auto_optimization_enabled: bool, // Whether auto-optimization is enabled
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct ConcentrationLimits {
    pub max_single_pool_concentration: u16, // Maximum concentration in single pool (basis points)
    pub max_provider_concentration: u16, // Maximum concentration per provider (basis points)
    pub min_pool_diversity: u16, // Minimum number of pools for diversification
    pub concentration_check_frequency: u64, // How often to check concentration (blocks)
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct CreditScore {
    pub score: u16, // Credit score (300-850)
    pub factors: Vec<CreditFactor>, // Factors contributing to score
    pub last_updated: u64, // Block number when score was last updated
    pub score_history: Vec<CreditScoreRecord>, // Historical score changes
    pub risk_level: RiskLevel, // Risk level based on score
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct CreditFactor {
    pub factor_type: CreditFactorType,
    pub weight: u16, // Weight in basis points (1000 = 100%)
    pub value: u16, // Factor value
    pub description: String,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct CreditScoreRecord {
    pub score: u16,
    pub change: i16, // Score change from previous
    pub reason: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct CollateralRequirement {
    pub collateral_type: CollateralType,
    pub required_amount: Balance, // Required collateral amount
    pub current_amount: Balance, // Current collateral provided
    pub liquidation_threshold: u16, // Liquidation threshold in basis points
    pub maintenance_margin: u16, // Maintenance margin in basis points
    pub last_updated: u64,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct InsurancePolicy {
    pub policy_id: u64,
    pub insured_amount: Balance, // Amount insured
    pub premium_rate: u16, // Premium rate in basis points
    pub coverage_period: u64, // Coverage period in blocks
    pub deductible: Balance, // Deductible amount
    pub status: InsuranceStatus,
    pub created_at: u64,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct FraudDetectionRule {
    pub rule_id: u64,
    pub rule_type: FraudRuleType,
    pub threshold: u16, // Threshold value
    pub action: FraudAction, // Action to take when triggered
    pub is_active: bool,
    pub description: String,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct ComplianceRecord {
    pub record_id: u64,
    pub user_id: AccountId,
    pub compliance_type: ComplianceType,
    pub status: ComplianceStatus,
    pub verification_date: u64,
    pub expiry_date: u64,
    pub documents: Vec<String>, // Document references
}

// Enums for risk management
#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum RiskLevel {
    Excellent, // 750-850
    Good,      // 700-749
    Fair,      // 650-699
    Poor,      // 600-649
    VeryPoor,  // 300-599
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum CreditFactorType {
    PaymentHistory,      // Payment history weight
    CreditUtilization,   // Credit utilization ratio
    CreditHistoryLength, // Length of credit history
    NewCredit,           // New credit applications
    CreditMix,           // Types of credit used
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum CollateralType {
    Stablecoin,    // USDC, DAI, etc.
    Cryptocurrency, // BTC, ETH, etc.
    NFT,           // Non-fungible tokens
    RealEstate,    // Real estate tokens
    Commodities,   // Gold, silver tokens
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum InsuranceStatus {
    Active,
    Expired,
    Cancelled,
    Claimed,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum FraudRuleType {
    UnusualActivity,    // Unusual transaction patterns
    MultipleAccounts,   // Multiple account creation
    RapidTransactions,  // Rapid transaction sequences
    GeographicAnomaly, // Geographic location anomalies
    AmountThreshold,    // Amount threshold violations
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum FraudAction {
    Flag,           // Flag for review
    Block,          // Block transaction
    RequireKYC,     // Require additional KYC
    FreezeAccount,  // Freeze account temporarily
    Report,         // Report to authorities
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum ComplianceType {
    KYC,            // Know Your Customer
    AML,            // Anti-Money Laundering
    Identity,       // Identity verification
    Address,        // Address verification
    SourceOfFunds,  // Source of funds verification
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum ComplianceStatus {
    Pending,        // Verification pending
    Verified,       // Successfully verified
    Rejected,       // Verification rejected
    Expired,        // Verification expired
    UnderReview,    // Under manual review
}

pub type AccountId = <ink_env::DefaultEnvironment as ink_env::Environment>::AccountId;
pub type Balance = <ink_env::DefaultEnvironment as ink_env::Environment>::Balance;
pub type BlockNumber = u64; 

// ============================================================================
// ANALYTICS & REPORTING STRUCTURES (Phase 5)
// ============================================================================

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct LoanPerformanceMetrics {
    pub loan_id: u64,
    pub borrower: AccountId,
    pub total_interest_paid: Balance,
    pub total_fees_paid: Balance,
    pub average_daily_balance: Balance,
    pub days_to_repayment: u64,
    pub payment_efficiency: u16, // 0-10000 (0-100%)
    pub risk_adjusted_return: u16, // 0-10000 (0-100%)
    pub collateral_utilization: u16, // 0-10000 (0-100%)
    pub late_payment_count: u32,
    pub extension_count: u32,
    pub refinance_count: u32,
    pub performance_score: u16, // 0-10000 (0-100%)
    pub last_updated: u64,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PortfolioAnalytics {
    pub user_id: AccountId,
    pub total_portfolio_value: Balance,
    pub active_loans_count: u32,
    pub completed_loans_count: u32,
    pub defaulted_loans_count: u32,
    pub average_loan_size: Balance,
    pub portfolio_diversification_score: u16, // 0-10000 (0-100%)
    pub risk_concentration: u16, // 0-10000 (0-100%)
    pub expected_return: u16, // 0-10000 (0-100%)
    pub volatility_score: u16, // 0-10000 (0-100%)
    pub liquidity_score: u16, // 0-10000 (0-100%)
    pub last_updated: u64,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct MarketStatistics {
    pub total_market_cap: Balance,
    pub total_active_loans: u64,
    pub average_interest_rate: u16,
    pub market_volatility: u16, // 0-10000 (0-100%)
    pub liquidity_depth: u16, // 0-10000 (0-100%)
    pub default_rate: u16, // 0-10000 (0-100%)
    pub utilization_rate: u16, // 0-10000 (0-100%)
    pub market_trend: MarketTrend,
    pub last_updated: u64,
}

#[derive(Debug, Clone, Copy, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum MarketTrend {
    Bullish,
    Bearish,
    Stable,
    Volatile,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct HistoricalDataPoint {
    pub timestamp: u64,
    pub total_loans: u64,
    pub total_volume: Balance,
    pub average_rate: u16,
    pub default_count: u32,
    pub active_users: u32,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PerformanceBenchmark {
    pub benchmark_id: u64,
    pub name: String,
    pub category: BenchmarkCategory,
    pub target_score: u16,
    pub current_score: u16,
    pub weight: u16, // 0-10000 (0-100%)
    pub last_updated: u64,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum BenchmarkCategory {
    LoanPerformance,
    RiskManagement,
    LiquidityEfficiency,
    UserExperience,
    Compliance,
    Overall,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct AnalyticsReport {
    pub report_id: u64,
    pub report_type: ReportType,
    pub generated_at: u64,
    pub data_period: u64, // Blocks
    pub summary: String,
    pub metrics: Vec<AnalyticsMetric>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Copy, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum ReportType {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annual,
    Custom,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct AnalyticsMetric {
    pub name: String,
    pub value: String,
    pub unit: String,
    pub change_from_previous: i32, // Percentage change
    pub trend: MetricTrend,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum MetricTrend {
    Increasing,
    Decreasing,
    Stable,
    Unknown,
} 

// ============================================================================
// DEFI INTEGRATION STRUCTURES (Phase 6)
// ============================================================================

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct FlashLoan {
    pub id: u64,
    pub borrower: AccountId,
    pub asset: AccountId, // Token contract address
    pub amount: Balance,
    pub fee_rate: u16, // Flash loan fee in basis points (typically 9 = 0.09%)
    pub fee_amount: Balance,
    pub total_repay_amount: Balance,
    pub status: FlashLoanStatus,
    pub created_at: u64,
    pub executed_at: Option<u64>,
    pub repaid_at: Option<u64>,
    pub callback_data: Vec<u8>, // Data passed to flash loan callback
    pub callback_target: AccountId, // Contract to call for flash loan logic
}

#[derive(Debug, Clone, Copy, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum FlashLoanStatus {
    Pending,
    Executed,
    Repaid,
    Failed,
    Expired,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct NFTMetadata {
    pub token_id: u64,
    pub contract_address: AccountId,
    pub token_uri: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct NFTCollateral {
    pub nft_id: u64,
    pub metadata: NFTMetadata,
    pub valuation: Balance,
    pub liquidation_threshold: u16, // Basis points
    pub maintenance_margin: u16, // Basis points
    pub is_verified: bool, // NFT authenticity verification
    pub floor_price: Balance,
    pub rarity_score: u16, // 0-10000 (0-100%)
    pub market_demand: u16, // 0-10000 (0-100%)
    pub last_valuation_update: u64,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct CrossChainBridge {
    pub bridge_id: u64,
    pub source_chain: u32,
    pub target_chain: u32,
    pub source_asset: AccountId,
    pub target_asset: AccountId,
    pub bridge_fee: Balance,
    pub min_transfer: Balance,
    pub max_transfer: Balance,
    pub status: BridgeStatus,
    pub total_volume: Balance,
    pub total_fees_collected: Balance,
    pub last_updated: u64,
}

#[derive(Debug, Clone, Copy, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum BridgeStatus {
    Active,
    Paused,
    Maintenance,
    Disabled,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct CrossChainTransfer {
    pub transfer_id: u64,
    pub user: AccountId,
    pub source_chain: u32,
    pub target_chain: u32,
    pub amount: Balance,
    pub bridge_fee: Balance,
    pub status: TransferStatus,
    pub created_at: u64,
    pub completed_at: Option<u64>,
    pub transaction_hash: String,
}

#[derive(Debug, Clone, Copy, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum TransferStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}



#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct StakingPool {
    pub pool_id: u64,
    pub token: AccountId,
    pub total_staked: Balance,
    pub reward_rate: u16, // Basis points per block
    pub lock_periods: Vec<u64>, // Available lock periods
    pub multipliers: Vec<u16>, // Corresponding multipliers
    pub early_unstake_penalties: Vec<u16>, // Corresponding penalties
    pub min_stake: Balance,
    pub max_stake: Balance,
    pub total_rewards_distributed: Balance,
    pub last_reward_update: u64,
    pub is_active: bool,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct LiquidityMining {
    pub campaign_id: u64,
    pub name: String,
    pub description: String,
    pub reward_token: AccountId,
    pub total_rewards: Balance,
    pub distributed_rewards: Balance,
    pub start_block: u64,
    pub end_block: u64,
    pub reward_rate: u16, // Rewards per block
    pub min_stake: Balance,
    pub max_stake: Balance,
    pub staking_requirements: Vec<AccountId>, // Required tokens to stake
    pub bonus_multipliers: Vec<u16>, // Bonus multipliers for different staking levels
    pub is_active: bool,
    pub participants_count: u32,
    pub total_staked: Balance,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct LiquidityMiningPosition {
    pub position_id: u64,
    pub user: AccountId,
    pub campaign_id: u64,
    pub staked_amount: Balance,
    pub staked_at: u64,
    pub rewards_earned: Balance,
    pub last_claim: u64,
    pub multiplier: u16,
    pub is_active: bool,
} 

// ============================================================================
// GOVERNANCE & DAO STRUCTURES (Phase 7)
// ============================================================================

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct GovernanceToken {
    pub token_id: u64,
    pub name: String,
    pub symbol: String,
    pub total_supply: Balance,
    pub circulating_supply: Balance,
    pub decimals: u8,
    pub min_stake_for_voting: Balance,
    pub min_stake_for_proposal: Balance,
    pub voting_power_multiplier: u16, // 1000 = 1x
    pub staking_lock_period: u64, // Blocks required to lock tokens for voting
    pub is_active: bool,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct GovernanceProposal {
    pub proposal_id: u64,
    pub creator: AccountId,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub target_contract: Option<AccountId>,
    pub target_function: Option<String>,
    pub parameters: Vec<u8>, // Encoded function parameters
    pub value: Balance, // ETH value to send with proposal execution
    pub voting_start: u64,
    pub voting_end: u64,
    pub execution_delay: u64, // Blocks to wait after voting ends
    pub quorum: Balance, // Minimum voting power required
    pub threshold: u16, // Percentage required for approval (basis points)
    pub status: ProposalStatus,
    pub total_votes_for: Balance,
    pub total_votes_against: Balance,
    pub total_votes_abstain: Balance,
    pub executed_at: Option<u64>,
    pub executed_by: Option<AccountId>,
}

#[derive(Debug, Clone, Copy, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum ProposalType {
    ParameterChange,    // Change contract parameters
    FunctionCall,       // Execute specific function
    TreasurySpend,      // Spend treasury funds
    EmergencyAction,    // Emergency actions (requires higher threshold)
    GovernanceUpdate,   // Update governance rules
    ContractUpgrade,    // Upgrade contract logic
}

#[derive(Debug, Clone, Copy, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum ProposalStatus {
    Active,
    Approved,
    Rejected,
    Executed,
    Expired,
    Cancelled,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct Vote {
    pub voter: AccountId,
    pub proposal_id: u64,
    pub vote_choice: VoteChoice,
    pub voting_power: Balance,
    pub voted_at: u64,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Copy, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum VoteChoice {
    For,
    Against,
    Abstain,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct Treasury {
    pub treasury_id: u64,
    pub name: String,
    pub description: String,
    pub total_balance: Balance,
    pub daily_spend_limit: Balance,
    pub monthly_spend_limit: Balance,
    pub required_signatures: u32, // Multi-sig requirement
    pub authorized_spenders: Vec<AccountId>,
    pub pending_transactions: Vec<u64>,
    pub transaction_history: Vec<u64>,
    pub last_updated: u64,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct TreasuryTransaction {
    pub transaction_id: u64,
    pub treasury_id: u64,
    pub proposer: AccountId,
    pub recipient: AccountId,
    pub amount: Balance,
    pub purpose: String,
    pub status: TransactionStatus,
    pub approvals: Vec<AccountId>,
    pub created_at: u64,
    pub executed_at: Option<u64>,
    pub executed_by: Option<AccountId>,
}

#[derive(Debug, Clone, Copy, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub enum TransactionStatus {
    Pending,
    Approved,
    Rejected,
    Executed,
    Cancelled,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct MultiSignatureWallet {
    pub wallet_id: u64,
    pub name: String,
    pub description: String,
    pub owners: Vec<AccountId>,
    pub required_signatures: u32,
    pub daily_limit: Balance,
    pub total_balance: Balance,
    pub pending_transactions: Vec<u64>,
    pub transaction_history: Vec<u64>,
    pub is_active: bool,
    pub created_at: u64,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct MultiSigTransaction {
    pub transaction_id: u64,
    pub wallet_id: u64,
    pub proposer: AccountId,
    pub recipient: AccountId,
    pub amount: Balance,
    pub purpose: String,
    pub status: TransactionStatus,
    pub approvals: Vec<AccountId>,
    pub created_at: u64,
    pub executed_at: Option<u64>,
    pub executed_by: Option<AccountId>,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct DAOConfiguration {
    pub dao_id: u64,
    pub name: String,
    pub description: String,
    pub governance_token: AccountId,
    pub treasury: u64,
    pub multi_sig_wallet: u64,
    pub proposal_creation_threshold: Balance,
    pub voting_period: u64, // Blocks
    pub execution_delay: u64, // Blocks
    pub quorum_percentage: u16, // Basis points
    pub approval_threshold: u16, // Basis points
    pub emergency_threshold: u16, // Higher threshold for emergency actions
    pub max_active_proposals: u32,
    pub is_active: bool,
    pub created_at: u64,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct GovernanceSnapshot {
    pub snapshot_id: u64,
    pub proposal_id: u64,
    pub total_voting_power: Balance,
    pub total_participants: u32,
    pub voting_distribution: Vec<VoteDistribution>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct VoteDistribution {
    pub choice: VoteChoice,
    pub total_votes: Balance,
    pub participant_count: u32,
    pub percentage: u16, // Basis points
} 