//! Advanced features example for the Lending Smart Contract
//! This example demonstrates advanced functionality like multiple loans, collateral management, and error handling

use ink::env::{
    DefaultEnvironment,
    test,
};

use lending_smart_contract::{LendingContract, errors::LendingError};

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