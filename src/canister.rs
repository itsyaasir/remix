use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use candid::CandidType;
use candid::Deserialize;
use candid::Principal;
use canister_sdk::ic_canister::init;
use canister_sdk::ic_canister::query;
use canister_sdk::ic_canister::update;
use canister_sdk::ic_canister::Canister;
use canister_sdk::ic_canister::PreUpdate;
use canister_sdk::ic_storage;
use canister_sdk::ic_storage::stable::Versioned;
use canister_sdk::ic_storage::IcStorage;

use crate::error::RemixError;

pub type Owner = Principal;
pub type CanisterID = Principal;
// TODO:: The state should contain what ?
#[derive(Debug, Clone, IcStorage, CandidType, Deserialize, Default)]
pub struct RemixState {
    canisters: HashMap<Owner, CanisterID>,
}

impl Versioned for RemixState {
    type Previous = ();

    fn upgrade(previous: Self::Previous) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, Canister)]
pub struct RemixCanister {
    #[id]
    pub id: Principal,
    #[state]
    pub state: Rc<RefCell<RemixState>>,
}

impl PreUpdate for RemixCanister {}

impl RemixCanister {
    #[init]
    fn init(&self) {
        //
    }

    #[query]
    pub fn get_canisters(&self) -> Vec<CanisterID> {
        todo!()
    }

    #[update]
    pub fn delete_canister(&mut self, id: CanisterID) -> Result<(), RemixError> {
        todo!()
    }

    #[update]
    pub fn upgrade(&mut self, id: CanisterID) -> Result<(), RemixError> {
        todo!()
    }
}
