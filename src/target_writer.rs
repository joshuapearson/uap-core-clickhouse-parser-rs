use serde::Serialize;
use std::{fs, path::PathBuf};

#[derive(Debug)]
pub enum WriterError {
    Io,
    Serialize,
}

pub fn write_target<T>(path: PathBuf, target: Vec<T>) -> Result<(), WriterError>
where
    T: Sized + Serialize,
{
    let dir_path = path.parent().ok_or(WriterError::Io)?;
    let dir_result = fs::create_dir_all(dir_path);
    if dir_result.is_err() {
        eprintln!(
            "Error creating directory for output files: {:?}",
            dir_result.unwrap_err()
        );
        return Err(WriterError::Io);
    }

    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .map_err(|e| {
            eprintln!("IO Error: {:?}", e);
            WriterError::Io
        })?;
    serde_yaml_ng::to_writer(file, &target).map_err(|e| {
        eprintln!("Serialize Error: {:?}", e);
        WriterError::Serialize
    })
}
