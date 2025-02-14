use std::path::PathBuf;

use crate::args::Args;

#[derive(Debug)]
pub enum SettingsError {
    InvalidInputFile,
    InvalidOutputDir,
    ForceOptionRequired,
}

#[derive(Debug)]
pub struct Settings {
    pub input_file: PathBuf,
    pub device_file: PathBuf,
    pub os_file: PathBuf,
    pub user_agent_file: PathBuf,
}

impl TryFrom<Args> for Settings {
    type Error = SettingsError;
    fn try_from(value: Args) -> Result<Self, Self::Error> {
        if !value.input_file.is_file() {
            eprintln!("Error: cannot find input file specified");
            return Err(SettingsError::InvalidInputFile);
        }

        if value.outdir.is_file() {
            eprintln!("Error: invalid output directory specified");
            return Err(SettingsError::InvalidOutputDir);
        }

        let mut device_path = value.outdir.clone();
        device_path.push(&value.device);
        if !value.force && device_path.is_file() {
            eprintln!("Error: device file will be overwritten, must use -f --force option");
            return Err(SettingsError::ForceOptionRequired);
        }

        let mut os_path = value.outdir.clone();
        os_path.push(&value.os);
        if !value.force && os_path.is_file() {
            eprintln!("Error: os file will be overwritten, must use -f --force option");
            return Err(SettingsError::ForceOptionRequired);
        }

        let mut user_agent_path = value.outdir.clone();
        user_agent_path.push(&value.user_agent);
        if !value.force && user_agent_path.is_file() {
            eprintln!("Error: user agent file will be overwritten, must use -f --force option");
            return Err(SettingsError::ForceOptionRequired);
        }

        Ok(Self {
            input_file: value.input_file,
            device_file: device_path,
            os_file: os_path,
            user_agent_file: user_agent_path,
        })
    }
}
