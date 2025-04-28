use std::collections::BTreeMap;

use amaru_kernel::protocol_parameters::ProtocolParameters;
use amaru_ledger::{context, rules::{self, parse_block}};
use risc0_zkvm::guest::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement your guest code here

    // read the input
    let input: String = env::read();

    let raw_block = hex::decode(input.clone())?;
    let block = parse_block(&raw_block[..])?;

    let inputs = BTreeMap::new();
    let _volatile_state = rules::validate_block(
        context::DefaultValidationContext::new(inputs),
        ProtocolParameters::default(),
        block,
    );

    // write public output to the journal
    env::commit(&input);

    Ok(())
}
