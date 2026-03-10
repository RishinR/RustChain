use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::{collections::BTreeMap, ops::AddAssign};

pub trait Config {
    type AccountId: Ord + Clone;
    type Balance: Copy + AddAssign + Zero + CheckedAdd + CheckedSub;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
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

    pub fn transfer(
        &mut self,
        caller: &T::AccountId,
        to: &T::AccountId,
        amount: T::Balance,
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

    struct TestConfig;
    impl Config for TestConfig {
        type AccountId = String;
        type Balance = u128;
    }

    #[test]
    fn test_transfer() {
        let mut balances: Pallet<TestConfig> = Pallet::new();

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
