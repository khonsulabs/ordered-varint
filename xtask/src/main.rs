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
    type Universal = DefaultConfig;
}

impl khonsu_tools::publish::Config for Config {
    fn paths() -> Vec<String> {
        vec![String::from(".")]
    }
}
