use std::path::{PathBuf};
use std::{fs, io};
use crate::hasher::{Hasher, HashError};

#[derive(Debug)]
pub enum FileError {
    InvalidHashAlgorithm(String),
    IoError(io::Error),
}

impl From<io::Error> for FileError {
    fn from(error: io::Error) -> Self {
        FileError::IoError(error)
    }
}

impl From<HashError> for FileError {
    fn from(error: HashError) -> Self {
        match error {
            HashError::InvalidHashAlgorithm(algo) => FileError::InvalidHashAlgorithm(algo),
        }
    }
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileError::IoError(error) => write!(f, "IO error: {}", error),
            FileError::InvalidHashAlgorithm(algo) => write!(f, "Invalid hash algorithm: {}", algo),
        }
    }
}

impl std::error::Error for FileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FileError::IoError(error) => Some(error),
            FileError::InvalidHashAlgorithm(_) => None,
        }
    }
}

#[derive(Debug)]
pub struct File {
    path: PathBuf
}

impl File {
    pub fn new(path: PathBuf) -> File {
        File {
            path
        }
    }

    fn read(&self) -> Result<Vec<u8>, FileError> {
        let mut reader = fs::File::open(&self.path)?;
        let mut bytes = vec![];
        io::copy(&mut reader, &mut bytes)?;
        Ok(bytes)
    }

    pub fn hash(&self, algo: &str) -> Result<String, FileError> {
        let hasher = Hasher::new(algo)?;
        let bytes = self.read()?;
        let hash = hasher.compute(&bytes);
        Ok(hash)
    } 
}