use anyhow::Result;
use std::path::Path;
use std::{fs, io};
use crate::hasher::Hasher;

#[derive(Debug)]
pub struct File<'a> {
    path: &'a Path
}

impl<'a> File<'a> {
    pub fn new(path: &'a Path) -> File {
        File {
            path
        }
    }

    fn read(&self) -> Result<Vec<u8>> {
        let mut reader = fs::File::open(&self.path)?;
        let mut bytes = vec![];
        io::copy(&mut reader, &mut bytes)?;
        Ok(bytes)
    }

    pub fn hash(&self, algo: &str) -> Result<String> {
        let hasher = Hasher::new(algo)?;
        let bytes = self.read()?;
        let hash = hasher.compute(&bytes);
        Ok(hash)
    } 
}