mod balances;
mod system;

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Runtime>,
    balances: balances::Pallet<Runtime>,
}

impl system::Config for Runtime {
    type AccountId = String;
    type BlockNumber = u128;
    type Nonce = u32;
}

impl balances::Config for Runtime {
    type AccountId = String;
    type Balance = u128;
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() -> Result<(), &'static str>{
    let mut runtime = Runtime::new();

    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    // Genesis
    runtime.balances.set_balance(&alice, 100);
    runtime.system.inc_block_number()?;

    // Check block number is 1
    assert_eq!(runtime.system.block_number(), 1);

    // Increement the nonce of the user
    runtime.system.inc_nonce(&alice)?;

    // Transfer funds
    let _ = runtime
        .balances
        .transfer(&alice, &bob, 30)
        .map_err(|e| println!("Error: {:?}", e));

    // Increement the nonce of the user
    runtime.system.inc_nonce(&alice)?;

    let _ = runtime
        .balances
        .transfer(&alice, &charlie, 20)
        .map_err(|e| println!("Error: {:?}", e));

    // Increement the nonce of the user
    runtime.system.inc_nonce(&alice)?;

    println!("Runtime is {:#?}", runtime);

    Ok(())
}
