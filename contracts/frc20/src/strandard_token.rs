use crate::frc20::FRC20;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_shared::ActorID;
use fvm_utils::abort;
use fvm_utils::u256::U256;
use std::collections::HashMap;
use std::ops::{Add, Sub};

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, Default)]
pub struct StandardToken {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: U256,
    pub balances: HashMap<ActorID, U256>,
    allowed: HashMap<ActorID, HashMap<ActorID, U256>>,
}

impl FRC20 for StandardToken {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn symbol(&self) -> String {
        self.symbol.clone()
    }

    fn decimals(&self) -> u8 {
        self.decimals
    }

    fn total_supply(&self) -> U256 {
        self.total_supply.clone()
    }

    fn balance_of(&self, _owner: ActorID) -> U256 {
        let balance_default = U256::default();
        let balance = self.balances.get(&_owner).unwrap_or(&balance_default);
        balance.clone()
    }

    fn transfer(&mut self, _to: ActorID, _value: U256) -> bool {
        let _from = fvm_sdk::message::caller();
        let _from_balance_default = U256::default();

        let from_balance = self.balances.get_mut(&_from);
        match from_balance {
            Some(balance) => {
                if *balance < _value {
                    abort!(SYS_ASSERTION_FAILED, "Insufficient Balance")
                }
                *balance = balance.clone() - _value.clone();
            }
            _ => {}
        }

        let to_balance = self.balances.get_mut(&_to);
        match to_balance {
            Some(balance) => {
                if *balance < _value {
                    abort!(SYS_ASSERTION_FAILED, "Insufficient Balance")
                }
                *balance = balance.clone().add(_value.clone())
            }
            None => {
                self.balances.insert(_to, _value.clone());
            }
        }
        true
    }

    fn transfer_from(&mut self, _from: ActorID, _to: ActorID, _value: U256) -> bool {
        match self.allowed.get_mut(&_from) {
            Some(allowance) => match allowance.get_mut(&_to) {
                Some(allowance_balance) => {
                    if *allowance_balance < _value {
                        abort!(SYS_ASSERTION_FAILED, "Insufficient Balance");
                    }

                    let from_balance = self.balances.get_mut(&_from);
                    match from_balance {
                        Some(balance) => {
                            if *balance < _value {
                                abort!(SYS_ASSERTION_FAILED, "Insufficient Balance")
                            }
                            *balance = balance.clone().sub(_value.clone());
                        }
                        _ => {}
                    }

                    let to_balance = self.balances.get_mut(&_to);
                    match to_balance {
                        Some(balance) => {
                            if *balance < _value {
                                abort!(SYS_ASSERTION_FAILED, "Insufficient Balance")
                            }
                            *balance = balance.clone().add(_value.clone())
                        }
                        None => {
                            self.balances.insert(_to, _value.clone());
                        }
                    }
                }
                None => {
                    abort!(SYS_ASSERTION_FAILED, "Insufficient Balance");
                }
            },
            None => {
                abort!(SYS_ASSERTION_FAILED, "Insufficient Balance");
            }
        }
        false
    }

    fn approve(&mut self, _spender: ActorID, _value: U256) -> bool {
        let from = fvm_sdk::message::caller();
        if let None = self.allowed.get_mut(&from) {
            self.allowed.insert(from, HashMap::new());
        }
        self.allowed
            .get_mut(&from)
            .unwrap()
            .insert(_spender, _value);
        true
    }

    fn allowance(&mut self, _owner: ActorID, _spender: ActorID) -> U256 {
        if let Some(spender_balance) = self.allowed.get_mut(&_owner) {
            if let Some(balance) = spender_balance.get(&_spender) {
                balance.clone()
            } else {
                U256::default()
            }
        } else {
            U256::default()
        }
    }
}
