use crate::{parser_source_document::ParserSourceDocument, settings::Settings};
use std::fs;

#[derive(Debug)]
pub enum ReaderError {
    Io,
    Malformed,
}

pub fn read_document(settings: &Settings) -> Result<ParserSourceDocument, ReaderError> {
    let contents = fs::read_to_string(settings.input_file.clone()).map_err(|_| ReaderError::Io)?;
    serde_yml::from_str(&contents).map_err(|_| ReaderError::Malformed)
}
