use near_sdk::{json_types::ValidAccountId, serde_json::json, PendingContractTx};
use near_sdk_sim::{
    deploy, init_simulator, near_crypto::Signer, to_yocto, ContractAccount, UserAccount,
    DEFAULT_GAS, STORAGE_AMOUNT,
};

/// Bring contract crate into namespace
extern crate fungible_token;
/// Import the generated proxy contract
use fungible_token::ContractContract;
use near_sdk::json_types::U128;
use near_sdk_sim::account::AccessKey;
use std::convert::TryFrom;

const FT_CONTRACT_NAME: &str = "ft-contract";

// Load in contract bytes
near_sdk_sim::lazy_static! {
    static ref TOKEN_WASM_BYTES: &'static [u8] = include_bytes!("../res/fungible_token.wasm").as_ref();
}

fn init(initial_balance: u128) -> (UserAccount, ContractAccount<ContractContract>, UserAccount) {
    let master_account = init_simulator(None);
    // uses default values for deposit and gas
    let contract_user = deploy!(
        // Contract Proxy
        contract: ContractContract,
        // Contract account id
        contract_id: FT_CONTRACT_NAME,
        // Bytes of contract
        bytes: &TOKEN_WASM_BYTES,
        // User deploying the contract,
        signer_account: master_account,
        // init method
        /*
        (owner_id: ValidAccountId, total_supply: U128, version: String, name: String, symbol: String, reference: String, reference_hash: String, decimals: u8)
         */

        init_method: new(
            ValidAccountId::try_from(master_account.account_id()).unwrap(),
            U128::from(initial_balance),
            "1.0.0".to_string(),
            "Mochi Tokens".to_string(),
            "MOCHI".to_string(),
            "https://example.com/mochi.json".to_string(),
            "7c879fa7b49901d0ecc6ff5d64d7f673da5e4a5eb52a8d50a214175760d8919a".to_string(),
            19
        )
    );
    let alice = master_account.create_user("alice".to_string(), to_yocto("100"));
    (master_account, contract_user, alice)
}

/// Example of how to create and use an user transaction.
fn init2(initial_balance: u128) {
    let master_account = init_simulator(None);
    let txn = master_account.create_transaction(FT_CONTRACT_NAME.into());
    // uses default values for deposit and gas
    let res = txn
        .create_account()
        .add_key(master_account.signer.public_key(), AccessKey::full_access())
        .transfer(initial_balance)
        .deploy_contract((&TOKEN_WASM_BYTES).to_vec())
        .submit();
    println!("{:#?}", res);
}

#[test]
pub fn mint_token() {
    init2(to_yocto("35"));
}

#[test]
fn simulate_transfer() {
    let transfer_amount = to_yocto("100");
    let initial_balance = to_yocto("100000");
    let (master_account, contract, alice) = init(initial_balance);
    // Uses default gas amount, `near_sdk_sim::DEFAULT_GAS`
    let res = alice.call(
        PendingContractTx::new(
            &contract.user_account.account_id(),
            "storage_deposit",
            json!({}),
            false,
        ),
        2350000000000000000000,
        DEFAULT_GAS,
    );
    // println!("{:#?}\n Cost:\n{:#?}", res.status(), res.profile_data());
    assert!(res.is_ok());

    let res = master_account.call(
        PendingContractTx::new(
            &contract.user_account.account_id(),
            "ft_transfer",
            json!({ "receiver_id": alice.account_id(), "amount": U128::from(transfer_amount) }),
            false,
        ),
        1,
        DEFAULT_GAS,
    );
    // println!("{:#?}\n Cost:\n{:#?}", res.status(), res.profile_data());
    assert!(res.is_ok());

    let value = alice.call(
        PendingContractTx::new(
            FT_CONTRACT_NAME,
            "ft_balance_of",
            json!({ "account_id": master_account.account_id() }),
            true,
        ),
        STORAGE_AMOUNT,
        DEFAULT_GAS,
    );
    // let value = view!(contract.ft_balance_of(master_account.account_id()));
    let value: U128 = value.unwrap_json();
    assert_eq!(initial_balance - transfer_amount, value.0);
}
