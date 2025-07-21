use std::collections::BTreeMap;

use amaru_kernel::{EraHistory, network::NetworkName, protocol_parameters::GlobalParameters};
use amaru_ledger::{context, rules::{self, parse_block, block::BlockValidation}, state::State, store::in_memory::MemoryStore};
use risc0_zkvm::guest::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_block: Vec<u8> = env::read();

    let block = parse_block(&raw_block[..])?;

    let network = NetworkName::Preprod;
    let era_history: &EraHistory = network.into();

    let global_parameters: &GlobalParameters = network.into();
    let state = State::new(
        MemoryStore {},
        MemoryStore {},
        era_history.clone(),
        global_parameters.clone(),
    )?;

    let inputs = BTreeMap::new(); 
    let mut context = context::DefaultValidationContext::new(inputs);
    if let BlockValidation::Invalid(_slot, _id, err) = rules::validate_block(&mut context, state.protocol_parameters(), &block) {
         println!("Failed to validate block: {:?}", err);
    }

    env::commit(&"");

    Ok(())
}
