use ink_e2e::{
    build_message, create_client, ink_e2e::AccountKeyring, ContractsBackend, E2EBackend,
    Environment, SubmittableExt,
};

use lending_smart_contract::{
    LendingContract, LendingContractRef, Loan, LoanStatus, UserProfile,
};

type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[ink_e2e::test]
async fn test_contract_creation(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    // Given
    let mut constructor = LendingContractRef::new();
    
    // When
    let contract = client
        .instantiate("lending_smart_contract", &ink_e2e::InstantiateArgs::default())
        .submit()
        .await
        .expect("instantiate failed");
    let mut call_builder = contract.call_builder::<LendingContract>();

    // Then
    let total_loans = client
        .call(&mut call_builder.get_total_loans())
        .call()
        .await
        .expect("call failed")
        .value;
    assert_eq!(total_loans, 0);

    let total_liquidity = client
        .call(&mut call_builder.get_total_liquidity())
        .call()
        .await
        .expect("call failed")
        .value;
    assert_eq!(total_liquidity, 0);

    Ok(())
}

#[ink_e2e::test]
async fn test_create_loan(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    // Given
    let mut constructor = LendingContractRef::new();
    let contract = client
        .instantiate("lending_smart_contract", &ink_e2e::InstantiateArgs::default())
        .submit()
        .await
        .expect("instantiate failed");
    let mut call_builder = contract.call_builder::<LendingContract>();

    let alice = ink_e2e::AccountKeyring::Alice.public_key();
    let alice_address = ink_e2e::AccountKeyring::Alice.address();

    // When
    let create_loan = call_builder.create_loan(1000, 500, 1000, 1500);
    let result = client
        .call(&alice, &create_loan)
        .value(0)
        .submit()
        .await
        .expect("create_loan failed");

    // Then
    let loan_id = result.value;
    assert_eq!(loan_id, 1);

    let loan = client
        .call(&mut call_builder.get_loan(loan_id))
        .call()
        .await
        .expect("get_loan failed")
        .value;
    
    assert!(loan.is_some());
    let loan = loan.unwrap();
    assert_eq!(loan.borrower, alice_address);
    assert_eq!(loan.amount, 1000);
    assert_eq!(loan.interest_rate, 500);
    assert_eq!(loan.status, LoanStatus::Pending);

    Ok(())
}

#[ink_e2e::test]
async fn test_fund_loan(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    // Given
    let mut constructor = LendingContractRef::new();
    let contract = client
        .instantiate("lending_smart_contract", &ink_e2e::InstantiateArgs::default())
        .submit()
        .await
        .expect("instantiate failed");
    let mut call_builder = contract.call_builder::<LendingContract>();

    let alice = ink_e2e::AccountKeyring::Alice.public_key();
    let bob = ink_e2e::AccountKeyring::Bob.public_key();

    // Create loan
    let create_loan = call_builder.create_loan(1000, 500, 1000, 1500);
    let loan_result = client
        .call(&alice, &create_loan)
        .value(0)
        .submit()
        .await
        .expect("create_loan failed");
    let loan_id = loan_result.value;

    // When - Fund loan
    let fund_loan = call_builder.fund_loan(loan_id);
    let result = client
        .call(&bob, &fund_loan)
        .value(1000)
        .submit()
        .await
        .expect("fund_loan failed");

    // Then
    let loan = client
        .call(&mut call_builder.get_loan(loan_id))
        .call()
        .await
        .expect("get_loan failed")
        .value
        .unwrap();
    
    assert_eq!(loan.status, LoanStatus::Active);
    assert!(loan.lender.is_some());

    Ok(())
}

#[ink_e2e::test]
async fn test_repay_loan(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    // Given
    let mut constructor = LendingContractRef::new();
    let contract = client
        .instantiate("lending_smart_contract", &ink_e2e::InstantiateArgs::default())
        .submit()
        .await
        .expect("instantiate failed");
    let mut call_builder = contract.call_builder::<LendingContract>();

    let alice = ink_e2e::AccountKeyring::Alice.public_key();
    let bob = ink_e2e::AccountKeyring::Bob.public_key();

    // Create and fund loan
    let create_loan = call_builder.create_loan(1000, 500, 1000, 1500);
    let loan_result = client
        .call(&alice, &create_loan)
        .value(0)
        .submit()
        .await
        .expect("create_loan failed");
    let loan_id = loan_result.value;

    let fund_loan = call_builder.fund_loan(loan_id);
    client
        .call(&bob, &fund_loan)
        .value(1000)
        .submit()
        .await
        .expect("fund_loan failed");

    // When - Repay loan (1000 + 50 interest = 1050)
    let repay_loan = call_builder.repay_loan(loan_id);
    let result = client
        .call(&alice, &repay_loan)
        .value(1050)
        .submit()
        .await
        .expect("repay_loan failed");

    // Then
    let loan = client
        .call(&mut call_builder.get_loan(loan_id))
        .call()
        .await
        .expect("get_loan failed")
        .value
        .unwrap();
    
    assert_eq!(loan.status, LoanStatus::Repaid);

    Ok(())
}

#[ink_e2e::test]
async fn test_invalid_loan_creation(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    // Given
    let mut constructor = LendingContractRef::new();
    let contract = client
        .instantiate("lending_smart_contract", &ink_e2e::InstantiateArgs::default())
        .submit()
        .await
        .expect("instantiate failed");
    let mut call_builder = contract.call_builder::<LendingContract>();

    let alice = ink_e2e::AccountKeyring::Alice.public_key();

    // When - Try to create loan with invalid parameters
    let create_loan = call_builder.create_loan(0, 500, 1000, 1500);
    let result = client
        .call(&alice, &create_loan)
        .value(0)
        .submit()
        .await;

    // Then - Should fail
    assert!(result.is_err());

    Ok(())
} 