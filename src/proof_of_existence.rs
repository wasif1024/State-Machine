use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;
pub trait Config:crate::system::Config{
    type content:Debug+Ord;
}
pub enum Call<T: Config> {
	CreateClaim { content: T::content },
	RevokeClaim { content: T::content },
}
#[derive(Debug)]
pub struct Pallet<T:Config> {
    claims: BTreeMap<T::content, T::AccountId>,
}
impl<T:Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }
    pub fn get_claim(&mut self, content: T::content) -> Option<&T::AccountId> {
        //unimplemented!()
        self.claims.get(&content)
    }
    pub fn create_claim(&mut self, caller: T::AccountId, content: T::content) -> DispatchResult {
       if self.claims.contains_key(&content){
        return Err("Claim already exists");
       }
       self.claims.insert(content, caller);
        Ok(())
    }
    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::content) -> DispatchResult {
        //Ok(())
        let owner = self.claims.get(&claim).ok_or("Claim not found")?;
        if owner != &caller{
            return Err("You are not the owner of the claim");
        }
        self.claims.remove(&claim);
        Ok(())
    }
}
impl<T:Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;
    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> crate::support::DispatchResult {
        match call {
            Call::CreateClaim { content } => {
                self.create_claim(caller, content)?;
                //self.transfer(&caller, &to, amount)?;
            }
            Call::RevokeClaim { content } => {
                self.revoke_claim(caller, content)?;
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use crate::proof_of_existence;
    use crate::system;
    use super::*;
    struct TestConfig;
    impl system::Config for TestConfig {
        type AccountId = String;
        type Nonce = u32;
        type BlockNumber = u32;
    }
    impl Config for TestConfig {
        type content = String;
    }
    #[test]
    fn basic_proof_of_existence() {
        let mut proof_of_existence = proof_of_existence::Pallet::<TestConfig>::new();
        assert_eq!(proof_of_existence.get_claim("Hello".to_string()), None);
        assert_eq!(proof_of_existence.create_claim("alice".to_string(), "Hello".to_string()).is_ok(), true);
        assert_eq!(proof_of_existence.get_claim("Hello".to_string()), Some(&"alice".to_string()));
        assert_eq!(proof_of_existence.revoke_claim("alice".to_string(), "Hello".to_string()).is_ok(), true);
        assert_eq!(proof_of_existence.get_claim("Hello".to_string()), None);
    }

}