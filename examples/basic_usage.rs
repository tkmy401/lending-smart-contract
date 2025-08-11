//! Basic usage example for the Lending Smart Contract
//! This example demonstrates the core functionality of the contract

use ink_e2e::{
    build_message, create_client, ink_e2e::AccountKeyring, ContractsBackend, E2EBackend,
    Environment, SubmittableExt,
};

use lending_smart_contract::{
    LendingContract, LendingContractRef, Loan, LoanStatus, UserProfile,
};

/// Example demonstrating the complete lending workflow
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Lending Smart Contract Demo");
    
    // Initialize the client
    let mut client = create_client().await;
    
    // Deploy the contract
    println!("📦 Deploying contract...");
    let contract = client
        .instantiate("lending_smart_contract", &ink_e2e::InstantiateArgs::default())
        .submit()
        .await?;
    
    let mut call_builder = contract.call_builder::<LendingContract>();
    
    // Get initial contract state
    let total_loans = client
        .call(&mut call_builder.get_total_loans())
        .call()
        .await?
        .value;
    println!("📊 Initial total loans: {}", total_loans);
    
    // Example 1: Alice creates a loan request
    println!("\n👩 Alice creates a loan request...");
    let alice = ink_e2e::AccountKeyring::Alice.public_key();
    let alice_address = ink_e2e::AccountKeyring::Alice.address();
    
    let create_loan = call_builder.create_loan(1000, 500, 1000, 1500);
    let loan_result = client
        .call(&alice, &create_loan)
        .value(0)
        .submit()
        .await?;
    
    let loan_id = loan_result.value;
    println!("✅ Loan created with ID: {}", loan_id);
    
    // Get loan details
    let loan = client
        .call(&mut call_builder.get_loan(loan_id))
        .call()
        .await?
        .value
        .unwrap();
    
    println!("📋 Loan details:");
    println!("   Amount: {}", loan.amount);
    println!("   Interest Rate: {} basis points", loan.interest_rate);
    println!("   Duration: {} blocks", loan.duration);
    println!("   Status: {:?}", loan.status);
    
    // Example 2: Bob funds the loan
    println!("\n👨 Bob funds the loan...");
    let bob = ink_e2e::AccountKeyring::Bob.public_key();
    
    let fund_loan = call_builder.fund_loan(loan_id);
    client
        .call(&bob, &fund_loan)
        .value(1000)
        .submit()
        .await?;
    
    println!("✅ Loan funded successfully!");
    
    // Check loan status after funding
    let funded_loan = client
        .call(&mut call_builder.get_loan(loan_id))
        .call()
        .await?
        .value
        .unwrap();
    
    println!("📊 Loan status after funding: {:?}", funded_loan.status);
    
    // Example 3: Alice repays the loan
    println!("\n👩 Alice repays the loan...");
    let repayment_amount = 1000 + ((1000 * 500) / 10000); // Principal + Interest
    println!("💰 Repayment amount: {} (Principal: 1000 + Interest: {})", 
             repayment_amount, repayment_amount - 1000);
    
    let repay_loan = call_builder.repay_loan(loan_id);
    client
        .call(&alice, &repay_loan)
        .value(repayment_amount)
        .submit()
        .await?;
    
    println!("✅ Loan repaid successfully!");
    
    // Check final loan status
    let final_loan = client
        .call(&mut call_builder.get_loan(loan_id))
        .call()
        .await?
        .value
        .unwrap();
    
    println!("📊 Final loan status: {:?}", final_loan.status);
    
    // Get updated contract statistics
    let final_total_loans = client
        .call(&mut call_builder.get_total_loans())
        .call()
        .await?
        .value;
    
    let final_liquidity = client
        .call(&mut call_builder.get_total_liquidity())
        .call()
        .await?
        .value;
    
    println!("\n📈 Final Contract Statistics:");
    println!("   Total Loans: {}", final_total_loans);
    println!("   Total Liquidity: {}", final_liquidity);
    
    // Get user profiles
    let alice_profile = client
        .call(&mut call_builder.get_user_profile(alice_address))
        .call()
        .await?
        .value;
    
    if let Some(profile) = alice_profile {
        println!("\n👩 Alice's Profile:");
        println!("   Total Borrowed: {}", profile.total_borrowed);
        println!("   Total Lent: {}", profile.total_lent);
        println!("   Active Loans: {}", profile.active_loans.len());
        println!("   Credit Score: {}", profile.credit_score);
    }
    
    println!("\n🎉 Demo completed successfully!");
    Ok(())
} 