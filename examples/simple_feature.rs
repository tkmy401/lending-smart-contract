#[cfg(feature = "simple-feature")]
pub fn simple_feature_function() {
    println!("Simple feature is enabled!");
}

#[cfg(not(feature = "simple-feature"))]
pub fn simple_feature_function() {
    println!("Simple feature is not enabled.");
}

fn main() {
    simple_feature_function();
    match contract.fund_loan(1) {
        Ok(_) => {
            println!("   ✅ Loan funded successfully");
        }
        Err(e) => {
            println!("   ❌ Failed to fund loan: {:?}", e);
            return;
        }
    }
    println!();

    // Test 5: Repay the loan
    println!("5. Testing Loan Repayment...");
    // Set up test environment for repayment
    test::set_caller::<DefaultEnvironment>(accounts.alice);
    let repayment_amount = 1050; // Principal + Interest

    match contract.repay_loan(1, repayment_amount) {
        Ok(_) => {
            println!("   ✅ Loan repaid successfully");
        }
        Err(e) => {
            println!("   ❌ Failed to repay loan: {:?}", e);
            return;
        }
    }
    println!();

    // Test 6: Query loan status after repayment
    println!("6. Testing Loan Status After Repayment...");
    if let Some(loan) = contract.get_loan(1) {
        println!("   ✅ Loan retrieved successfully");
        println!("   - ID: {}", loan.id);
        println!("   - Status: {:?}", loan.status);
    } else {
        println!("   ❌ Failed to retrieve loan");
        return;
    }
    println!();

    println!("All tests completed.");
}