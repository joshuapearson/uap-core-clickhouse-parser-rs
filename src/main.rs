use clap::Parser;
use saphyr::Array;
use saphyr::Yaml;
use std::env;
use std::fs;
use std::path::PathBuf;

/// Program that creates yaml files that can be used in the Clickhouse DB to
/// create dictionaries for parsing User Agent strings based on the uap-core
/// set of regexes maintained at https://github.com/ua-parser/uap-core
/// This generates up to three output files, one each for devices, OS and user agents
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file that contains uap-core formatted yaml
    input_file: PathBuf,

    /// Output directory in which to write device, os and user agent files
    #[arg(long, default_value=get_default_outdir().into_os_string())]
    outdir: PathBuf,

    /// Output filename that will be written containing the devices regex yaml
    #[arg(long, default_value = "device.yaml")]
    device: String,

    /// Output filename that will be written containing the OS regex yaml
    #[arg(long, default_value = "os.yaml")]
    os: String,

    /// Output filename that will be written containing the user agent regex yaml
    #[arg(long, default_value = "user_agent.yaml")]
    user_agent: String,

    /// Force overwrite of existing output files
    #[arg(short, long, default_value_t = false)]
    force: bool,
}

fn main() {
    let args = Args::parse();
    let settings_result = Settings::try_from(args);
    let Ok(settings) = settings_result else {
        return;
    };
    println!("Reading file: {:?}", &settings.input_file);

    let uap_document_result = read_document(&settings);
    let Ok(uap_document) = uap_document_result else {
        println!("Error reading or parsing input file");
        return;
    };

    let mut output = OutputYaml {
        devices: None,
        os: None,
        ua: None,
    };

    if let Some(device_arr) = uap_document.get(&Yaml::String("device_parsers".to_owned())) {
        let Yaml::Array(devices) = device_arr else {
            println!("Error reading device section of input file");
            return;
        };
        output.devices = Some(generate_device_output(devices.clone()));
    }
    println!("Device output file: {:?}", &settings.device_file);
    println!("OS output file: {:?}", &settings.os_file);
    println!("User Agent output file: {:?}", &settings.user_agent_file);
}

#[derive(Debug)]
enum SettingsError {
    InvalidInputFile,
    InvalidOutputDir,
    ForceOptionRequired,
}

#[derive(Debug)]
struct Settings {
    input_file: PathBuf,
    device_file: PathBuf,
    os_file: PathBuf,
    user_agent_file: PathBuf,
}

impl TryFrom<Args> for Settings {
    type Error = SettingsError;
    fn try_from(value: Args) -> Result<Self, Self::Error> {
        if !value.input_file.is_file() {
            println!("Error: cannot find input file specified");
            return Err(SettingsError::InvalidInputFile);
        }

        if value.outdir.is_file() {
            println!("Error: invalid output directory specified");
            return Err(SettingsError::InvalidOutputDir);
        }

        let mut device_path = value.outdir.clone();
        device_path.push(&value.device);
        if !value.force && device_path.is_file() {
            println!("Error: device file will be overwritten, must use -f --force option");
            return Err(SettingsError::ForceOptionRequired);
        }

        let mut os_path = value.outdir.clone();
        os_path.push(&value.os);
        if !value.force && os_path.is_file() {
            println!("Error: os file will be overwritten, must use -f --force option");
            return Err(SettingsError::ForceOptionRequired);
        }

        let mut user_agent_path = value.outdir.clone();
        user_agent_path.push(&value.user_agent);
        if !value.force && user_agent_path.is_file() {
            println!("Error: user agent file will be overwritten, must use -f --force option");
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

fn get_default_outdir() -> PathBuf {
    let Ok(current_dir) = env::current_dir() else {
        panic!("Could not determine current working directory");
    };
    current_dir
}

enum ReaderError {
    Io,
    Malformed,
    YamlScan,
}

fn read_document(settings: &Settings) -> Result<saphyr::Hash, ReaderError> {
    let contents = fs::read_to_string(settings.input_file.clone()).map_err(|_| ReaderError::Io)?;
    let mut yaml_vec = Yaml::load_from_str(&contents).map_err(|_| ReaderError::YamlScan)?;
    if yaml_vec.len() > 1 {
        return Err(ReaderError::Malformed);
    }
    let Yaml::Hash(hash) = yaml_vec.pop().ok_or(ReaderError::Malformed)? else {
        return Err(ReaderError::Malformed);
    };
    Ok(hash)
}

struct OutputYaml {
    devices: Option<Yaml>,
    os: Option<Yaml>,
    ua: Option<Yaml>,
}

fn generate_device_output(devices: Array) -> Yaml {}
