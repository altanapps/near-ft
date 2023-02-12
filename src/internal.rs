use near_sdk::{require};

use crate::*;

impl Contract {
    // Crate means that 
    pub(crate) fn internal_deposit (&mut self, account_id: &AccountId,
    amount: Balance) {
        // Get the current balance of the account
        // If it fails, give it a zero
        let balance = self.accounts_to_balance.get(&account_id).unwrap_or(0);

        if let Some (new_balance) = balance.checked_add(amount) {
            self.accounts_to_balance.insert(&account_id, &new_balance);
        } else {
            env::panic_str("Balance overflow");
        }
    }
}