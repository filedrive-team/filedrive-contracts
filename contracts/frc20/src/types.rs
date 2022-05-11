use fvm_shared::ActorID;
use fvm_utils::u256::U256;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transfer {
    from: ActorID,
    to: ActorID,
    value: U256,
}

impl Transfer {
    pub fn new(_from: ActorID, _to: ActorID, _value: U256) -> Self {
        Self {
            from: _from,
            to: _to,
            value: _value,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Approve {
    owner: ActorID,
    spend: ActorID,
    value: U256,
}

impl Approve {
    pub fn new(owner: ActorID, spend: ActorID, value: U256) -> Self {
        Self {
            owner,
            spend,
            value,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}
