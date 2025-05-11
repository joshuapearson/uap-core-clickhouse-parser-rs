use args::Args;
use clap::Parser;
use parser_target_documents::{DeviceParserTarget, OsParserTarget, UserAgentParserTarget};
use settings::Settings;
use source_reader::read_document;
use target_writer::write_target;

mod args;
mod parser_source_document;
mod parser_target_documents;
mod settings;
mod source_reader;
mod target_writer;

fn main() {
    let args = Args::parse();
    let Ok(settings) = Settings::try_from(args) else {
        return;
    };
    println!("Reading file: {:?}", &settings.input_file);

    let source_result = read_document(&settings);
    let Ok(source) = source_result else {
        let err = source_result.unwrap_err();
        eprintln!("Error reading source document: {:?}", err);
        return;
    };

    println!("Found {} device rules", source.device_parsers.len());
    println!("Found {} os rules", source.os_parsers.len());
    println!("Found {} user agent rules", source.user_agent_parsers.len());

    let device_target: Vec<DeviceParserTarget> = source
        .device_parsers
        .into_iter()
        .map(|d| d.into())
        .collect();
    println!("Device output file: {:?}", &settings.device_file);
    let device_result = write_target(settings.device_file, device_target);
    if device_result.is_err() {
        eprintln!(
            "Error writing out devices file {:?}",
            device_result.unwrap_err()
        );
        return;
    }

    let os_target: Vec<OsParserTarget> = source.os_parsers.into_iter().map(|o| o.into()).collect();
    println!("OS output file: {:?}", &settings.os_file);
    let os_result = write_target(settings.os_file, os_target);
    if os_result.is_err() {
        eprintln!("Error writing out OS file {:?}", os_result.unwrap_err());
        return;
    }

    let ua_target: Vec<UserAgentParserTarget> = source
        .user_agent_parsers
        .into_iter()
        .map(|u| u.into())
        .collect();
    println!("User Agent output file: {:?}", &settings.user_agent_file);
    let ua_result = write_target(settings.user_agent_file, ua_target);
    if ua_result.is_err() {
        eprintln!(
            "Error writing out User Agent file {:?}",
            ua_result.unwrap_err()
        );
        return;
    }
    println!("Success");
}
