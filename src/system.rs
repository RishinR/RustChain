use std::collections::BTreeMap;

pub struct Pallet {
    block_number: u32,
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    pub fn inc_block_number(&mut self) -> Result<(), &'static str> {
        self.block_number = self
            .block_number
            .checked_add(1)
            .ok_or("Block number overflowed")?;
        Ok(())
    }

    pub fn inc_nonce(&mut self, who: &String) -> Result<(), &'static str> {
        let nonce = self.nonce.get(who).unwrap_or(&0);
        let new_nonce = nonce.checked_add(1).ok_or("Nonce overflowed!")?;
        self.nonce.insert(who.clone(), new_nonce);
        Ok(())
    }

    pub fn get_nonce(&self, who: &String) -> Result<u32, &'static str> {
        let nonce = *self.nonce.get(who).ok_or("Error in reading nonce!")?;
        Ok(nonce)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init_system() {
        let system = Pallet::new();
        assert_eq!(system.block_number, 0);
    }
}
