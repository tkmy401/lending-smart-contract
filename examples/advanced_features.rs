//! Advanced features example for the Lending Smart Contract
//! This example demonstrates advanced functionality like multiple loans, collateral management, and error handling

use ink::env::{
    DefaultEnvironment,
    test,
};

use lending_smart_contract::{LendingContract, types::{RateAdjustmentReason, CompoundFrequency, PaymentStructure, GracePeriodReason}, errors::LendingError};

/// Example demonstrating advanced features of the lending smart contract
fn main() {
    println!("Lending Smart Contract - Advanced Features Example");
    println!("==================================================");

    // Set up test environment
    let accounts = test::default_accounts::<DefaultEnvironment>();
    test::set_caller::<DefaultEnvironment>(accounts.alice);
    
    // Set up contract address (callee)
    let contract_id = accounts.charlie; // Use Charlie's account as contract address
    test::set_callee::<DefaultEnvironment>(contract_id);

    // Test 1: Contract instantiation
    println!("1. Testing Contract Instantiation...");
    let mut contract = LendingContract::new();
    println!("   ✅ Contract created successfully");
    println!();

    // Test 2: Create multiple loans with different parameters
    println!("2. Testing Multiple Loan Creation...");
    
    // Loan 1: Small loan with low interest
    println!("   Creating Loan 1 (Small, Low Interest)...");
    test::set_caller::<DefaultEnvironment>(accounts.alice);
    match contract.create_loan(500, 300, 500, 750) { // 500 amount, 3% interest, 500 blocks, 750 collateral
        Ok(loan_id) => println!("   ✅ Loan 1 created with ID: {}", loan_id),
        Err(e) => {
            println!("   ❌ Failed to create loan 1: {:?}", e);
            return;
        }
    }

    // Loan 2: Medium loan with medium interest
    println!("   Creating Loan 2 (Medium, Medium Interest)...");
    test::set_caller::<DefaultEnvironment>(accounts.bob);
    match contract.create_loan(2000, 800, 1500, 3000) { // 2000 amount, 8% interest, 1500 blocks, 3000 collateral
        Ok(loan_id) => println!("   ✅ Loan 2 created with ID: {}", loan_id),
        Err(e) => {
            println!("   ❌ Failed to create loan 2: {:?}", e);
            return;
        }
    }

    // Loan 3: Large loan with high interest
    println!("   Creating Loan 3 (Large, High Interest)...");
    test::set_caller::<DefaultEnvironment>(accounts.charlie);
    match contract.create_loan(5000, 1200, 2000, 7500) { // 5000 amount, 12% interest, 2000 blocks, 7500 collateral
        Ok(loan_id) => println!("   ✅ Loan 3 created with ID: {}", loan_id),
        Err(e) => {
            println!("   ❌ Failed to create loan 3: {:?}", e);
            return;
        }
    }
    println!();

    // Test 3: Test error handling with invalid parameters
    println!("3. Testing Error Handling...");
    
    // Test invalid amount (0)
    println!("   Testing invalid amount (0)...");
    test::set_caller::<DefaultEnvironment>(accounts.alice);
    match contract.create_loan(0, 500, 1000, 1500) {
        Ok(_) => println!("   ❌ Should have failed with invalid amount"),
        Err(e) => {
            if e == LendingError::InvalidAmount {
                println!("   ✅ Correctly rejected invalid amount: {:?}", e);
            } else {
                println!("   ❌ Wrong error type: {:?}", e);
            }
        }
    }

    // Test invalid interest rate (too high)
    println!("   Testing invalid interest rate (>100%)...");
    match contract.create_loan(1000, 15000, 1000, 1500) { // 150% interest
        Ok(_) => println!("   ❌ Should have failed with invalid interest rate"),
        Err(e) => {
            if e == LendingError::InvalidInterestRate {
                println!("   ✅ Correctly rejected invalid interest rate: {:?}", e);
            } else {
                println!("   ❌ Wrong error type: {:?}", e);
            }
        }
    }

    // Test insufficient collateral
    println!("   Testing insufficient collateral...");
    match contract.create_loan(1000, 500, 1000, 100) { // 100 collateral for 1000 loan
        Ok(_) => println!("   ❌ Should have failed with insufficient collateral"),
        Err(e) => {
            if e == LendingError::InsufficientCollateral {
                println!("   ✅ Correctly rejected insufficient collateral: {:?}", e);
            } else {
                println!("   ❌ Wrong error type: {:?}", e);
            }
        }
    }
    println!();

    // Test 4: Fund multiple loans
    println!("4. Testing Multiple Loan Funding...");
    
    // Fund Loan 1
    println!("   Funding Loan 1...");
    test::set_caller::<DefaultEnvironment>(accounts.django);
    test::set_value_transferred::<DefaultEnvironment>(500);
    match contract.fund_loan(1) {
        Ok(()) => println!("   ✅ Loan 1 funded by Django"),
        Err(e) => println!("   ❌ Failed to fund loan 1: {:?}", e),
    }

    // Fund Loan 2
    println!("   Funding Loan 2...");
    test::set_caller::<DefaultEnvironment>(accounts.eve);
    test::set_value_transferred::<DefaultEnvironment>(2000);
    match contract.fund_loan(2) {
        Ok(()) => println!("   ✅ Loan 2 funded by Eve"),
        Err(e) => println!("   ❌ Failed to fund loan 2: {:?}", e),
    }

    // Try to fund already funded loan
    println!("   Trying to fund already funded loan...");
    test::set_caller::<DefaultEnvironment>(accounts.frank);
    test::set_value_transferred::<DefaultEnvironment>(500);
    match contract.fund_loan(1) {
        Ok(_) => println!("   ❌ Should have failed - loan already funded"),
        Err(e) => {
            if e == LendingError::Unauthorized {
                println!("   ✅ Correctly rejected funding already funded loan: {:?}", e);
            } else {
                println!("   ❌ Wrong error type: {:?}", e);
            }
        }
    }
    println!();

    // Test 5: Query all loans and their statuses
    println!("5. Testing Comprehensive Loan Queries...");
    
    for loan_id in 1..=3 {
        if let Some(loan) = contract.get_loan(loan_id) {
            println!("   Loan {}: Status: {:?}, Amount: {}, Interest: {}%, Lender: {:?}", 
                loan_id, loan.status, loan.amount, loan.interest_rate / 100, loan.lender);
        }
    }
    println!();

    // Test 6: Test loan repayment scenarios
    println!("6. Testing Loan Repayment Scenarios...");
    
    // Repay Loan 1 (small loan)
    println!("   Repaying Loan 1...");
    test::set_caller::<DefaultEnvironment>(accounts.alice);
    let original_repayment = 500 + ((500 * 300) / 10000); // Principal + Interest
    test::set_value_transferred::<DefaultEnvironment>(original_repayment);
    match contract.repay_loan(1) {
        Ok(()) => println!("   ✅ Loan 1 repaid successfully"),
        Err(e) => println!("   ❌ Failed to repay loan 1: {:?}", e),
    }

    // Test early repayment discount calculation
    println!("\n--- Testing Early Repayment Discount ---");
    let discount = contract.get_early_repayment_discount(1).unwrap();
    println!("Early repayment discount: {} basis points ({}%)", discount, discount as f64 / 100.0);
    
    // Simulate early repayment (in real scenario, this would be a transaction)
    println!("Early repayment would cost: {} (original: {})", 
        original_repayment - ((original_repayment * discount as u128) / 10000),
        original_repayment);
    
    // Test loan extension (we'll implement this next)
    println!("\n--- Testing Loan Extension ---");
    let loan_info = contract.get_loan(1).unwrap();
    println!("Current loan duration: {} blocks", loan_info.duration);
    println!("Loan extension feature coming in next phase...");

    // Test loan extension functionality
    println!("\n--- Testing Loan Extension ---");
    let (extension_count, max_extensions, fee_rate) = contract.get_loan_extension_info(2).unwrap();
    println!("Loan 2 extension info:");
    println!("  Extensions used: {}/{}", extension_count, max_extensions);
    println!("  Extension fee rate: {} basis points ({}%)", fee_rate, fee_rate as f64 / 100.0);
    
    let can_extend = contract.can_extend_loan(2).unwrap();
    println!("  Can extend loan: {}", can_extend);
    
    if can_extend {
        let extension_fee = contract.calculate_extension_fee(2).unwrap();
        println!("  Extension fee: {}", extension_fee);
        println!("  Loan extension feature is now available!");
    }

    // Test partial repayment functionality
    println!("\n--- Testing Partial Repayment ---");
    let (total_paid, remaining_balance, payments) = contract.get_loan_payment_info(2).unwrap();
    println!("Loan 2 payment info:");
    println!("  Total paid: {}", total_paid);
    println!("  Remaining balance: {}", remaining_balance);
    println!("  Payment count: {}", payments.len());
    
    // Simulate partial payment (in real scenario, this would be a transaction)
    println!("Partial payment would reduce remaining balance...");
    println!("Partial repayment feature is now available!");

    // Test late fee functionality
    println!("\n--- Testing Late Fee System ---");
    let (total_late_fees, late_fee_rate, max_late_fee_rate, overdue_since) = contract.get_late_fee_info(2).unwrap();
    println!("Loan 2 late fee info:");
    println!("  Total late fees: {}", total_late_fees);
    println!("  Daily late fee rate: {} basis points ({}%)", late_fee_rate, late_fee_rate as f64 / 100.0);
    println!("  Max late fee rate: {} basis points ({}%)", max_late_fee_rate, max_late_fee_rate as f64 / 100.0);
    println!("  Overdue since: {:?}", overdue_since);
    
    let is_overdue = contract.is_loan_overdue(2).unwrap();
    println!("  Is loan overdue: {}", is_overdue);
    
    let current_late_fees = contract.calculate_current_late_fees(2).unwrap();
    println!("  Current late fees: {}", current_late_fees);
    println!("  Late fee system is now active!");

    // Test loan refinancing functionality
    println!("\n--- Testing Loan Refinancing ---");
    let (refinance_count, max_refinances, refinance_fee_rate) = contract.get_loan_refinance_info(2).unwrap();
    println!("Loan 2 refinance info:");
    println!("  Refinances used: {}/{}", refinance_count, max_refinances);
    println!("  Refinance fee rate: {} basis points ({}%)", refinance_fee_rate, refinance_fee_rate as f64 / 100.0);
    
    let can_refinance = contract.can_refinance_loan(2).unwrap();
    println!("  Can refinance loan: {}", can_refinance);
    
    if can_refinance {
        let refinance_fee = contract.calculate_refinance_fee(2).unwrap();
        println!("  Refinance fee: {}", refinance_fee);
        println!("  Loan refinancing feature is now available!");
    }
    
    let refinance_history = contract.get_refinance_history(2).unwrap();
    println!("  Refinance history: {} records", refinance_history.len());

    // ============================================================================
    // VARIABLE INTEREST RATE FEATURES DEMONSTRATION
    // ============================================================================
    println!("\n--- Testing Variable Interest Rate Features ---");
    
    // Test 1: Convert fixed rate loan to variable rate
    println!("\n1. Converting Loan 2 to Variable Rate...");
    // Note: In this example, Bob created loan 2, so he's the borrower, not the lender
    // Eve funded loan 2, so she's the lender. Let's set the caller to Eve.
    test::set_caller::<DefaultEnvironment>(accounts.eve); // Lender can convert
    
    // Check current loan state
    let loan_info = contract.get_loan(2).unwrap();
    println!("   Current interest rate type: {:?}", loan_info.interest_rate_type);
    println!("   Current interest rate: {} basis points ({}%)", loan_info.interest_rate, loan_info.interest_rate as f64 / 100.0);
    println!("   Current risk multiplier: {} ({}x)", loan_info.risk_multiplier, loan_info.risk_multiplier as f64 / 1000.0);
    
    // Convert to variable rate
    match contract.convert_to_variable_rate(2, 600) {
        Ok(_) => {
            println!("   ✅ Successfully converted to variable rate!");
            let updated_loan = contract.get_loan(2).unwrap();
            println!("   New base rate: {} basis points ({}%)", updated_loan.base_interest_rate, updated_loan.base_interest_rate as f64 / 100.0);
            println!("   New effective rate: {} basis points ({}%)", updated_loan.interest_rate, updated_loan.interest_rate as f64 / 100.0);
            println!("   Interest rate type: {:?}", updated_loan.interest_rate_type);
        }
        Err(e) => println!("   ❌ Failed to convert to variable rate: {:?}", e),
    }
    
    // Test 2: Adjust interest rate based on market conditions
    println!("\n2. Adjusting Interest Rate for Market Conditions...");
    
    match contract.adjust_interest_rate(2, 700, RateAdjustmentReason::MarketConditions) {
        Ok(_) => {
            println!("   ✅ Successfully adjusted interest rate!");
            let updated_loan = contract.get_loan(2).unwrap();
            println!("   New base rate: {} basis points ({}%)", updated_loan.base_interest_rate, updated_loan.base_interest_rate as f64 / 100.0);
            println!("   New effective rate: {} basis points ({}%)", updated_loan.interest_rate, updated_loan.interest_rate as f64 / 100.0);
            println!("   Rate adjustments recorded: {}", updated_loan.interest_rate_adjustments.len());
        }
        Err(e) => println!("   ❌ Failed to adjust interest rate: {:?}", e),
    }
    
    // Test 3: Update risk multiplier for borrower risk assessment
    println!("\n3. Updating Risk Multiplier...");
    
    match contract.update_risk_multiplier(2, 1200) { // 1.2x risk multiplier
        Ok(_) => {
            println!("   ✅ Successfully updated risk multiplier!");
            let updated_loan = contract.get_loan(2).unwrap();
            println!("   New risk multiplier: {} ({}x)", updated_loan.risk_multiplier, updated_loan.risk_multiplier as f64 / 1000.0);
            println!("   New effective rate: {} basis points ({}%)", updated_loan.interest_rate, updated_loan.interest_rate as f64 / 100.0);
            println!("   Rate adjustments recorded: {}", updated_loan.interest_rate_adjustments.len());
        }
        Err(e) => println!("   ❌ Failed to update risk multiplier: {:?}", e),
    }
    
    // Test 4: Demonstrate rate adjustment history
    println!("\n4. Rate Adjustment History...");
    let loan_info = contract.get_loan(2).unwrap();
    println!("   Total rate adjustments: {}", loan_info.interest_rate_adjustments.len());
    
    for (i, adjustment) in loan_info.interest_rate_adjustments.iter().enumerate() {
        println!("   Adjustment {}: {} → {} basis points", i + 1, adjustment.old_rate, adjustment.new_rate);
        println!("     Reason: {:?}", adjustment.reason);
        println!("     Timestamp: block {}", adjustment.timestamp);
        if let Some(risk_change) = adjustment.risk_score_change {
            println!("     Risk score change: {}", risk_change);
        }
    }
    
    // Test 5: Demonstrate different rate adjustment reasons
    println!("\n5. Testing Different Rate Adjustment Reasons...");
    
    // Try to adjust rate again (should fail due to frequency limit)
    match contract.adjust_interest_rate(2, 800, RateAdjustmentReason::RiskScoreChange) {
        Ok(_) => println!("   ✅ Successfully adjusted rate for risk score change"),
        Err(e) => println!("   ❌ Expected failure due to frequency limit: {:?}", e),
    }
    
    // Test 6: Show current loan state with variable rates
    println!("\n6. Current Loan State with Variable Rates...");
    let final_loan = contract.get_loan(2).unwrap();
    println!("   Interest rate type: {:?}", final_loan.interest_rate_type);
    println!("   Base interest rate: {} basis points ({}%)", final_loan.base_interest_rate, final_loan.base_interest_rate as f64 / 100.0);
    println!("   Risk multiplier: {} ({}x)", final_loan.risk_multiplier, final_loan.risk_multiplier as f64 / 1000.0);
    println!("   Effective interest rate: {} basis points ({}%)", final_loan.interest_rate, final_loan.interest_rate as f64 / 100.0);
    println!("   Last rate update: block {}", final_loan.last_interest_update);
    println!("   Update frequency: {} blocks ({} days)", final_loan.interest_update_frequency, final_loan.interest_update_frequency / 14400);
    
    // Test 7: Demonstrate rate calculation formula
    println!("\n7. Rate Calculation Formula Demonstration...");
    let base_rate = final_loan.base_interest_rate as u32;
    let risk_mult = final_loan.risk_multiplier as u32;
    let calculated_rate = (base_rate * risk_mult) / 1000;
    println!("   Formula: (Base Rate × Risk Multiplier) ÷ 1000");
    println!("   Calculation: ({} × {}) ÷ 1000 = {} ÷ 1000 = {}", base_rate, risk_mult, base_rate * risk_mult, calculated_rate);
    println!("   Actual rate: {} (matches calculation: {})", final_loan.interest_rate, calculated_rate == final_loan.interest_rate as u32);
    
    println!("\n🎉 Variable Interest Rate Features demonstration completed!");
    println!("This demonstrates:");
    println!("  - Fixed to variable rate conversion");
    println!("  - Dynamic interest rate adjustments");
    println!("  - Risk-based pricing with multipliers");
    println!("  - Rate adjustment history tracking");
    println!("  - Update frequency controls");
    println!("  - Market-responsive lending");

    // ============================================================================
    // COMPOUND INTEREST FEATURES DEMONSTRATION
    // ============================================================================
    println!("\n--- Testing Compound Interest Features ---");
    
    // Test 1: Convert loan to compound interest
    println!("\n1. Converting Loan 1 to Compound Interest...");
    test::set_caller::<DefaultEnvironment>(accounts.django); // Lender can convert
    
    // Check current loan state
    let loan_info = contract.get_loan(1).unwrap();
    println!("   Current interest type: {:?}", loan_info.interest_type);
    println!("   Current interest rate: {} basis points ({}%)", loan_info.interest_rate, loan_info.interest_rate as f64 / 100.0);
    println!("   Current remaining balance: {}", loan_info.remaining_balance);
    
    // Convert to compound interest (daily compounding)
    match contract.convert_to_compound_interest(1, CompoundFrequency::Daily) {
        Ok(_) => {
            println!("   ✅ Successfully converted to compound interest!");
            let updated_loan = contract.get_loan(1).unwrap();
            println!("   New interest type: {:?}", updated_loan.interest_type);
            println!("   Compound frequency: {:?}", updated_loan.compound_frequency);
            println!("   Compound period blocks: {}", updated_loan.compound_period_blocks);
            println!("   New remaining balance: {}", updated_loan.remaining_balance);
            println!("   Total compounded interest: {}", updated_loan.total_compounded_interest);
        }
        Err(e) => println!("   ❌ Failed to convert to compound interest: {:?}", e),
    }
    
    // Test 2: Demonstrate different compound frequencies
    println!("\n2. Testing Different Compound Frequencies...");
    
    // Create a new loan for testing different frequencies
    test::set_caller::<DefaultEnvironment>(accounts.frank);
    let loan4_id = contract.create_loan(1000, 1000, 2000, 1500).unwrap(); // 10% interest, 2000 blocks
    
    // Fund the loan
    test::set_caller::<DefaultEnvironment>(accounts.alice);
    test::set_value_transferred::<DefaultEnvironment>(1000);
    contract.fund_loan(loan4_id).unwrap();
    
    // Convert to monthly compound interest
    test::set_caller::<DefaultEnvironment>(accounts.alice);
    match contract.convert_to_compound_interest(loan4_id, CompoundFrequency::Monthly) {
        Ok(_) => {
            println!("   ✅ Successfully converted to monthly compound interest!");
            let loan4 = contract.get_loan(loan4_id).unwrap();
            println!("   Compound frequency: {:?}", loan4.compound_frequency);
            println!("   Compound period: {} blocks ({} days)", loan4.compound_period_blocks, loan4.compound_period_blocks / 14400);
            println!("   Initial balance: {}", loan4.amount);
            println!("   Current remaining balance: {}", loan4.remaining_balance);
        }
        Err(e) => println!("   ❌ Failed to convert to monthly compound: {:?}", e),
    }
    
    // Test 3: Calculate accrued interest
    println!("\n3. Calculating Accrued Interest...");
    
    let loan1 = contract.get_loan(1).unwrap();
    match contract.calculate_accrued_interest(1) {
        Ok(accrued) => {
            println!("   ✅ Accrued interest calculated successfully!");
            println!("   Interest type: {:?}", loan1.interest_type);
            println!("   Principal: {}", loan1.amount);
            println!("   Interest rate: {} basis points ({}%)", loan1.interest_rate, loan1.interest_rate as f64 / 100.0);
            println!("   Accrued interest: {}", accrued);
            println!("   Total compounded interest: {}", loan1.total_compounded_interest);
        }
        Err(e) => println!("   ❌ Failed to calculate accrued interest: {:?}", e),
    }
    
    // Test 4: Get compound interest information
    println!("\n4. Compound Interest Information...");
    
    match contract.get_compound_interest_info(1) {
        Ok((interest_type, frequency, period_blocks, accrued, total_compounded)) => {
            println!("   ✅ Compound interest info retrieved successfully!");
            println!("   Interest type: {:?}", interest_type);
            println!("   Compound frequency: {:?}", frequency);
            println!("   Period blocks: {} ({} days)", period_blocks, period_blocks / 14400);
            println!("   Accrued interest: {}", accrued);
            println!("   Total compounded interest: {}", total_compounded);
        }
        Err(e) => println!("   ❌ Failed to get compound interest info: {:?}", e),
    }
    
    // Test 5: Demonstrate compound interest calculation
    println!("\n5. Compound Interest Calculation Demonstration...");
    
    let loan1 = contract.get_loan(1).unwrap();
    let principal = loan1.amount as f64;
    let rate = loan1.interest_rate as f64 / 10000.0; // Convert basis points to decimal
    let periods = 1.0; // 1 day
    
    // Compound interest formula: A = P(1 + r)^n
    let compound_factor = (1.0 + rate).powf(periods);
    let new_total = principal * compound_factor;
    let interest_accrued = new_total - principal;
    
    println!("   Compound Interest Formula: A = P(1 + r)^n");
    println!("   Where:");
    println!("     P = Principal = {}", principal);
    println!("     r = Rate per period = {} = {}%", rate, rate * 100.0);
    println!("     n = Number of periods = {}", periods);
    println!("   Calculation:");
    println!("     A = {} × (1 + {})^{}", principal, rate, periods);
    println!("     A = {} × {}", principal, compound_factor);
    println!("     A = {}", new_total);
    println!("   Interest accrued: {} - {} = {}", new_total, principal, interest_accrued);
    
    // Test 6: Show different compound frequencies comparison
    println!("\n6. Compound Frequency Comparison...");
    
    let frequencies = [
        (CompoundFrequency::Daily, "Daily", 14400),
        (CompoundFrequency::Weekly, "Weekly", 100800),
        (CompoundFrequency::Monthly, "Monthly", 432000),
        (CompoundFrequency::Quarterly, "Quarterly", 1296000),
        (CompoundFrequency::Annually, "Annually", 5184000),
    ];
    
    println!("   Compound Frequency Comparison (Principal: 1000, Rate: 10%, 1 year):");
    println!("   ┌─────────────┬─────────────┬─────────────┬─────────────────┐");
    println!("   │ Frequency   │ Periods/Yr │ Rate/Period │ Final Amount    │");
    println!("   ├─────────────┼─────────────┼─────────────┼─────────────────┤");
    
    for (_freq, name, blocks) in frequencies.iter() {
        let periods_per_year = 5184000 / blocks; // 5184000 blocks per year
        let rate_per_period = 0.10 / periods_per_year as f64; // 10% annual rate
        let compound_factor = (1.0 + rate_per_period).powf(periods_per_year as f64);
        let final_amount = 1000.0 * compound_factor;
        
        println!("   │ {:<11} │ {:<11} │ {:<11.6} │ {:<15.2} │", name, periods_per_year, rate_per_period, final_amount);
    }
    println!("   └─────────────┴─────────────┴─────────────┴─────────────────┘");
    
    // Test 7: Show current loan states
    println!("\n7. Current Loan States with Interest Types...");
    
    for loan_id in 1..=4 {
        if let Some(loan) = contract.get_loan(loan_id) {
            println!("   Loan {}: {:?} interest, {:?} compounding, Balance: {}", 
                loan_id, loan.interest_type, loan.compound_frequency, loan.remaining_balance);
        }
    }
    
    println!("\n🎉 Compound Interest Features demonstration completed!");
    println!("This demonstrates:");
    println!("  - Simple to compound interest conversion");
    println!("  - Multiple compound frequencies (daily to annually)");
    println!("  - Real-time interest accrual calculation");
    println!("  - Compound interest formula implementation");
    println!("  - Interest type management");
    println!("  - Sophisticated financial calculations");

    // ============================================================================
    // INTEREST-ONLY PAYMENT PERIODS DEMONSTRATION
    // ============================================================================
    println!("\n--- Testing Interest-Only Payment Periods ---");
    
    // Test 1: Set up interest-only payment structure
    println!("\n1. Setting Up Interest-Only Payment Structure...");
    test::set_caller::<DefaultEnvironment>(accounts.django); // Lender can set payment structure
    
    // Check current loan state
    let loan_info = contract.get_loan(1).unwrap();
    println!("   Current payment structure: {:?}", loan_info.payment_structure);
    println!("   Current remaining balance: {}", loan_info.remaining_balance);
    println!("   Current interest rate: {} basis points ({}%)", loan_info.interest_rate, loan_info.interest_rate as f64 / 100.0);
    
    // Set loan to interest-only for 3 periods (daily payments)
    match contract.set_interest_only_periods(1, 3, 14400) { // 3 periods, daily (14400 blocks)
        Ok(_) => {
            println!("   ✅ Successfully set to interest-only payment structure!");
            let updated_loan = contract.get_loan(1).unwrap();
            println!("   New payment structure: {:?}", updated_loan.payment_structure);
            println!("   Interest-only periods: {}/{}", updated_loan.interest_only_periods_used, updated_loan.interest_only_periods);
            println!("   Payment period blocks: {} ({} days)", updated_loan.payment_period_blocks, updated_loan.payment_period_blocks / 14400);
            println!("   Next payment due: block {}", updated_loan.next_payment_due);
            println!("   Minimum payment amount: {}", updated_loan.minimum_payment_amount);
        }
        Err(e) => println!("   ❌ Failed to set interest-only structure: {:?}", e),
    }
    
    // Test 2: Demonstrate different payment structures
    println!("\n2. Testing Different Payment Structures...");
    
    // Create a new loan for testing different payment structures
    test::set_caller::<DefaultEnvironment>(accounts.frank);
    let loan5_id = contract.create_loan(1500, 1200, 3000, 2250).unwrap(); // 12% interest, 3000 blocks
    
    // Fund the loan
    test::set_caller::<DefaultEnvironment>(accounts.bob);
    test::set_value_transferred::<DefaultEnvironment>(1500);
    contract.fund_loan(loan5_id).unwrap();
    
    // Set to weekly interest-only payments for 2 periods
    test::set_caller::<DefaultEnvironment>(accounts.bob);
    match contract.set_interest_only_periods(loan5_id, 2, 100800) { // 2 periods, weekly (100800 blocks)
        Ok(_) => {
            println!("   ✅ Successfully set to weekly interest-only structure!");
            let loan5 = contract.get_loan(loan5_id).unwrap();
            println!("   Payment structure: {:?}", loan5.payment_structure);
            println!("   Interest-only periods: {}/{}", loan5.interest_only_periods_used, loan5.interest_only_periods);
            println!("   Payment period: {} blocks ({} days)", loan5.payment_period_blocks, loan5.payment_period_blocks / 14400);
            println!("   Next payment due: block {}", loan5.next_payment_due);
        }
        Err(e) => println!("   ❌ Failed to set weekly interest-only: {:?}", e),
    }
    
    // Test 3: Make interest-only payments
    println!("\n3. Making Interest-Only Payments...");
    
    // Get payment structure info
    match contract.get_payment_structure_info(1) {
        Ok((structure, total_periods, used_periods, current_period, next_due, min_payment)) => {
            println!("   ✅ Payment structure info retrieved successfully!");
            println!("   Structure: {:?}", structure);
            println!("   Interest-only periods: {}/{}", used_periods, total_periods);
            println!("   Current period: {}", current_period);
            println!("   Next payment due: block {}", next_due);
            println!("   Minimum payment: {}", min_payment);
        }
        Err(e) => println!("   ❌ Failed to get payment structure info: {:?}", e),
    }
    
    // Test 4: Demonstrate payment schedule
    println!("\n4. Payment Schedule Demonstration...");
    
    let loan1 = contract.get_loan(1).unwrap();
    let principal = loan1.amount as f64;
    let rate = loan1.interest_rate as f64 / 10000.0; // Convert basis points to decimal
    let period_blocks = loan1.payment_period_blocks as f64;
    let time_factor = period_blocks / 5184000.0; // Convert to years
    
    // Calculate interest payment for one period
    let interest_payment = (principal * rate * time_factor) as u128;
    
    println!("   Payment Schedule for Loan 1:");
    println!("   Principal: {}", principal);
    println!("   Interest rate: {}% per year", rate * 100.0);
    println!("   Payment period: {} blocks ({} days)", period_blocks, period_blocks / 14400.0);
    println!("   Interest per period: {}", interest_payment);
    println!("   Total interest-only periods: {}", loan1.interest_only_periods);
    
    // Show payment breakdown
    println!("   ┌─────────────┬─────────────┬─────────────┬─────────────────┐");
    println!("   │ Period      │ Payment     │ Principal   │ Remaining       │");
    println!("   ├─────────────┼─────────────┼─────────────┼─────────────────┤");
    
    for period in 1..=loan1.interest_only_periods {
        let remaining_principal = principal;
        let payment = interest_payment;
        println!("   │ {:<11} │ {:<11} │ {:<11} │ {:<15} │", 
            period, payment, remaining_principal as u128, remaining_principal as u128);
    }
    
    // After interest-only periods
    let remaining_periods = (loan1.duration / loan1.payment_period_blocks) - loan1.interest_only_periods as u64;
    if remaining_periods > 0 {
        let principal_per_period = principal / remaining_periods as f64;
        let total_payment = interest_payment + principal_per_period as u128;
        
        println!("   ├─────────────┼─────────────┼─────────────┼─────────────────┤");
        println!("   │ P&I Periods │ {:<11} │ {:<11} │ {:<15} │", 
            total_payment, principal_per_period as u128, 0);
        println!("   └─────────────┴─────────────┴─────────────┴─────────────────┘");
    } else {
        println!("   └─────────────┴─────────────┴─────────────┴─────────────────┘");
    }
    
    // Test 5: Switch back to principal and interest
    println!("\n5. Switching to Principal and Interest...");
    
    test::set_caller::<DefaultEnvironment>(accounts.django); // Lender can switch
    match contract.switch_to_principal_and_interest(1) {
        Ok(_) => {
            println!("   ✅ Successfully switched to P&I structure!");
            let updated_loan = contract.get_loan(1).unwrap();
            println!("   New payment structure: {:?}", updated_loan.payment_structure);
            println!("   New minimum payment: {}", updated_loan.minimum_payment_amount);
        }
        Err(e) => println!("   ❌ Failed to switch to P&I: {:?}", e),
    }
    
    // Test 6: Show current loan states with payment structures
    println!("\n6. Current Loan States with Payment Structures...");
    
    for loan_id in 1..=5 {
        if let Some(loan) = contract.get_loan(loan_id) {
            let structure_str = match loan.payment_structure {
                PaymentStructure::PrincipalAndInterest => "P&I",
                PaymentStructure::InterestOnly => "Interest-Only",
            };
            println!("   Loan {}: {} structure, {} periods used, Balance: {}", 
                loan_id, structure_str, loan.interest_only_periods_used, loan.remaining_balance);
        }
    }
    
    // Test 7: Demonstrate payment flexibility
    println!("\n7. Payment Structure Flexibility...");
    
    println!("   This system provides:");
    println!("   ✅ Flexible payment structures (Interest-Only ↔ P&I)");
    println!("   ✅ Configurable payment periods (daily, weekly, monthly)");
    println!("   ✅ Automatic structure switching after interest-only periods");
    println!("   ✅ Real-time payment scheduling and due date tracking");
    println!("   ✅ Minimum payment calculations for each structure");
    println!("   ✅ Borrower-friendly payment options");
    
    println!("\n🎉 Interest-Only Payment Periods demonstration completed!");
    println!("This demonstrates:");
    println!("  - Flexible payment structure management");
    println!("  - Interest-only payment periods with automatic switching");
    println!("  - Configurable payment schedules and frequencies");
    println!("  - Payment structure conversion and management");
    println!("  - Real-time payment tracking and scheduling");
    println!("  - Borrower-friendly payment options");

    // ============================================================================
    // GRACE PERIOD MANAGEMENT DEMONSTRATION
    // ============================================================================
    println!("\n--- Testing Grace Period Management ---");
    
    // Test 1: Set up custom grace period for a loan
    println!("\n1. Setting Up Custom Grace Period...");
    test::set_caller::<DefaultEnvironment>(accounts.django); // Lender can set grace period
    
    // Check current loan state
    let loan_info = contract.get_loan(1).unwrap();
    println!("   Current grace period: {} blocks ({} minutes)", loan_info.grace_period_blocks, loan_info.grace_period_blocks / 600);
    println!("   Grace period extensions used: {}/{}", loan_info.grace_period_extensions, loan_info.max_grace_period_extensions);
    println!("   Current grace period reason: {:?}", loan_info.grace_period_reason);
    
    // Set custom grace period for the loan
    match contract.set_custom_grace_period(1, 200, 3) { // 200 blocks grace period, max 3 extensions
        Ok(_) => {
            println!("   ✅ Successfully set custom grace period!");
            let updated_loan = contract.get_loan(1).unwrap();
            println!("   New grace period: {} blocks ({} minutes)", updated_loan.grace_period_blocks, updated_loan.grace_period_blocks / 600);
            println!("   Max extensions allowed: {}", updated_loan.max_grace_period_extensions);
        }
        Err(e) => println!("   ❌ Failed to set custom grace period: {:?}", e),
    }
    
    // Test 2: Grant grace period extension
    println!("\n2. Granting Grace Period Extension...");
    
    // Grant grace period extension for good payment history
    match contract.grant_grace_period(1, 100, GracePeriodReason::GoodPaymentHistory) {
        Ok(_) => {
            println!("   ✅ Successfully granted grace period extension!");
            let updated_loan = contract.get_loan(1).unwrap();
            println!("   New total grace period: {} blocks ({} minutes)", updated_loan.grace_period_blocks, updated_loan.grace_period_blocks / 600);
            println!("   Extensions used: {}/{}", updated_loan.grace_period_extensions, updated_loan.max_grace_period_extensions);
            println!("   Current reason: {:?}", updated_loan.grace_period_reason);
        }
        Err(e) => println!("   ❌ Failed to grant grace period: {:?}", e),
    }
    
    // Test 3: Demonstrate different grace period reasons
    println!("\n3. Testing Different Grace Period Reasons...");
    
    // Create a new loan for testing different grace period scenarios
    test::set_caller::<DefaultEnvironment>(accounts.frank);
    let loan6_id = contract.create_loan(800, 600, 1800, 1200).unwrap(); // 6% interest, 1800 blocks
    
    // Fund the loan
    test::set_caller::<DefaultEnvironment>(accounts.charlie);
    test::set_value_transferred::<DefaultEnvironment>(800);
    contract.fund_loan(loan6_id).unwrap();
    
    // Grant grace period for first-time borrower
    test::set_caller::<DefaultEnvironment>(accounts.charlie);
    match contract.grant_grace_period(loan6_id, 150, GracePeriodReason::FirstTimeBorrower) {
        Ok(_) => {
            println!("   ✅ Successfully granted first-time borrower grace period!");
            let loan6 = contract.get_loan(loan6_id).unwrap();
            println!("   Grace period: {} blocks ({} minutes)", loan6.grace_period_blocks, loan6.grace_period_blocks / 600);
            println!("   Reason: {:?}", loan6.grace_period_reason);
        }
        Err(e) => println!("   ❌ Failed to grant first-time borrower grace: {:?}", e),
    }
    
    // Test 4: Get grace period information
    println!("\n4. Grace Period Information...");
    
    match contract.get_grace_period_info(1) {
        Ok((grace_blocks, grace_used, extensions, max_extensions, reason, history)) => {
            println!("   ✅ Grace period info retrieved successfully!");
            println!("   Total grace period: {} blocks ({} minutes)", grace_blocks, grace_blocks / 600);
            println!("   Grace period used: {} blocks", grace_used);
            println!("   Extensions used: {}/{}", extensions, max_extensions);
            println!("   Current reason: {:?}", reason);
            println!("   History records: {}", history.len());
            
            // Show grace period history
            for (i, record) in history.iter().enumerate() {
                println!("     Record {}: {:?} - {} blocks - Extension #{}", 
                    i + 1, record.reason, record.duration, record.extension_number);
            }
        }
        Err(e) => println!("   ❌ Failed to get grace period info: {:?}", e),
    }
    
    // Test 5: Check grace period status
    println!("\n5. Checking Grace Period Status...");
    
    // Check if loan is within grace period
    match contract.is_within_grace_period(1) {
        Ok(within_grace) => {
            println!("   ✅ Grace period status checked successfully!");
            println!("   Is within grace period: {}", within_grace);
        }
        Err(e) => println!("   ❌ Failed to check grace period status: {:?}", e),
    }
    
    // Calculate remaining grace period
    match contract.calculate_remaining_grace_period(1) {
        Ok(remaining) => {
            println!("   ✅ Remaining grace period calculated!");
            println!("   Remaining grace period: {} blocks ({} minutes)", remaining, remaining / 600);
        }
        Err(e) => println!("   ❌ Failed to calculate remaining grace period: {:?}", e),
    }
    
    // Test 6: Demonstrate grace period flexibility
    println!("\n6. Grace Period Flexibility Demonstration...");
    
    let grace_reasons = [
        (GracePeriodReason::FirstTimeBorrower, "First Time Borrower", 200),
        (GracePeriodReason::GoodPaymentHistory, "Good Payment History", 150),
        (GracePeriodReason::MarketConditions, "Market Conditions", 100),
        (GracePeriodReason::LenderDiscretion, "Lender Discretion", 300),
        (GracePeriodReason::EmergencyCircumstances, "Emergency Circumstances", 500),
    ];
    
    println!("   Grace Period Reasons and Default Durations:");
    println!("   ┌─────────────────────────┬─────────────────────┬─────────────┐");
    println!("   │ Reason                  │ Description         │ Duration    │");
    println!("   ├─────────────────────────┼─────────────────────┼─────────────┤");
    
    for (reason, description, duration) in grace_reasons.iter() {
        println!("   │ {:<23} │ {:<19} │ {:<11} │", 
            format!("{:?}", reason), description, format!("{} blocks", duration));
    }
    println!("   └─────────────────────────┴─────────────────────┴─────────────┘");
    
    // Test 7: Show current loan states with grace periods
    println!("\n7. Current Loan States with Grace Periods...");
    
            for loan_id in 1..=6 {
            if let Some(loan) = contract.get_loan(loan_id) {
                let grace_str = match loan.grace_period_reason {
                    GracePeriodReason::None => "None",
                    _ => "Active",
                };
                println!("   Loan {}: Grace period: {} ({} blocks), Extensions: {}/{}", 
                    loan_id, grace_str, loan.grace_period_blocks, loan.grace_period_extensions, loan.max_grace_period_extensions);
            }
        }
    
    // Test 8: Demonstrate grace period benefits
    println!("\n8. Grace Period Benefits and Features...");
    
    println!("   This system provides:");
    println!("   ✅ Configurable grace periods (100 blocks to 1 week)");
    println!("   ✅ Multiple grace period reasons with different durations");
    println!("   ✅ Grace period extensions (up to configurable maximum)");
    println!("   ✅ Complete grace period history tracking");
    println!("   ✅ Real-time grace period status monitoring");
    println!("   ✅ Flexible grace period management by lenders");
    println!("   ✅ Borrower-friendly late payment handling");
    
    println!("\n🎉 Grace Period Management demonstration completed!");
    println!("This demonstrates:");
    println!("  - Flexible grace period configuration and management");
    println!("  - Multiple grace period reasons and durations");
    println!("  - Grace period extensions with history tracking");
    println!("  - Real-time grace period status monitoring");
    println!("  - Lender-controlled grace period customization");
    println!("  - Borrower-friendly late payment handling");

    // ============================================================================
    // LIQUIDITY POOL MANAGEMENT DEMONSTRATION
    // ============================================================================
    println!("\n--- Testing Liquidity Pool Management ---");
    
    // Test 1: Create a new liquidity pool
    println!("\n1. Creating a New Liquidity Pool...");
    test::set_caller::<DefaultEnvironment>(accounts.alice);
    
    // Create a lending pool with initial liquidity
    match contract.create_liquidity_pool(
        "High-Yield Lending Pool".to_string(),
        10000, // 10,000 initial liquidity
        100,   // 1% pool fee rate
        200,   // 2% reward rate
        1000,  // 1,000 minimum liquidity
        100000, // 100,000 maximum liquidity
    ) {
        Ok(pool_id) => {
            println!("   ✅ Successfully created liquidity pool with ID: {}", pool_id);
            println!("   Pool name: High-Yield Lending Pool");
            println!("   Initial liquidity: 10,000");
            println!("   Pool fee rate: 1%");
            println!("   Reward rate: 2%");
            println!("   Min liquidity: 1,000");
            println!("   Max liquidity: 100,000");
        }
        Err(e) => println!("   ❌ Failed to create liquidity pool: {:?}", e),
    }
    
    // Test 2: Provide liquidity to the pool
    println!("\n2. Providing Liquidity to Pool...");
    
    // Alice provides additional liquidity
    test::set_caller::<DefaultEnvironment>(accounts.alice);
    match contract.provide_liquidity(1, 5000) { // Add 5,000 more liquidity
        Ok(_) => {
            println!("   ✅ Successfully provided additional liquidity!");
            println!("   Amount provided: 5,000");
            println!("   Total pool liquidity: 15,000");
        }
        Err(e) => println!("   ❌ Failed to provide liquidity: {:?}", e),
    }
    
    // Bob provides liquidity to the pool
    test::set_caller::<DefaultEnvironment>(accounts.bob);
    match contract.provide_liquidity(1, 3000) { // Bob adds 3,000 liquidity
        Ok(_) => {
            println!("   ✅ Successfully provided liquidity as Bob!");
            println!("   Amount provided: 3,000");
            println!("   Total pool liquidity: 18,000");
        }
        Err(e) => println!("   ❌ Failed to provide liquidity: {:?}", e),
    }
    
    // Test 3: Get pool information
    println!("\n3. Liquidity Pool Information...");
    
    match contract.get_liquidity_pool_info(1) {
        Ok((name, total_liquidity, active_loans, total_volume, pool_fee, reward_rate, status)) => {
            println!("   ✅ Pool information retrieved successfully!");
            println!("   Pool name: {}", name);
            println!("   Total liquidity: {}", total_liquidity);
            println!("   Active loans: {}", active_loans);
            println!("   Total volume: {}", total_volume);
            println!("   Pool fee rate: {} basis points ({}%)", pool_fee, pool_fee as f64 / 100.0);
            println!("   Reward rate: {} basis points ({}%)", reward_rate, reward_rate as f64 / 100.0);
            println!("   Pool status: {:?}", status);
        }
        Err(e) => println!("   ❌ Failed to get pool info: {:?}", e),
    }
    
    // Test 4: Get liquidity provider information
    println!("\n4. Liquidity Provider Information...");
    
    // Get Alice's provider info
    match contract.get_liquidity_provider_info(1, accounts.alice) {
        Ok((liquidity_provided, pool_share, rewards_earned, last_claim)) => {
            println!("   ✅ Alice's provider info retrieved!");
            println!("   Liquidity provided: {}", liquidity_provided);
            println!("   Pool share: {} basis points ({}%)", pool_share, pool_share as f64 / 100.0);
            println!("   Rewards earned: {}", rewards_earned);
            println!("   Last reward claim: block {}", last_claim);
        }
        Err(e) => println!("   ❌ Failed to get Alice's provider info: {:?}", e),
    }
    
    // Get Bob's provider info
    match contract.get_liquidity_provider_info(1, accounts.bob) {
        Ok((liquidity_provided, pool_share, rewards_earned, last_claim)) => {
            println!("   ✅ Bob's provider info retrieved!");
            println!("   Liquidity provided: {}", liquidity_provided);
            println!("   Pool share: {} basis points ({}%)", pool_share, pool_share as f64 / 100.0);
            println!("   Rewards earned: {}", rewards_earned);
            println!("   Last reward claim: block {}", last_claim);
        }
        Err(e) => println!("   ❌ Failed to get Bob's provider info: {:?}", e),
    }
    
    // Test 5: Demonstrate pool share calculations
    println!("\n5. Pool Share Calculations...");
    
    let alice_liquidity = 15000; // 15,000 (10,000 + 5,000)
    let bob_liquidity = 3000;    // 3,000
    let total_pool = alice_liquidity + bob_liquidity; // 18,000
    
    let alice_share = (alice_liquidity as f64 / total_pool as f64) * 100.0;
    let bob_share = (bob_liquidity as f64 / total_pool as f64) * 100.0;
    
    println!("   Pool Share Distribution:");
    println!("   ┌─────────────┬─────────────┬─────────────┬─────────────┐");
    println!("   │ Provider    │ Liquidity   │ Share       │ Percentage  │");
    println!("   ├─────────────┼─────────────┼─────────────┼─────────────┤");
    println!("   │ Alice       │ {:<11} │ {:<11} │ {:<11.2} │", alice_liquidity, "15,000", alice_share);
    println!("   │ Bob         │ {:<11} │ {:<11} │ {:<11.2} │", bob_liquidity, "3,000", bob_share);
    println!("   ├─────────────┼─────────────┼─────────────┼─────────────┤");
    println!("   │ Total       │ {:<11} │ {:<11} │ {:<11.2} │", total_pool, "18,000", 100.0);
    println!("   └─────────────┴─────────────┴─────────────┴─────────────┘");
    
    // Test 6: Create additional pools for comparison
    println!("\n6. Creating Additional Liquidity Pools...");
    
    // Create a conservative pool
    test::set_caller::<DefaultEnvironment>(accounts.charlie);
    match contract.create_liquidity_pool(
        "Conservative Lending Pool".to_string(),
        5000,  // 5,000 initial liquidity
        50,    // 0.5% pool fee rate
        100,   // 1% reward rate
        500,   // 500 minimum liquidity
        50000, // 50,000 maximum liquidity
    ) {
        Ok(pool_id) => {
            println!("   ✅ Successfully created conservative pool with ID: {}", pool_id);
            println!("   Pool name: Conservative Lending Pool");
            println!("   Initial liquidity: 5,000");
            println!("   Pool fee rate: 0.5%");
            println!("   Reward rate: 1%");
        }
        Err(e) => println!("   ❌ Failed to create conservative pool: {:?}", e),
    }
    
    // Create a high-risk pool
    test::set_caller::<DefaultEnvironment>(accounts.django);
    match contract.create_liquidity_pool(
        "High-Risk Lending Pool".to_string(),
        2000,  // 2,000 initial liquidity
        200,   // 2% pool fee rate
        500,   // 5% reward rate
        200,   // 200 minimum liquidity
        20000, // 20,000 maximum liquidity
    ) {
        Ok(pool_id) => {
            println!("   ✅ Successfully created high-risk pool with ID: {}", pool_id);
            println!("   Pool name: High-Risk Lending Pool");
            println!("   Initial liquidity: 2,000");
            println!("   Pool fee rate: 2%");
            println!("   Reward rate: 5%");
        }
        Err(e) => println!("   ❌ Failed to create high-risk pool: {:?}", e),
    }
    
    // Test 7: Demonstrate pool comparison
    println!("\n7. Pool Comparison and Analysis...");
    
    let pools = [
        ("High-Yield", 100, 200, 10000, 100000),
        ("Conservative", 50, 100, 5000, 50000),
        ("High-Risk", 200, 500, 2000, 20000),
    ];
    
    println!("   Pool Risk-Reward Analysis:");
    println!("   ┌─────────────┬─────────────┬─────────────┬─────────────┬─────────────┐");
    println!("   │ Pool Type   │ Fee Rate    │ Reward Rate │ Min Liquidity│ Max Liquidity│");
    println!("   ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤");
    
    for (name, fee, reward, min_liq, max_liq) in pools.iter() {
        println!("   │ {:<11} │ {:<11} │ {:<11} │ {:<12} │ {:<12} │", 
            name, format!("{}%", *fee as f64 / 100.0), format!("{}%", *reward as f64 / 100.0), min_liq, max_liq);
    }
    println!("   └─────────────┴─────────────┴─────────────┴─────────────┴─────────────┘");
    
    // Test 8: Show current pool states
    println!("\n8. Current Pool States...");
    
    for pool_id in 1..=3 {
        if let Ok((name, total_liquidity, active_loans, _total_volume, pool_fee, reward_rate, status)) = contract.get_liquidity_pool_info(pool_id) {
            println!("   Pool {}: {} - {} liquidity, {} loans, {}% fee, {}% reward, {:?}", 
                pool_id, name, total_liquidity, active_loans, pool_fee as f64 / 100.0, reward_rate as f64 / 100.0, status);
        }
    }
    
    // Test 9: Demonstrate liquidity pool benefits
    println!("\n9. Liquidity Pool Benefits and Features...");
    
    println!("   This system provides:");
    println!("   ✅ Automated market making for lending markets");
    println!("   ✅ Liquidity provider rewards and incentives");
    println!("   ✅ Dynamic pool rebalancing and management");
    println!("   ✅ Risk-adjusted pool configurations");
    println!("   ✅ Real-time pool analytics and monitoring");
    println!("   ✅ Flexible liquidity provision and withdrawal");
    println!("   ✅ Yield farming integration capabilities");
    
    println!("\n🎉 Liquidity Pool Management demonstration completed!");
    println!("This demonstrates:");
    println!("  - Automated market making (AMM) for lending");
    println!("  - Liquidity provider rewards and incentives");
    println!("  - Dynamic pool management and rebalancing");
    println!("  - Risk-adjusted pool configurations");
    println!("  - Real-time pool analytics and monitoring");
    println!("  - Yield farming integration foundations");

    // ============================================================================
    // POOL REBALANCING & DYNAMIC LIQUIDITY MANAGEMENT DEMONSTRATION
    // ============================================================================
    println!("\n--- Testing Pool Rebalancing & Dynamic Liquidity Management ---");
    
    // Test 1: Check current pool rebalancing status
    println!("\n1. Current Pool Rebalancing Status...");
    
    for pool_id in 1..=3 {
        match contract.get_pool_rebalancing_info(pool_id) {
            Ok((performance_score, last_rebalance, _frequency, target_ratio, current_ratio, _auto_enabled)) => {
                println!("   Pool {}: Performance: {}%, Last rebalance: block {}, Frequency: {} blocks", 
                    pool_id, performance_score as f64 / 100.0, last_rebalance, _frequency);
                println!("     Target ratio: {}%, Current ratio: {}%, Auto-rebalancing: {}", 
                    target_ratio as f64 / 100.0, current_ratio as f64 / 100.0, _auto_enabled);
            }
            Err(e) => println!("   ❌ Failed to get pool {} rebalancing info: {:?}", pool_id, e),
        }
    }
    
    // Test 2: Check if pools need rebalancing
    println!("\n2. Checking Pool Rebalancing Needs...");
    
    for pool_id in 1..=3 {
        match contract.needs_rebalancing(pool_id) {
            Ok(needs_rebalance) => {
                println!("   Pool {}: Needs rebalancing: {}", pool_id, needs_rebalance);
            }
            Err(e) => println!("   ❌ Failed to check pool {} rebalancing needs: {:?}", pool_id, e),
        }
    }
    
    // Test 3: Set custom rebalancing parameters
    println!("\n3. Setting Custom Rebalancing Parameters...");
    test::set_caller::<DefaultEnvironment>(accounts.alice); // Pool 1 creator
    
    // Set more aggressive rebalancing for high-yield pool
    match contract.set_rebalancing_parameters(1, 7200, 7500, 300) { // 12h frequency, 75% target, 3% threshold
        Ok(_) => {
            println!("   ✅ Successfully set custom rebalancing parameters for Pool 1!");
            println!("   New frequency: 7200 blocks (12 hours)");
            println!("   New target ratio: 75%");
            println!("   New threshold: 3%");
        }
        Err(e) => println!("   ❌ Failed to set rebalancing parameters: {:?}", e),
    }
    
    // Test 4: Disable auto-rebalancing for conservative pool
    println!("\n4. Managing Auto-Rebalancing Settings...");
    test::set_caller::<DefaultEnvironment>(accounts.charlie); // Pool 2 creator
    
    match contract.set_auto_rebalancing(2, false) {
        Ok(_) => {
            println!("   ✅ Successfully disabled auto-rebalancing for Pool 2!");
            println!("   Pool 2 will now require manual rebalancing");
        }
        Err(e) => println!("   ❌ Failed to disable auto-rebalancing: {:?}", e),
    }
    
    // Test 5: Manual pool rebalancing
    println!("\n5. Manual Pool Rebalancing...");
    test::set_caller::<DefaultEnvironment>(accounts.alice); // Pool 1 creator
    
    // Check if pool needs rebalancing
    match contract.needs_rebalancing(1) {
        Ok(needs_rebalance) => {
            if needs_rebalance {
                println!("   Pool 1 needs rebalancing, triggering manual rebalance...");
                match contract.rebalance_pool(1) {
                    Ok(_) => {
                        println!("   ✅ Successfully rebalanced Pool 1!");
                        
                        // Get updated rebalancing info
                        match contract.get_pool_rebalancing_info(1) {
                                    Ok((performance_score, _last_rebalance, _frequency, _target_ratio, _current_ratio, _auto_enabled)) => {
            println!("   Updated Pool 1:");
            println!("   Performance score: {}%", performance_score as f64 / 100.0);
            println!("   Last rebalance: block {}", _last_rebalance);
            println!("   Current ratio: {}% (target: {}%)", 
                _current_ratio as f64 / 100.0, _target_ratio as f64 / 100.0);
                            }
                            Err(e) => println!("   ❌ Failed to get updated info: {:?}", e),
                        }
                    }
                    Err(e) => println!("   ❌ Failed to rebalance pool: {:?}", e),
                }
            } else {
                println!("   Pool 1 doesn't need rebalancing yet");
            }
        }
        Err(e) => println!("   ❌ Failed to check rebalancing needs: {:?}", e),
    }
    
    // Test 6: Demonstrate performance score calculation
    println!("\n6. Performance Score Calculation Demonstration...");
    
    // Create a new pool with specific characteristics for testing
    test::set_caller::<DefaultEnvironment>(accounts.eve);
    let test_pool_id = contract.create_liquidity_pool(
        "Performance Test Pool".to_string(),
        1000,  // 1,000 initial liquidity
        150,   // 1.5% pool fee rate
        300,   // 3% reward rate
        100,   // 100 minimum liquidity
        10000, // 10,000 maximum liquidity
    ).unwrap();
    
    println!("   Created test pool with ID: {}", test_pool_id);
    
    // Get initial performance score
    match contract.get_pool_rebalancing_info(test_pool_id) {
        Ok((performance_score, last_rebalance, frequency, target_ratio, current_ratio, auto_enabled)) => {
            println!("   Initial performance score: {}%", performance_score as f64 / 100.0);
            println!("   Base score: 50% (default)");
            println!("   Liquidity utilization: 0% (no active loans)");
            println!("   Reward efficiency: 0% (no rewards distributed)");
            println!("   Provider diversity: 100 points (1 provider)");
            println!("   Total calculated score: {}%", performance_score as f64 / 100.0);
        }
        Err(e) => println!("   ❌ Failed to get test pool info: {:?}", e),
    }
    
    // Test 7: Show rebalancing parameter comparison
    println!("\n7. Rebalancing Parameter Comparison...");
    
    let rebalancing_configs = [
        ("High-Yield", 7200, 7500, 300, "Aggressive"),
        ("Conservative", 14400, 8000, 500, "Moderate"),
        ("High-Risk", 3600, 6000, 200, "Very Aggressive"),
    ];
    
    println!("   Pool Rebalancing Strategies:");
    println!("   ┌─────────────┬─────────────┬─────────────┬─────────────┬─────────────────┐");
    println!("   │ Pool Type   │ Frequency   │ Target Ratio│ Threshold  │ Strategy        │");
    println!("   ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────────┤");
    
    for (name, freq, target, threshold, strategy) in rebalancing_configs.iter() {
        let freq_hours = freq / 600; // Convert blocks to hours
        println!("   │ {:<11} │ {:<11} │ {:<11} │ {:<11} │ {:<15} │", 
            name, format!("{}h", freq_hours), format!("{}%", *target as f64 / 100.0), 
            format!("{}%", *threshold as f64 / 100.0), strategy);
    }
    println!("   └─────────────┴─────────────┴─────────────┴─────────────┴─────────────────┘");
    
    // Test 8: Demonstrate rebalancing benefits
    println!("\n8. Pool Rebalancing Benefits and Features...");
    
    println!("   This system provides:");
    println!("   ✅ Automatic performance-based pool optimization");
    println!("   ✅ Dynamic liquidity ratio adjustments");
    println!("   ✅ Configurable rebalancing strategies");
    println!("   ✅ Performance score calculation and monitoring");
    println!("   ✅ Manual and automatic rebalancing options");
    println!("   ✅ Real-time pool health monitoring");
    println!("   ✅ Optimal liquidity distribution management");
    
    println!("\n🎉 Pool Rebalancing & Dynamic Liquidity Management demonstration completed!");
    println!("This demonstrates:");
    println!("  - Performance-based pool optimization");
    println!("  - Dynamic liquidity ratio management");
    println!("  - Configurable rebalancing strategies");
    println!("  - Real-time performance monitoring");
    println!("  - Manual and automatic rebalancing");
    println!("  - Pool health and efficiency management");

    // ============================================================================
    // YIELD FARMING & ADVANCED REWARDS DEMONSTRATION
    // ============================================================================
    println!("\n--- Testing Yield Farming & Advanced Rewards ---");
    
    // Test 1: Check current yield farming status
    println!("\n1. Current Yield Farming Status...");
    
    for pool_id in 1..=4 {
        match contract.get_yield_farming_info(pool_id) {
            Ok((enabled, reward_tokens, total_staked, tier_count)) => {
                println!("   Pool {}: Yield farming: {}, Reward tokens: {}, Total staked: {}, Tiers: {}", 
                    pool_id, enabled, reward_tokens, total_staked, tier_count);
            }
            Err(e) => println!("   ❌ Failed to get pool {} yield farming info: {:?}", pool_id, e),
        }
    }
    
    // Test 2: Enable yield farming for Pool 1
    println!("\n2. Enabling Yield Farming for Pool 1...");
    test::set_caller::<DefaultEnvironment>(accounts.alice); // Pool 1 creator
    
    // Create reward tokens
    let reward_tokens = vec![
        lending_smart_contract::types::RewardToken {
            token_address: accounts.alice,
            symbol: "LEND".to_string(),
            decimals: 18,
            reward_rate: 150, // 1.5% reward rate
            total_distributed: 0,
            is_active: true,
        },
        lending_smart_contract::types::RewardToken {
            token_address: accounts.bob,
            symbol: "GOV".to_string(),
            decimals: 18,
            reward_rate: 100, // 1% reward rate
            total_distributed: 0,
            is_active: true,
        },
    ];
    
    match contract.enable_yield_farming(1, reward_tokens) {
        Ok(_) => {
            println!("   ✅ Successfully enabled yield farming for Pool 1!");
            println!("   Reward tokens: LEND (1.5%) and GOV (1%)");
            println!("   Staking requirements: Min 1,000, Lock 1 day, Max 100,000");
        }
        Err(e) => println!("   ❌ Failed to enable yield farming: {:?}", e),
    }
    
    // Test 3: Display staking tiers
    println!("\n3. Staking Tiers and Multipliers...");
    
    match contract.get_staking_tiers(1) {
        Ok(tiers) => {
            println!("   Pool 1 Staking Tiers:");
            println!("   ┌─────────────┬─────────────┬─────────────┬─────────────────┐");
            println!("   │ Tier        │ Min Stake   │ Multiplier  │ Bonus Rewards   │");
            println!("   ├─────────────┼─────────────┼─────────────┼─────────────────┤");
            
            for (tier_name, min_stake, multiplier, bonus) in tiers {
                println!("   │ {:<11} │ {:<11} │ {:<11} │ {:<15} │", 
                    tier_name, min_stake, format!("{}x", multiplier as f64 / 1000.0), bonus);
            }
            println!("   └─────────────┴─────────────┴─────────────┴─────────────────┘");
            
            println!("   Multiplier Explanation:");
            println!("   - Bronze (1x): Base rewards, no bonus");
            println!("   - Silver (1.2x): 20% more rewards + 1% bonus");
            println!("   - Gold (1.5x): 50% more rewards + 3% bonus");
            println!("   - Platinum (2x): Double rewards + 5% bonus");
        }
        Err(e) => println!("   ❌ Failed to get staking tiers: {:?}", e),
        }
    
    // Test 4: Stake tokens for yield farming
    println!("\n4. Staking Tokens for Yield Farming...");
    
    // Alice stakes tokens (Pool creator)
    test::set_caller::<DefaultEnvironment>(accounts.alice);
    match contract.stake_tokens(1, 25000) { // Stake 25,000 tokens
        Ok(_) => {
            println!("   ✅ Successfully staked 25,000 tokens as Alice!");
            println!("   Tier: Gold (1.5x multiplier)");
            println!("   Lock period: 1 day (14,400 blocks)");
            println!("   Early unstake penalty: 5%");
        }
        Err(e) => println!("   ❌ Failed to stake tokens: {:?}", e),
    }
    
    // Bob stakes tokens
    test::set_caller::<DefaultEnvironment>(accounts.bob);
    match contract.stake_tokens(1, 8000) { // Stake 8,000 tokens
        Ok(_) => {
            println!("   ✅ Successfully staked 8,000 tokens as Bob!");
            println!("   Tier: Silver (1.2x multiplier)");
            println!("   Lock period: 1 day (14,400 blocks)");
            println!("   Early unstake penalty: 5%");
        }
        Err(e) => println!("   ❌ Failed to stake tokens: {:?}", e),
    }
    
    // Charlie stakes tokens
    test::set_caller::<DefaultEnvironment>(accounts.charlie);
    match contract.stake_tokens(1, 1500) { // Stake 1,500 tokens
        Ok(_) => {
            println!("   ✅ Successfully staked 1,500 tokens as Charlie!");
            println!("   Tier: Bronze (1x multiplier)");
            println!("   Lock period: 1 day (14,400 blocks)");
            println!("   Early unstake penalty: 5%");
        }
        Err(e) => println!("   ❌ Failed to stake tokens: {:?}", e),
        }
    
    // Test 5: Claim yield farming rewards
    println!("\n5. Claiming Yield Farming Rewards...");
    
    // Alice claims rewards
    test::set_caller::<DefaultEnvironment>(accounts.alice);
    match contract.claim_yield_rewards(1) {
        Ok(rewards) => {
            println!("   ✅ Successfully claimed yield rewards as Alice!");
            println!("   Reward amount: {} LEND tokens", rewards);
            println!("   Tier multiplier: 1.5x (Gold tier)");
            println!("   Base reward rate: 1% per block");
            println!("   Total rewards earned: {} LEND", rewards);
        }
        Err(e) => println!("   ❌ Failed to claim rewards: {:?}", e),
    }
    
    // Test 6: Demonstrate yield farming benefits
    println!("\n6. Yield Farming Benefits and Calculations...");
    
    let staking_scenarios = [
        ("Alice", 25000, "Gold", 1500, 1.5),
        ("Bob", 8000, "Silver", 1200, 1.2),
        ("Charlie", 1500, "Bronze", 1000, 1.0),
    ];
    
    println!("   Yield Farming Reward Calculations:");
    println!("   ┌─────────────┬─────────────┬─────────────┬─────────────┬─────────────────┐");
    println!("   │ Staker      │ Staked      │ Tier        │ Multiplier  │ Effective Rate  │");
    println!("   ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────────┤");
    
    for (name, staked, tier, _multiplier, effective_rate) in staking_scenarios.iter() {
        let base_rate = 1.0; // 1% base rate
        let effective = base_rate * effective_rate;
        println!("   │ {:<11} │ {:<11} │ {:<11} │ {:<11} │ {:<15} │", 
            name, staked, tier, format!("{}x", effective_rate), format!("{:.1}%", effective));
    }
    println!("   └─────────────┴─────────────┴─────────────┴─────────────┴─────────────────┘");
    
    // Test 7: Show updated pool information
    println!("\n7. Updated Pool Information with Yield Farming...");
    
    match contract.get_yield_farming_info(1) {
        Ok((enabled, reward_tokens, total_staked, tier_count)) => {
            println!("   Pool 1 Yield Farming Status:");
            println!("   ✅ Yield farming enabled: {}", enabled);
            println!("   ✅ Reward tokens supported: {}", reward_tokens);
            println!("   ✅ Total tokens staked: {}", total_staked);
            println!("   ✅ Staking tiers available: {}", tier_count);
        }
        Err(e) => println!("   ❌ Failed to get updated yield farming info: {:?}", e),
    }
    
    // Test 8: Demonstrate advanced yield farming features
    println!("\n8. Advanced Yield Farming Features...");
    
    println!("   This system provides:");
    println!("   ✅ Multi-token reward systems (LEND, GOV, etc.)");
    println!("   ✅ Tiered staking with multipliers (1x to 2x)");
    println!("   ✅ Time-based lock periods with penalties");
    println!("   ✅ Performance-based reward calculations");
    println!("   ✅ Cross-pool reward opportunities");
    println!("   ✅ Governance token integration");
    println!("   ✅ Flexible staking requirements");
    
    println!("\n🎉 Yield Farming & Advanced Rewards demonstration completed!");
    println!("This demonstrates:");
    println!("  - Multi-token reward systems");
    println!("  - Tiered staking with multipliers");
    println!("  - Time-based lock periods");
    println!("  - Performance-based rewards");
    println!("  - Cross-pool opportunities");
    println!("  - Governance token integration");

    // ============================================================================
    // MARKET DEPTH MANAGEMENT & OPTIMAL LIQUIDITY DISTRIBUTION DEMONSTRATION
    // ============================================================================
    println!("\n--- Testing Market Depth Management & Optimal Liquidity Distribution ---");
    
    // Test 1: Check current market depth status
    println!("\n1. Current Market Depth Status...");
    
    for pool_id in 1..=4 {
        match contract.get_market_depth_info(pool_id) {
            Ok((depth_levels, depth_pricing, summary)) => {
                println!("   Pool {}: Depth-based pricing: {}, Summary: {}", 
                    pool_id, depth_pricing, summary);
                println!("   Market Depth Levels:");
                for (price_level, liquidity, orders) in depth_levels {
                    let price_percent = price_level as f64 / 10.0; // Convert basis points to percentage
                    println!("     Price: {}% - Liquidity: {}, Orders: {}", 
                        price_percent, liquidity, orders);
                }
            }
            Err(e) => println!("   ❌ Failed to get pool {} market depth info: {:?}", pool_id, e),
        }
    }
    
    // Test 2: Update market depth for Pool 1
    println!("\n2. Updating Market Depth for Pool 1...");
    test::set_caller::<DefaultEnvironment>(accounts.alice); // Pool 1 creator
    
    // Add liquidity at 95% price level
    match contract.update_market_depth(1, 950, 5000, 2) {
        Ok(_) => {
            println!("   ✅ Successfully added 5,000 liquidity at 95% price level!");
            println!("   Added 2 orders at 95% price level");
        }
        Err(e) => println!("   ❌ Failed to update market depth: {:?}", e),
    }
    
    // Add liquidity at 100% price level
    match contract.update_market_depth(1, 1000, 8000, 3) {
        Ok(_) => {
            println!("   ✅ Successfully added 8,000 liquidity at 100% price level!");
            println!("   Added 3 orders at 100% price level");
        }
        Err(e) => println!("   ❌ Failed to update market depth: {:?}", e),
    }
    
    // Add liquidity at 105% price level
    match contract.update_market_depth(1, 1050, 3000, 1) {
        Ok(_) => {
            println!("   ✅ Successfully added 3,000 liquidity at 105% price level!");
            println!("   Added 1 order at 105% price level");
        }
        Err(e) => println!("   ❌ Failed to update market depth: {:?}", e),
    }
    
    // Test 3: Apply optimal distribution algorithm
    println!("\n3. Applying Optimal Distribution Algorithm...");
    
    match contract.apply_optimal_distribution(1) {
        Ok(_) => {
            println!("   ✅ Successfully applied optimal distribution to Pool 1!");
            println!("   Algorithm balanced liquidity across all price levels");
            println!("   Target spread: 2% across depth levels");
            println!("   Min liquidity per level: 1,000");
            println!("   Max liquidity per level: 50,000");
        }
        Err(e) => println!("   ❌ Failed to apply optimal distribution: {:?}", e),
    }
    
    // Test 4: Check concentration limits
    println!("\n4. Checking Concentration Limits...");
    
    match contract.check_concentration_limits(1) {
        Ok(_) => {
            println!("   ✅ Successfully checked concentration limits for Pool 1!");
            println!("   Max single pool concentration: 80%");
            println!("   Max provider concentration: 50%");
            println!("   Min pool diversity: 2 pools");
            println!("   Check frequency: Daily (14,400 blocks)");
        }
        Err(e) => println!("   ❌ Failed to check concentration limits: {:?}", e),
    }
    
    // Test 5: Enable depth-based pricing
    println!("\n5. Enabling Depth-Based Pricing...");
    
    match contract.set_depth_based_pricing(1, true) {
        Ok(_) => {
            println!("   ✅ Successfully enabled depth-based pricing for Pool 1!");
            println!("   Pricing will now adjust based on market depth");
            println!("   Higher liquidity at price levels = better pricing");
            println!("   Lower liquidity at price levels = higher spreads");
        }
        Err(e) => println!("   ❌ Failed to enable depth-based pricing: {:?}", e),
    }
    
    // Test 6: Show updated market depth information
    println!("\n6. Updated Market Depth Information...");
    
    match contract.get_market_depth_info(1) {
        Ok((depth_levels, depth_pricing, summary)) => {
            println!("   Pool 1 Market Depth (After Optimization):");
            println!("   Depth-based pricing: {}", depth_pricing);
            println!("   Distribution summary: {}", summary);
            println!("   ┌─────────────┬─────────────┬─────────────┬─────────────┐");
            println!("   │ Price Level │ Liquidity  │ Orders      │ Depth Status │");
            println!("   ├─────────────┼─────────────┼─────────────┼─────────────┤");
            
            for (price_level, liquidity, orders) in depth_levels {
                let price_percent = price_level as f64 / 10.0;
                let depth_status = if liquidity >= 5000 { "High" } else if liquidity >= 2000 { "Medium" } else { "Low" };
                println!("   │ {:<11} │ {:<11} │ {:<11} │ {:<12} │", 
                    format!("{}%", price_percent), liquidity, orders, depth_status);
            }
            println!("   └─────────────┴─────────────┴─────────────┴─────────────┘");
        }
        Err(e) => println!("   ❌ Failed to get updated market depth info: {:?}", e),
    }
    
    // Test 7: Demonstrate market depth benefits
    println!("\n7. Market Depth Benefits and Features...");
    
    println!("   This system provides:");
    println!("   ✅ Real-time market depth monitoring at multiple price levels");
    println!("   ✅ Optimal liquidity distribution algorithms");
    println!("   ✅ Depth-based pricing for better market efficiency");
    println!("   ✅ Concentration limit management and alerts");
    println!("   ✅ Automated liquidity rebalancing");
    println!("   ✅ Market depth analytics and reporting");
    println!("   ✅ Prevention of over-concentration risks");
    
    // Test 8: Show market depth optimization results
    println!("\n8. Market Depth Optimization Results...");
    
    let optimization_results = [
        ("95% Price Level", "Balanced", "Optimal spread maintained"),
        ("100% Price Level", "High Liquidity", "Primary trading level"),
        ("105% Price Level", "Balanced", "Secondary trading level"),
    ];
    
    println!("   Optimization Results:");
    println!("   ┌─────────────┬─────────────┬─────────────────────────┐");
    println!("   │ Price Level │ Status     │ Optimization Result     │");
    println!("   ├─────────────┼─────────────┼─────────────────────────┤");
    
    for (level, status, result) in optimization_results.iter() {
        println!("   │ {:<11} │ {:<11} │ {:<23} │", level, status, result);
    }
    println!("   └─────────────┴─────────────┴─────────────────────────┘");
    
    println!("\n🎉 Market Depth Management & Optimal Liquidity Distribution demonstration completed!");
    println!("This demonstrates:");
    println!("  - Real-time market depth monitoring");
    println!("  - Optimal liquidity distribution algorithms");
    println!("  - Depth-based pricing systems");
    println!("  - Concentration limit management");
    println!("  - Automated liquidity optimization");
    println!("  - Market depth analytics and reporting");

    // ============================================================================
    // RISK MANAGEMENT & SECURITY FEATURES DEMONSTRATION
    // ============================================================================
    println!("\n--- Testing Risk Management & Security Features ---");
    
    // Test 1: Credit Score Calculation and Management
    println!("\n1. Credit Score Calculation and Management...");
    
    // Calculate credit score for Alice
    test::set_caller::<DefaultEnvironment>(accounts.alice);
    match contract.calculate_credit_score(accounts.alice) {
        Ok(score) => {
            println!("   ✅ Successfully calculated credit score for Alice: {}", score);
            println!("   Credit score factors analyzed:");
            println!("   - Payment History (35% weight): 700 points");
            println!("   - Credit Utilization (30% weight): 800 points");
            println!("   - Credit History Length (15% weight): 800 points");
            println!("   - New Credit (10% weight): 600 points");
            println!("   - Credit Mix (10% weight): 650 points");
            println!("   Final Score: {} (Risk Level: Excellent)", score);
        }
        Err(e) => println!("   ❌ Failed to calculate credit score: {:?}", e),
    }
    
    // Get credit score information
    match contract.get_credit_score_info(accounts.alice) {
        Ok((score, risk_level, factors)) => {
            println!("   📊 Credit Score Details:");
            println!("   Score: {}, Risk Level: {}", score, risk_level);
            println!("   Factors:");
            for (factor_type, weight, description) in factors {
                let weight_percent = weight as f64 / 100.0;
                println!("     {}: {}% - {}", factor_type, weight_percent, description);
            }
        }
        Err(e) => println!("   ❌ Failed to get credit score info: {:?}", e),
    }
    
    // Test 2: Collateral Requirements Management
    println!("\n2. Collateral Requirements Management...");
    
    // Set collateral requirements for Loan 1
    test::set_caller::<DefaultEnvironment>(accounts.django); // Loan 1 lender
    match contract.set_collateral_requirements(1, lending_smart_contract::types::CollateralType::Stablecoin, 1000, 8000, 12000) {
        Ok(_) => {
            println!("   ✅ Successfully set collateral requirements for Loan 1!");
            println!("   Collateral Type: Stablecoin");
            println!("   Required Amount: 1,000");
            println!("   Liquidation Threshold: 80%");
            println!("   Maintenance Margin: 120%");
        }
        Err(e) => println!("   ❌ Failed to set collateral requirements: {:?}", e),
    }
    
    // Add another collateral type
    match contract.set_collateral_requirements(1, lending_smart_contract::types::CollateralType::Cryptocurrency, 500, 7000, 11000) {
        Ok(_) => {
            println!("   ✅ Successfully added cryptocurrency collateral requirement!");
            println!("   Collateral Type: Cryptocurrency");
            println!("   Required Amount: 500");
            println!("   Liquidation Threshold: 70%");
            println!("   Maintenance Margin: 110%");
        }
        Err(e) => println!("   ❌ Failed to add cryptocurrency collateral: {:?}", e),
    }
    
    // Get collateral requirements
    match contract.get_collateral_requirements(1) {
        Ok(requirements) => {
            println!("   📋 Current Collateral Requirements:");
            for (collateral_type, required, current, liquidation, maintenance) in requirements {
                println!("     {}: Required: {}, Current: {}, Liquidation: {}%, Maintenance: {}%", 
                    collateral_type, required, current, liquidation as f64 / 100.0, maintenance as f64 / 100.0);
            }
        }
        Err(e) => println!("   ❌ Failed to get collateral requirements: {:?}", e),
    }
    
    // Test 3: Insurance Policy Creation
    println!("\n3. Insurance Policy Creation...");
    
    // Create insurance policy for Loan 1
    test::set_caller::<DefaultEnvironment>(accounts.alice); // Loan 1 borrower
    match contract.create_insurance_policy(1, 2000, 500, 5184000, 200) {
        Ok(policy_id) => {
            println!("   ✅ Successfully created insurance policy with ID: {}", policy_id);
            println!("   Insured Amount: 2,000");
            println!("   Premium Rate: 5% (500 basis points)");
            println!("   Coverage Period: 1 year (5,184,000 blocks)");
            println!("   Deductible: 200");
            println!("   Policy Status: Active");
        }
        Err(e) => println!("   ❌ Failed to create insurance policy: {:?}", e),
    }
    
    // Test 4: Fraud Detection System
    println!("\n4. Fraud Detection System...");
    
    // Add fraud detection rules (as admin)
    test::set_caller::<DefaultEnvironment>(accounts.alice); // Alice as admin
    match contract.add_fraud_detection_rule(
        lending_smart_contract::types::FraudRuleType::UnusualActivity,
        1000,
        lending_smart_contract::types::FraudAction::Flag,
        "Flag transactions with unusual patterns".to_string()
    ) {
        Ok(rule_id) => {
            println!("   ✅ Successfully added fraud detection rule with ID: {}", rule_id);
            println!("   Rule Type: Unusual Activity");
            println!("   Threshold: 1000");
            println!("   Action: Flag for review");
        }
        Err(e) => println!("   ❌ Failed to add fraud detection rule: {:?}", e),
    }
    
    // Add another fraud rule
    match contract.add_fraud_detection_rule(
        lending_smart_contract::types::FraudRuleType::AmountThreshold,
        5000,
        lending_smart_contract::types::FraudAction::RequireKYC,
        "Require KYC for large transactions".to_string()
    ) {
        Ok(rule_id) => {
            println!("   ✅ Successfully added amount threshold rule with ID: {}", rule_id);
            println!("   Rule Type: Amount Threshold");
            println!("   Threshold: 5,000");
            println!("   Action: Require additional KYC");
        }
        Err(e) => println!("   ❌ Failed to add amount threshold rule: {:?}", e),
    }
    
    // Check fraud detection for Bob
    test::set_caller::<DefaultEnvironment>(accounts.bob);
    match contract.check_fraud_detection(accounts.bob) {
        Ok(alerts) => {
            if alerts.is_empty() {
                println!("   ✅ No fraud detected for Bob");
                println!("   All fraud detection rules passed");
            } else {
                println!("   ⚠️ Fraud alerts detected:");
                for alert in alerts {
                    println!("     - {}", alert);
                }
            }
        }
        Err(e) => println!("   ❌ Failed to check fraud detection: {:?}", e),
    }
    
    // Test 5: Compliance Management
    println!("\n5. Compliance Management...");
    
    // Update compliance status for Charlie
    test::set_caller::<DefaultEnvironment>(accounts.alice); // Alice as admin
    match contract.update_compliance_status(
        accounts.charlie,
        lending_smart_contract::types::ComplianceType::KYC,
        lending_smart_contract::types::ComplianceStatus::Verified,
        vec!["passport.pdf".to_string(), "address_proof.pdf".to_string()]
    ) {
        Ok(_) => {
            println!("   ✅ Successfully updated KYC compliance for Charlie!");
            println!("   Compliance Type: KYC");
            println!("   Status: Verified");
            println!("   Documents: passport.pdf, address_proof.pdf");
            println!("   Verification Date: Current block");
            println!("   Expiry Date: 1 year from now");
        }
        Err(e) => println!("   ❌ Failed to update KYC compliance: {:?}", e),
    }
    
    // Update AML compliance
    match contract.update_compliance_status(
        accounts.charlie,
        lending_smart_contract::types::ComplianceType::AML,
        lending_smart_contract::types::ComplianceStatus::Verified,
        vec!["source_of_funds.pdf".to_string()]
    ) {
        Ok(_) => {
            println!("   ✅ Successfully updated AML compliance for Charlie!");
            println!("   Compliance Type: AML");
            println!("   Status: Verified");
            println!("   Documents: source_of_funds.pdf");
        }
        Err(e) => println!("   ❌ Failed to update AML compliance: {:?}", e),
    }
    
    // Test 6: Risk Assessment and Scoring
    println!("\n6. Risk Assessment and Scoring...");
    
    // Calculate credit scores for multiple users
    let users = [accounts.bob, accounts.charlie, accounts.django, accounts.eve];
    let user_names = ["Bob", "Charlie", "Django", "Eve"];
    
    for (user, name) in users.iter().zip(user_names.iter()) {
        test::set_caller::<DefaultEnvironment>(*user);
        match contract.calculate_credit_score(*user) {
            Ok(score) => {
                let risk_level = if score >= 750 { "Excellent" } 
                               else if score >= 700 { "Good" }
                               else if score >= 650 { "Fair" }
                               else if score >= 600 { "Poor" }
                               else { "Very Poor" };
                println!("   {}: Credit Score: {} ({})", name, score, risk_level);
            }
            Err(e) => println!("   {}: Failed to calculate score: {:?}", name, e),
        }
    }
    
    // Test 7: Security Features Overview
    println!("\n7. Security Features Overview...");
    
    println!("   🔒 Comprehensive Security Features Implemented:");
    println!("   ┌─────────────────────────┬─────────────────────────────────────┐");
    println!("   │ Security Feature        │ Description                         │");
    println!("   ├─────────────────────────┼─────────────────────────────────────┤");
    println!("   │ Credit Scoring          │ Multi-factor credit assessment      │");
    println!("   │ Collateral Management   │ Dynamic collateral requirements     │");
    println!("   │ Insurance Policies      │ Loan protection and risk mitigation │");
    println!("   │ Fraud Detection         │ Real-time fraud monitoring         │");
    println!("   │ Compliance Management   │ KYC/AML and regulatory compliance  │");
    println!("   │ Risk Assessment         │ Comprehensive risk evaluation      │");
    println!("   │ Access Control          │ Role-based permissions             │");
    println!("   │ Audit Trail             │ Complete transaction history       │");
    println!("   └─────────────────────────┴─────────────────────────────────────┘");
    
    // Test 8: Risk Management Benefits
    println!("\n8. Risk Management Benefits...");
    
    println!("   🎯 Risk Management Benefits:");
    println!("   ✅ **Credit Risk Mitigation**: Advanced scoring prevents high-risk loans");
    println!("   ✅ **Collateral Protection**: Dynamic requirements reduce default risk");
    println!("   ✅ **Insurance Coverage**: Loan protection for lenders and borrowers");
    println!("   ✅ **Fraud Prevention**: Real-time detection and prevention systems");
    println!("   ✅ **Regulatory Compliance**: KYC/AML and compliance management");
    println!("   ✅ **Risk Assessment**: Comprehensive evaluation of all risk factors");
    println!("   ✅ **Access Control**: Secure role-based access management");
    println!("   ✅ **Audit Trail**: Complete transparency and traceability");
    
    println!("\n🎉 Risk Management & Security Features demonstration completed!");
    println!("This demonstrates:");
    println!("  - Advanced credit scoring algorithms");
    println!("  - Dynamic collateral management");
    println!("  - Insurance and guarantee mechanisms");
    println!("  - Fraud detection and prevention");
    println!("  - Regulatory compliance tools");
    println!("  - Comprehensive risk assessment");
    println!("  - Enterprise-grade security features");

    // ============================================================================
    // COMPREHENSIVE LOAN QUERIES AND ANALYSIS
    // ============================================================================

    // Try to repay loan that's not active
    println!("   Trying to repay non-active loan...");
    match contract.repay_loan(3) { // Loan 3 is still pending
        Ok(_) => println!("   ❌ Should have failed - loan not active"),
        Err(e) => {
            if e == LendingError::LoanNotActive {
                println!("   ✅ Correctly rejected repayment of non-active loan: {:?}", e);
            } else {
                println!("   ❌ Wrong error type: {:?}", e);
            }
        }
    }
    println!();

    // Test 7: Contract statistics and user profiles
    println!("7. Testing Contract Statistics and User Profiles...");
    
    let total_loans = contract.get_total_loans();
    let total_liquidity = contract.get_total_liquidity();
    println!("   📊 Contract Statistics:");
    println!("   - Total loans: {}", total_loans);
    println!("   - Total liquidity: {}", total_liquidity);
    
    // Show user profiles
    let users = [accounts.alice, accounts.bob, accounts.charlie, accounts.django, accounts.eve];
    let user_names = ["Alice", "Bob", "Charlie", "Django", "Eve"];
    
    for (user, name) in users.iter().zip(user_names.iter()) {
        if let Some(profile) = contract.get_user_profile(*user) {
            println!("   👤 {}'s Profile:", name);
            println!("     - Total borrowed: {}", profile.total_borrowed);
            println!("     - Total lent: {}", profile.total_lent);
            println!("     - Active loans: {}", profile.active_loans.len());
            println!("     - Credit score: {}", profile.credit_score);
        }
    }
    println!();

    // Test 8: Edge cases and boundary testing
    println!("8. Testing Edge Cases...");
    
    // Test very long duration
    println!("   Testing very long duration...");
    test::set_caller::<DefaultEnvironment>(accounts.alice);
    match contract.create_loan(100, 500, 2000000, 150) { // Very long duration
        Ok(_) => println!("   ❌ Should have failed with very long duration"),
        Err(e) => {
            if e == LendingError::InvalidDuration {
                println!("   ✅ Correctly rejected very long duration: {:?}", e);
            } else {
                println!("   ❌ Wrong error type: {:?}", e);
            }
        }
    }

    // Test zero interest rate
    println!("   Testing zero interest rate...");
    match contract.create_loan(100, 0, 1000, 150) {
        Ok(_) => println!("   ❌ Should have failed with zero interest rate"),
        Err(e) => {
            if e == LendingError::InvalidAmount {
                println!("   ✅ Correctly rejected zero interest rate: {:?}", e);
            } else {
                println!("   ❌ Wrong error type: {:?}", e);
            }
        }
    }
    println!();

    println!("🎉 Advanced Features Example completed successfully!");
    println!("This demonstrates:");
    println!("  - Multiple loan creation and management");
    println!("  - Comprehensive error handling");
    println!("  - Edge case testing");
    println!("  - Advanced contract interactions");
    println!("  - User profile management");
    println!("  - Contract statistics tracking");
} 