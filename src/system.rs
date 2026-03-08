use num::{CheckedAdd, CheckedSub, One, Zero, zero};
use std::{collections::BTreeMap, ops::AddAssign};

#[derive(Debug)]
pub struct Pallet<AccountId, BlockNumber, Nonce> {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>,
}

impl<AccountId, BlockNumber, Nonce> Pallet<AccountId, BlockNumber, Nonce>
where
    AccountId: Ord + Clone,
    BlockNumber: Copy + AddAssign + Zero + One + CheckedAdd,
    Nonce: Ord + Clone + Copy + Zero + One + CheckedAdd,
{
    pub fn new() -> Self {
        Self {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) -> Result<(), &'static str> {
        self.block_number = self
            .block_number
            .checked_add(&BlockNumber::one())
            .ok_or("Block number overflowed")?;
        Ok(())
    }

    pub fn inc_nonce(&mut self, who: &AccountId) -> Result<(), &'static str> {
        let nonce = self.nonce.get(who).copied().unwrap_or_else(Nonce::zero);
        let new_nonce = nonce
            .checked_add(&Nonce::one())
            .ok_or("Nonce overflowed!")?;
        self.nonce.insert(who.clone(), new_nonce);
        Ok(())
    }

    pub fn get_nonce(&self, who: &AccountId) -> Result<Nonce, &'static str> {
        let nonce = *self.nonce.get(who).ok_or("Error in reading nonce!")?;
        Ok(nonce)
    }
}

#[cfg(test)]
mod test {
    use crate::types::{AccountId, BlockNumber, Nonce};

    use super::*;

    #[test]
    fn init_system() {
        let system: Pallet<AccountId, BlockNumber, Nonce> = Pallet::new();
        assert_eq!(system.block_number, 0);
    }
}
