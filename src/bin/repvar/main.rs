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
        .map(|variable_name_value_pair| {
            (
                variable_name_value_pair.variable_name_ref().as_str(),
                variable_name_value_pair.variable_value_ref().as_str(),
            )
        })
        .collect::<BTreeMap<&str, &str>>();

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let output = decode_string(input, |variable_name| variables.get(variable_name).copied())?;

    print!("{}", output);

    Ok(())
}
