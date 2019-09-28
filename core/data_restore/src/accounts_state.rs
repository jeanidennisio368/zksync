use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

use ff::{Field, PrimeField, PrimeFieldRepr};
use plasma::state::{OpSuccess, PlasmaState};

use models::node::operations::{
    TX_TYPE_BYTES_LEGTH, DepositOp, FranklinOp, FullExitOp, TransferOp, TransferToNewOp, WithdrawOp,
};
use models::node::priority_ops::{Deposit, FranklinPriorityOp, FullExit};
use models::node::tx::{Close, FranklinTx, Transfer, Withdraw};
use models::node::{AccountMap, Fr, AccountId};
use models::node::account::{Account, AccountAddress, AccountUpdate};
use crate::helpers::DataRestoreError;

/// Franklin Accounts states with data restore configuration
pub struct FranklinAccountsState {
    /// Accounts stored in a spase Merkle tree and current block number
    pub state: PlasmaState,
}

impl FranklinAccountsState {
    /// Creates empty Franklin Accounts states
    pub fn new() -> Self {
        Self {
            state: PlasmaState::empty(),
        }
    }

    /// Creates empty Franklin Accounts states
    pub fn load(accounts: AccountMap, current_block: u32) -> Self {
        Self {
            state: PlasmaState::new(accounts, current_block + 1),
        }
    }

    /// Updates Franklin Accounts states from Franklin op
    ///
    /// # Arguments
    ///
    /// * `op` - Franklin operation
    ///
    pub fn update_accounts_states_from_op(
        &mut self,
        op: &FranklinOp,
    ) {
        match op.clone() {
            FranklinOp::Deposit(_op) => {
                self.state.execute_priority_op(
                    FranklinPriorityOp::Deposit(_op.priority_op)
                );
            },
            FranklinOp::TransferToNew(_op) => {
                self.state.execute_tx(
                    FranklinTx::Transfer(_op.tx)
                );
            },
            FranklinOp::Withdraw(_op) => {
                self.state.execute_tx(
                    FranklinTx::Withdraw(_op.tx)
                );
            },
            FranklinOp::Close(_op) => {
                self.state.execute_tx(
                    FranklinTx::Close(_op.tx)
                );
            },
            FranklinOp::Transfer(_op) => {
                self.state.execute_tx(
                    FranklinTx::Transfer(_op.tx)
                );
            },
            FranklinOp::FullExit(_op) => {
                self.state.execute_priority_op(
                    FranklinPriorityOp::FullExit(_op.priority_op)
                );
            },
        }
    }

    /// Returns map of Franklin accounts ids and their descriptions
    pub fn get_accounts(&self) -> Vec<(u32, Account)> {
        self.state.get_accounts()
    }

    /// Returns sparse Merkle tree root hash
    pub fn root_hash(&self) -> Fr {
        self.state.root_hash()
    }

    /// Returns Franklin Account description by its id
    pub fn get_account_by_address(&self, address: &AccountAddress) -> Option<(AccountId, Account)> {
        self.state.get_account_by_address(address)
    }
}
