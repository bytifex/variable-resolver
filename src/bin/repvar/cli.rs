use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(
        short('v'),
        long("variable"),
        help("Variable to be passed to the resolver (e.g., -v name=John)")
    )]
    pub variables: Vec<String>,
}
