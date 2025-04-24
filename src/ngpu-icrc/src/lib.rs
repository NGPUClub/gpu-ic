use candid::{CandidType, Nat, Principal};

#[derive(CandidType, Clone, Debug)]
struct Allowance{
    spender: Principal,
    amount: Nat,
}



#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
