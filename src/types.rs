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

pub type AccountId = <ink_env::DefaultEnvironment as ink_env::Environment>::AccountId;
pub type Balance = <ink_env::DefaultEnvironment as ink_env::Environment>::Balance;
pub type BlockNumber = u64; 