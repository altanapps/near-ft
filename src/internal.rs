use near_sdk::{require};

use crate::*;

impl Contract {
    pub (crate) fn internal_unwrap_balance_of(&self, account_id: &AccountId) -> Balance {
        match self.accounts_to_balance.get(account_id) {
            Some(balance) => balance,
            None => {
                env::panic_str(format!("The account {} is not registered", &account_id).as_str())
            }
        }
    }

    pub(crate) fn internal_deposit (&mut self, receiver_id: &AccountId,
    amount: Balance) {
        // Get the current balance of the account
        // If it fails, give it a zero
        let balance = self.accounts_to_balance.get(&receiver_id).unwrap_or(0);

        if let Some (new_balance) = balance.checked_add(amount) {
            self.accounts_to_balance.insert(&receiver_id, &new_balance);
        } else {
            env::panic_str("Balance overflow");
        }
    }

    /// Internal method for registering an account with the contract.
    pub(crate) fn internal_register_account(&mut self, account_id: &AccountId) {
        if self.accounts_to_balance.insert(account_id, &0).is_some() {
            env::panic_str("The account is already registered");
        }
    }

    pub(crate) fn internal_withdraw(&mut self, sender_id: &AccountId,
        amount: Balance) {
        let balance = self.internal_unwrap_balance_of(sender_id);

        if let Some(new_balance) = balance.checked_sub(amount) {
            self.accounts_to_balance.insert(sender_id, &new_balance);
        } else {
            env::panic_str("The account doesn't have enough balance");
        }
    }

    pub(crate) fn internal_transfer(&mut self,
    sender_id: &AccountId, receiver_id: &AccountId,
    amount: Balance, memo: Option<String>) {
        // Ensure the sender can't transfer to themselves
        require!(sender_id != receiver_id, "Sender and receiver should be different");
        // Ensure that the amount you will receive is greater than 0
        require!(amount>0, "Ensure that the amount sent is greater than 0");

        // Withdraw from the sender and deposit into the receiver
        self.internal_withdraw(sender_id, amount);
        self.internal_deposit(receiver_id, amount);

        // Emit a Transfer event
        FtTransfer {
            old_owner_id: sender_id,
            new_owner_id: receiver_id,
            amount: &U128(amount),
            memo:memo.as_deref(), 
        }.emit();

    }
}