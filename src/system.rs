use num::{CheckedAdd, CheckedSub, One, Zero, zero};
use std::{collections::BTreeMap, ops::AddAssign};

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Copy + AddAssign + Zero + One + CheckedAdd;
    type Nonce: Ord + Clone + Copy + Zero + One + CheckedAdd;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) -> Result<(), &'static str> {
        self.block_number = self
            .block_number
            .checked_add(&T::BlockNumber::one())
            .ok_or("Block number overflowed")?;
        Ok(())
    }

    pub fn inc_nonce(&mut self, who: &T::AccountId) -> Result<(), &'static str> {
        let nonce = self.nonce.get(who).copied().unwrap_or_else(T::Nonce::zero);
        let new_nonce = nonce
            .checked_add(&T::Nonce::one())
            .ok_or("Nonce overflowed!")?;
        self.nonce.insert(who.clone(), new_nonce);
        Ok(())
    }

    pub fn get_nonce(&self, who: &T::AccountId) -> Result<T::Nonce, &'static str> {
        let nonce = *self.nonce.get(who).ok_or("Error in reading nonce!")?;
        Ok(nonce)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    struct TestConfig;
    impl Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u128;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        let system: Pallet<TestConfig> = Pallet::new();
        assert_eq!(system.block_number, 0);
    }
}
