use clap::{builder::TypedValueParser, Arg, Command, Error, Parser};

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(
        short('v'),
        long("variable"),
        value_parser(VariableNameValuePairParser),
        help("Variable to be passed to the resolver (e.g., -v name=Jane)")
    )]
    pub variables: Vec<VariableNameValuePair>,
}

#[derive(Debug, Clone)]
pub struct VariableNameValuePair(String, String);

impl VariableNameValuePair {
    pub fn new(variable_name: impl Into<String>, variable_value: impl Into<String>) -> Self {
        Self(variable_name.into(), variable_value.into())
    }

    pub fn variable_name_ref(&self) -> &String {
        &self.0
    }

    pub fn variable_value_ref(&self) -> &String {
        &self.1
    }
}

#[derive(Debug, Clone)]
struct VariableNameValuePairParser;

impl TypedValueParser for VariableNameValuePairParser {
    type Value = VariableNameValuePair;

    fn parse_ref(
        &self,
        _cmd: &Command,
        _arg: Option<&Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, Error> {
        let value = value.to_string_lossy();

        Ok(if let Some(mid) = value.find("=") {
            VariableNameValuePair::new(&value[..mid], &value[mid + 1..])
        } else {
            VariableNameValuePair::new(value, "")
        })
    }
}
