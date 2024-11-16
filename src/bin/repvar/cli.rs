use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct ServeParams {
    #[arg(
        short('l'),
        long("listener-address"),
        help("Address where the proxy server accepts the connections (e.g., 127.0.0.1:8000)")
    )]
    pub listener_address: String,
}

#[derive(Debug, Parser)]
pub struct QueryParams {
    #[arg(
        short('q'),
        long("query-path"),
        help("Path to the query file (e.g., ./query.graphql)")
    )]
    pub query_path: PathBuf,

    #[arg(
        short('a'),
        long("server-address"),
        help("Address where the server accepts the connections (e.g., https://someserver/api/graphql)")
    )]
    pub server_address: String,
}

#[derive(Debug, Parser)]
pub enum Command {
    Serve(ServeParams),
    Sdl,
    Query(QueryParams),
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(
        short('v'),
        long("variable"),
        help("Variable to be passed to the resolver (e.g., -v name=John)")
    )]
    pub variables: Vec<String>,
}
