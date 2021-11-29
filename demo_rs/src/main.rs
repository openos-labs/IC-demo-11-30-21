use candid::{candid_method, CandidType, Deserialize, Principal};
use ic_cdk::{api, storage};
use ic_cdk_macros::*;
use serde_bytes::{ByteBuf};
use std::collections::HashMap;

type Balances = HashMap<Principal, u64>;
type HeaderField = (String, String);

#[derive(Clone, Debug, CandidType, Deserialize)]
struct HttpResponse {
    status_code: u16,
    headers: Vec<HeaderField>,
    body: ByteBuf,
}

#[init]
#[candid_method(init)]
fn init() {
    let balances = storage::get_mut::<Balances>();
    balances.insert(api::caller(), 1000);
}

#[update(name = "transfer")]
#[candid_method(update)]
fn transfer(to: Principal, value: u64) -> bool {
    let balances = storage::get_mut::<Balances>();
    let from = api::caller();

    let from_balance = balance_of(from);
    let from_balance_new = from_balance - value;
    if from_balance_new != 0 {
        balances.insert(from, from_balance_new);
    } else {
        balances.remove(&from);
    };

    let to_balance = balance_of(to);
    let to_balance_new = to_balance + value;
    if to_balance_new != 0 {
        balances.insert(to, to_balance_new);
    };
    true
}


#[query(name = "balanceOf")]
#[candid_method(query, rename = "balanceOf")]
fn balance_of(who: Principal) -> u64 {
    let balances = storage::get::<Balances>();
    match balances.get(&who) {
        Some(balance) => *balance,
        None => 0,
    }
}

#[query(name = "allBalances")]
#[candid_method(query, rename = "allBalances")]
fn all_balance() -> Vec<(Principal, u64)> {
    let mut balance = Vec::new();
    for (&k, &v) in storage::get::<Balances>().iter() {
        balance.push((k, v));
    }
    balance    
}

#[query(name = "http_request")]
#[candid_method(query, rename = "http_request")]
fn http_request() -> HttpResponse {
    let balances = storage::get::<Balances>();
    let mut list: String = String::from("Total ");
    list.push_str(&balances.len().to_string());
    list.push_str(" hodl: \n\n");
    list.push_str("Principal:                                                       balances: \n");
    for (&k, &v) in balances.iter() {
        list.push_str(&k.to_text());
        list.push_str("  ");
        list.push_str(&v.to_string());
        list.push_str("\n");
    }
    return HttpResponse {
        status_code: 200,
        headers: vec![(String::from("content-type"), String::from("text/plain"))],
        body: ByteBuf::from(list.as_bytes()),
    };
}


#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    candid::export_service!();
    std::print!("{}", __export_service());
}

#[pre_upgrade]
fn pre_upgrade() {
    let mut balance = Vec::new();
    for (&k, &v) in storage::get::<Balances>().iter() {
        balance.push((k, v));
    }
    storage::stable_save((balance, )).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let (down,): (Vec<(Principal, u64)>,) = storage::stable_restore().unwrap();
    for (k, v) in down {
        storage::get_mut::<Balances>().insert(k, v);
    }
}