use fvm_shared::ActorID;
use fvm_utils::u256::U256;
pub trait FRC20 {
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn decimals(&self) -> u8;
    fn total_supply(&self) -> U256;
    fn balance_of(&self, _owner: ActorID) -> U256;
    fn transfer(&mut self, _to: ActorID, _value: U256) -> bool;
    fn transfer_from(&mut self, _from: ActorID, _to: ActorID, _value: U256) -> bool;
    fn approve(&mut self, _spender: ActorID, _value: U256) -> bool;
    fn allowance(&mut self, _owner: ActorID, _spender: ActorID) -> U256;
}
