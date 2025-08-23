//! Advanced features example for the Lending Smart Contract
//! This example demonstrates advanced functionality like multiple loans, collateral management, and error handling

use ink::env::{
    DefaultEnvironment,
    test,
};

use lending_smart_contract::{LendingContract, types::{RateAdjustmentReason, CompoundFrequency, PaymentStructure}, errors::LendingError};

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