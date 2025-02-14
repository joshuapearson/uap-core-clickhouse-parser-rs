use args::Args;
use clap::Parser;
use parser_source_document::ParserSourceDocument;
use parser_target_documents::{DeviceParserTarget, OsParserTarget, UserAgentParserTarget};
use serde::Serialize;
use settings::Settings;
use std::{fs, path::PathBuf};

mod args;
mod parser_source_document;
mod parser_target_documents;
mod settings;

fn main() {
    let args = Args::parse();
    let settings_result = Settings::try_from(args);
    let Ok(settings) = settings_result else {
        return;
    };
    println!("Reading file: {:?}", &settings.input_file);

    let source_result = read_document(&settings);
    let Ok(source) = source_result else {
        let err = source_result.unwrap_err();
        println!("Error reading source document: {:?}", err);
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
        println!(
            "Error writing out devices file {:?}",
            device_result.unwrap_err()
        );
        return;
    }

    let os_target: Vec<OsParserTarget> = source.os_parsers.into_iter().map(|o| o.into()).collect();
    println!("OS output file: {:?}", &settings.os_file);
    let os_result = write_target(settings.os_file, os_target);
    if os_result.is_err() {
        println!("Error writing out OS file {:?}", os_result.unwrap_err());
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
        println!(
            "Error writing out User Agent file {:?}",
            ua_result.unwrap_err()
        );
        return;
    }
    println!("Success");
}

#[derive(Debug)]
pub enum ReaderError {
    Io,
    Malformed,
}

fn read_document(settings: &Settings) -> Result<ParserSourceDocument, ReaderError> {
    let contents = fs::read_to_string(settings.input_file.clone()).map_err(|_| ReaderError::Io)?;
    serde_yml::from_str(&contents).map_err(|_| ReaderError::Malformed)
}

#[derive(Debug)]
pub enum WriterError {
    Io,
    Serialize,
}

fn write_target<T>(path: PathBuf, target: Vec<T>) -> Result<(), WriterError>
where
    T: Sized + Serialize,
{
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .map_err(|e| {
            println!("IO Error: {:?}", e);
            WriterError::Io
        })?;
    serde_yml::to_writer(file, &target).map_err(|e| {
        println!("Serialize Error: {:?}", e);
        WriterError::Serialize
    })
}
