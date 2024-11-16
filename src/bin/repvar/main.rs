#![forbid(unsafe_code)]

use clap::Parser;
use std::{collections::BTreeMap, io::Read};
use variable_resolver::decode_string;

mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

    let variables = cli
        .variables
        .iter()
        .map(|key_value_pair| {
            let mut parts = key_value_pair.split('=');
            let key = parts
                .next()
                .ok_or_else(|| InvalidVariableDefinition(key_value_pair.clone()))?;
            let value = parts
                .next()
                .ok_or_else(|| InvalidVariableDefinition(key_value_pair.clone()))?;

            if parts.next().is_some() {
                return Err(InvalidVariableDefinition(key_value_pair.to_string()));
            }

            Ok((key, value))
        })
        .collect::<Result<BTreeMap<&str, &str>, InvalidVariableDefinition>>()?;

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let output = decode_string(input, |variable_name| variables.get(variable_name).copied())?;

    print!("{}", output);

    Ok(())
}

#[derive(Debug, thiserror::Error)]
#[error("InvalidVariableDefinition = '{0}'")]
pub struct InvalidVariableDefinition(String);
