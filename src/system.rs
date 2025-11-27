use std::collections::BTreeMap;
use std::ops::{Add, AddAssign};
use num::traits::{CheckedAdd, CheckedSub, Zero,One};
type AccountId = String;
type Nonce = u32;
type BlockNumber = u32;
pub trait Config{
    type AccountId: Ord+Clone;
    type Nonce: Ord+Clone+Copy+Zero+Add<u32, Output=Nonce>+One;
    type BlockNumber: Ord+Clone+Copy+Zero+One;
}
#[derive(Debug)]
pub struct Pallet<T:Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}
impl<T:Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            block_number: <T as Config>::BlockNumber::zero(), nonce: BTreeMap::new()
        }
    }
    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }
    pub fn inc_block_number(&mut self) {
        self.block_number = self.block_number + T::BlockNumber::one();
    }
    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        self.nonce.insert(who.clone(), *self.nonce.get(who).unwrap_or(&T::Nonce::zero()) + T::Nonce::one());
    }
    pub fn nonce(&self, who: &T::AccountId) -> T::Nonce {
        *self.nonce.get(who).unwrap_or(&T::Nonce::zero())
    }
}
#[cfg(test)]
mod tests {
    use crate::system;
    use super::*;
    struct TestConfig;
    impl Config for TestConfig {
        type AccountId = String;
        type Nonce = u32;
        type BlockNumber = u32;
    }
#[test]
fn init_system() { 
    let system   = system::Pallet::<TestConfig>::new();
    assert_eq!(system.block_number(), 0);
    assert_eq!(system.nonce(&"alice".to_string()), 0);
}
#[test]
fn inc_block_number() { 
    let mut system = system::Pallet::<TestConfig>::new();
    system.inc_block_number();
    assert_eq!(system.block_number(), 1);
}
#[test]
fn inc_nonce() { 
    let mut system = system::Pallet::<TestConfig>::new();
    system.inc_nonce(&"alice".to_string());
    assert_eq!(system.nonce(&"alice".to_string()), 1);
}
}
