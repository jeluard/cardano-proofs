use std::collections::BTreeMap;

use amaru_kernel::protocol_parameters::ProtocolParameters;
use amaru_ledger::{context, rules::{self, parse_block}};
use risc0_zkvm::guest::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_block: Vec<u8> = env::read();

    let block = parse_block(&raw_block[..])?;
    let inputs = BTreeMap::new(); 
    let validity = rules::validate_block(
        context::DefaultValidationContext::new(inputs),
        ProtocolParameters::default(),
        block,
    );

    env::commit(&"");

    Ok(())
}
