use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
    balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId, Balance> Pallet<AccountId, Balance>
where
    AccountId: Ord + Clone,
    Balance: Zero + CheckedAdd + CheckedSub + Copy,
{
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &AccountId) -> Balance {
        *self.balances.get(who).unwrap_or(&Balance::zero())
    }

    pub fn transfer(
        &mut self,
        caller: &AccountId,
        to: &AccountId,
        amount: Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(caller);
        let to_balance = self.balance(to);

        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Insufficient Balance!")?;
        let new_to_balance = to_balance
            .checked_add(&amount)
            .ok_or("Overflow Adding Balance!")?;

        self.set_balance(caller, new_caller_balance);
        self.set_balance(to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer() {
        let mut balances = Pallet::new();

        let alice = "alice".to_string();
        let bob = "bob".to_string();

        balances.set_balance(&alice, 100);
        balances.set_balance(&bob, 0);

        let result = balances.transfer(&alice, &bob, 200);
        assert_eq!(result, Err("Insufficient Balance!"));
        assert_eq!(balances.balance(&alice), 100);
        assert_eq!(balances.balance(&bob), 0);
    }

    #[test]
    fn transfer_overflow() {
        let mut balances = Pallet::new();

        let alice = "alice".to_string();
        let bob = "bob".to_string();

        balances.set_balance(&alice, 100);
        balances.set_balance(&bob, u128::MAX);

        let result = balances.transfer(&alice, &bob, 1);
        assert_eq!(result, Err("Overflow Adding Balance!"));
        assert_eq!(balances.balance(&alice), 100);
        assert_eq!(balances.balance(&bob), u128::MAX);
    }
}
