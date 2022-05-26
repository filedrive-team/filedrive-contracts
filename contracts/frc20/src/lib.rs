mod frc20;
mod strandard_token;
mod types;

use crate::frc20::FRC20;
use crate::strandard_token::StandardToken;
use crate::types::{Approve, Transfer};
use fvm_ipld_encoding::Cbor;
use fvm_ipld_encoding::{RawBytes, DAG_CBOR};
use fvm_sdk as sdk;
use fvm_sdk::message::{params_raw, NO_DATA_BLOCK_ID};
use fvm_shared::bigint::BigUint;
use fvm_shared::ActorID;
use fvm_utils::storage::Storage;
use fvm_utils::u256::U256;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;

const TO_ACTOR_ID: ActorID = 200;

macro_rules! abort {
    ($code:ident, $msg:literal $(, $ex:expr)*) => {
        fvm_sdk::vm::abort(
            fvm_shared::error::ExitCode::$code.value(),
            Some(format!($msg, $($ex,)*).as_str()),
        )
    };
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Params {
    method: String,
    name: String,
    age: u32,
}

impl Cbor for Params {}

#[no_mangle]
pub fn invoke(_params_id: u32) -> u32 {
    let ret: Option<RawBytes> = match sdk::message::method_number() {
        1 => geneses(),
        2 => balance_of(),
        3 => transfer(),
        4 => approve(),
        5 => allowance(),
        10 => Some(RawBytes::new(
            fvm_sdk::message::caller().to_string().as_bytes().to_vec(),
        )),
        11 => {
            let (codec, raw) = match params_raw(_params_id) {
                Ok(v) => v,
                Err(e) => abort!(USR_UNHANDLED_MESSAGE, "failed to parse params1: {:?}", e),
            };
            debug_assert!(codec == DAG_CBOR, "parameters codec was not cbor");
            match fvm_ipld_encoding::from_slice(raw.as_slice()) {
                Ok(v) => v,
                Err(e) => abort!(USR_UNHANDLED_MESSAGE, "failed to parse params3: {:?}", e),
            }
        }
        _ => abort!(USR_UNHANDLED_MESSAGE, "unrecognized method"),
    };

    match ret {
        None => NO_DATA_BLOCK_ID,
        Some(v) => match sdk::ipld::put_block(DAG_CBOR, v.bytes()) {
            Ok(id) => id,
            Err(err) => abort!(USR_SERIALIZATION, "failed to store return value: {}", err),
        },
    }
}

pub fn geneses() -> Option<RawBytes> {
    let mut balance: HashMap<ActorID, U256> = HashMap::new();
    balance.insert(
        sdk::message::caller(),
        U256 {
            big_uint: BigUint::parse_bytes(b"1000000000", 10).unwrap(),
        },
    );
    let mut erc20 = StandardToken::default();
    erc20.name = "FToken".to_string();
    erc20.decimals = 18;
    erc20.symbol = "FT".to_string();
    erc20.total_supply = U256 {
        big_uint: BigUint::parse_bytes(b"1000000000", 10).unwrap(),
    };
    erc20.balances = balance;
    let storage = Storage {
        contract: erc20.clone(),
    };
    storage.save();
    Some(RawBytes::new(
        erc20.total_supply.to_string().as_bytes().to_vec(),
    ))
}

pub fn balance_of() -> Option<RawBytes> {
    let contract = Storage::<StandardToken>::load();
    let _balance = contract.balance_of(TO_ACTOR_ID);
    Some(RawBytes::new(_balance.to_string().as_bytes().to_vec()))
}

pub fn transfer() -> Option<RawBytes> {
    let mut contract = Storage::<StandardToken>::load();
    let to = 200;
    let value = U256 {
        big_uint: BigUint::parse_bytes(b"1", 10).unwrap(),
    };
    contract.transfer(TO_ACTOR_ID, value.clone());
    let storage = Storage { contract };
    storage.save();
    let result = Transfer::new(sdk::message::caller(), to, value.clone());
    Some(RawBytes::new(result.to_bytes()))
}

fn approve() -> Option<RawBytes> {
    let value = U256 {
        big_uint: BigUint::parse_bytes(b"1", 10).unwrap(),
    };
    let mut contract = Storage::<StandardToken>::load();
    contract.approve(TO_ACTOR_ID, value.clone());
    let storage = Storage { contract };
    storage.save();
    let result = Approve::new(sdk::message::caller(), TO_ACTOR_ID, value.clone());
    Some(RawBytes::new(result.to_bytes()))
}

fn allowance() -> Option<RawBytes> {
    let mut contract = Storage::<StandardToken>::load();
    let result = contract.allowance(sdk::message::caller(), TO_ACTOR_ID);
    let result = Approve::new(sdk::message::caller(), TO_ACTOR_ID, result);
    Some(RawBytes::new(result.to_bytes()))
}
