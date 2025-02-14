use clap::Parser;
use std::{env, path::PathBuf};

/// Program that creates yaml files that can be used in the Clickhouse DB to
/// create dictionaries for parsing User Agent strings based on the uap-core
/// set of regexes maintained at https://github.com/ua-parser/uap-core
/// This generates up to three output files, one each for devices, OS and user agents
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Input file that contains uap-core formatted yaml
    pub input_file: PathBuf,

    /// Output directory in which to write device, os and user agent files
    #[arg(long, default_value=get_default_outdir().into_os_string())]
    pub outdir: PathBuf,

    /// Output filename that will be written containing the devices regex yaml
    #[arg(long, default_value = "device.yaml")]
    pub device: String,

    /// Output filename that will be written containing the OS regex yaml
    #[arg(long, default_value = "os.yaml")]
    pub os: String,

    /// Output filename that will be written containing the user agent regex yaml
    #[arg(long, default_value = "user_agent.yaml")]
    pub user_agent: String,

    /// Force overwrite of existing output files
    #[arg(short, long, default_value_t = false)]
    pub force: bool,
}

fn get_default_outdir() -> PathBuf {
    let Ok(current_dir) = env::current_dir() else {
        panic!("Could not determine current working directory");
    };
    current_dir
}
