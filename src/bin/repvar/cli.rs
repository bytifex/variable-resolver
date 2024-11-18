use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(
        short('v'),
        long("variable"),
        help("Variable to be passed to the resolver (e.g., -v name=Jane)")
    )]
    pub variables: Vec<String>,
}
