use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub verbose: bool,

    #[arg(short, long)]
    pub total_hour: Option<String>,

    #[arg(short = 'd', long = "days")]
    pub work_days: Option<String>,

    #[arg(short, long, value_name = "FILE", default_value = "./setting.toml")]
    pub setting: PathBuf
}
