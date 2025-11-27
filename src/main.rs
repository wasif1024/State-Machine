mod balances;
mod system;
mod support;
use crate::support::Dispatch;
enum RuntimeCall {
    Balances(balances::Call<Runtime>),
    //BalancesTransfer { to: types::AccountId, amount: types::Balance },
}
mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
    pub type Extrinsic=crate::support::Extrinsic<AccountId,crate::RuntimeCall>;
    pub type Header=crate::support::Header<BlockNumber>;
    pub type Block=crate::support::Block<Header,Extrinsic>;
}
#[derive(Debug)]
struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet::<Self>,
}
impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}
impl balances::Config for Runtime {
	type Balance = types::Balance;
}
impl Runtime {
    pub fn new() -> Self {
        Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
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
                match call {
                    balances::Call::Transfer { to, amount } => {
                        self.balances.transfer(&caller, &to, amount)?;
                    }
                }
            }
        }
        Ok(())
    }
}
fn main() {
    let mut runtime = Runtime::new();
    runtime.balances.set_balance(&"alice".to_string(), 100);
    let block_1 = types::Block {
        header: types::Header { block_number: 1 },
        extrinsic: vec![
            types::Extrinsic { caller: "alice".to_string(), call: RuntimeCall::Balances(balances::Call::Transfer { to: "bob".to_string(), amount: 30 }) },
            types::Extrinsic { caller: "bob".to_string(), call: RuntimeCall::Balances(balances::Call::Transfer { to: "charlie".to_string(), amount: 20 }) },
        ],
    };
    runtime.execute_block(block_1).unwrap();
}
