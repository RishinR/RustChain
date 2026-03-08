use std::collections::BTreeMap;
use std::error::Error;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn transfer(
        &mut self,
        caller: &String,
        to: &String,
        amount: u128,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(caller);
        let to_balance = self.balance(to);

        let new_caller_balance = caller_balance
            .checked_sub(amount)
            .ok_or("Insufficient Balance!")?;
        let new_to_balance = to_balance
            .checked_add(amount)
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
