use khonsu_tools::universal::clap::Parser;
use khonsu_tools::universal::{anyhow, DefaultConfig};
use khonsu_tools::Commands;

fn main() -> anyhow::Result<()> {
    let command = Commands::parse();
    command.execute::<Config>()
}

struct Config;

impl khonsu_tools::Config for Config {
    type Publish = Self;
    type Universal = Self;
}

impl khonsu_tools::publish::Config for Config {
    fn paths() -> Vec<String> {
        vec![String::from(".")]
    }
}

impl khonsu_tools::universal::Config for Config {
    type Audit = DefaultConfig;
    type CodeCoverage = Self;
}

impl khonsu_tools::universal::code_coverage::Config for Config {
    fn ignore_paths() -> Vec<String> {
        vec![String::from("examples/*")]
    }
}
