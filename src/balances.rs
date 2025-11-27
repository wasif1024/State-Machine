use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};
pub trait Config:crate::system::Config{
    //type AccountId: Ord+Clone;
    type Balance: CheckedAdd + CheckedSub + Zero+Clone+Copy;
}
pub enum Call<T: Config> {
	Transfer { to: T::AccountId, amount: T::Balance },
}
#[derive(Debug)]
pub struct Pallet<T:Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}
impl<T:Config> Pallet<T> where T::AccountId: Ord+Clone, T::Balance: CheckedAdd + CheckedSub + Zero+Clone+Copy{
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }
    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
		self.balances.insert(who.clone(), amount);
	}
    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }
    pub fn transfer(&mut self, from: &T::AccountId, to: &T::AccountId, amount: T::Balance) -> Result<(), &'static str> {
        let check_balance = self.balance(from).checked_sub(&amount).ok_or("Insufficient balance");
        if check_balance.is_err() {
            return Err(check_balance.err().unwrap());
        }
        self.balances.insert(from.clone(), self.balance(from) - amount);
        self.balances.insert(to.clone(), self.balance(to) + amount);
        Ok(())
    }
    
}
impl<T:Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;
    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> crate::support::DispatchResult {
        match call {
            Call::Transfer { to, amount } => {
                self.transfer(&caller, &to, amount)?;
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use crate::balances;
    use crate::system;
    use super::*;
    struct TestConfig;
    impl system::Config for TestConfig {
        type AccountId = String;
        type Nonce = u32;
        type BlockNumber = u32;
    }
    impl Config for TestConfig {
        type Balance = u128;
    }
#[test]
fn init_balances() { 
    let mut balances = balances::Pallet::<TestConfig>::new();
    assert_eq!(balances.balance(&"alice".to_string()), 0);
	balances.set_balance(&"alice".to_string(), 100);
	assert_eq!(balances.balance(&"alice".to_string()), 100);
	assert_eq!(balances.balance(&"bob".to_string()), 0);
}
#[test]
fn transfer_balances() { 
    let mut balances = balances::Pallet::<TestConfig>::new();
    balances.set_balance(&"alice".to_string(), 100);
    balances.set_balance(&"bob".to_string(), 0);
    let _res=balances.transfer(&"alice".to_string(), &"bob".to_string(), 50);
    assert_eq!(balances.balance(&"alice".to_string()), 50);
    assert_eq!(balances.balance(&"bob".to_string()), 50);
}
}