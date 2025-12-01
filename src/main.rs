mod balances;
mod system;
mod support;
mod proof_of_existence;
use crate::support::Dispatch;
/*enum RuntimeCall {
    Balances(balances::Call<Runtime>),
    ProofOfExistence(proof_of_existence::Call<Runtime>),
    //BalancesTransfer { to: types::AccountId, amount: types::Balance },
}*/
mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
    pub type Extrinsic=crate::support::Extrinsic<AccountId,crate::RuntimeCall>;
    pub type Header=crate::support::Header<BlockNumber>;
    pub type Block=crate::support::Block<Header,Extrinsic>;
    pub type Content = String;
}
#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet::<Self>,
    proof_of_existence: proof_of_existence::Pallet::<Self>,
}
impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}
impl balances::Config for Runtime {
	type Balance = types::Balance;
}
impl proof_of_existence::Config for Runtime {
	type content = types::Content;
}
/*impl Runtime {
    pub fn new() -> Self {
        Self { system: system::Pallet::new(), balances: balances::Pallet::new(), proof_of_existence: proof_of_existence::Pallet::new() }
    }
    pub fn execute_block(&mut self, block: types::Block) -> crate::support::DispatchResult {
        self.system.inc_block_number();
        if block.header.block_number != self.system.block_number() {
            return Err("Invalid block number");
        }
        for (i,support::Extrinsic{caller,call}) in block.extrinsic.into_iter().enumerate() {
        self.system.inc_nonce(&caller);
        let _res=self.dispatch(caller, call).map_err(|e| {eprintln!(
            "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
            block.header.block_number, i, e
        )});
        }
        Ok(())
    }
}
impl Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;
    fn dispatch(&mut self, caller: Self::Caller, runtime_call: Self::Call) -> crate::support::DispatchResult {
        match runtime_call {
            RuntimeCall::Balances(call) => {
                /*match call {
                    balances::Call::Transfer { to, amount } => {
                        self.balances.transfer(&caller, &to, amount)?;
                    }
                }*/
                self.balances.dispatch(caller, call)?;
            }
            RuntimeCall::ProofOfExistence(call) => {
                self.proof_of_existence.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}*/
fn main() {
    let mut runtime = Runtime::new();
    runtime.balances.set_balance(&"alice".to_string(), 100);
    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsic: vec![
            support::Extrinsic { caller: "alice".to_string(), call: RuntimeCall::balances(balances::Call::transfer { to: "bob".to_string(), amount: 30 }) },
            support::Extrinsic { caller: "bob".to_string(), call: RuntimeCall::balances(balances::Call::transfer { to: "charlie".to_string(), amount: 20 }) },
        ],
    };
    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsic: vec![
            support::Extrinsic { caller: "alice".to_string(), call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim { content: "Hello, world!".to_string() }) },
            support::Extrinsic { caller: "bob".to_string(), call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim { content: "Hello, world!".to_string() }) },
        ],
    };
    let block_3 = types::Block {
        header: support::Header { block_number: 3 },
        extrinsic: vec![
            support::Extrinsic { caller: "alice".to_string(), call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim { claim: "Hello, world!".to_string() }) },
            support::Extrinsic { caller: "bob".to_string(), call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim { claim: "Hello, world!".to_string() }) },
        ],
    };
    runtime.execute_block(block_1).unwrap();
    runtime.execute_block(block_2).unwrap();
    runtime.execute_block(block_3).unwrap();
}
