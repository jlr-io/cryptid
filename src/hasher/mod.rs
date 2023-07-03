use blake2::{Blake2b512, Blake2s256};
use digest::{Digest};
use hex::encode_upper;
use fsb::{Fsb160, Fsb224, Fsb256, Fsb384, Fsb512};
use gost94::Gost94CryptoPro;
use groestl::Groestl256;
use md5::Md5;
use ripemd::{Ripemd160, Ripemd320};
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use shabal::{Shabal192, Shabal224, Shabal256, Shabal384, Shabal512};
use sm3::Sm3;
use streebog::*;
use tiger::Tiger;
use whirlpool::Whirlpool;

#[derive(Debug)]
pub enum HashError {
    InvalidHashAlgorithm(String),
}

impl std::fmt::Display for HashError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HashError::InvalidHashAlgorithm(algo) => write!(f, "Invalid hash algorithm: {}", algo),
        }
    }
}

pub struct Hasher
{
    digest: Box<dyn digest::DynDigest>,
}

impl Hasher
{
    pub fn new(algo: &str) -> Result<Hasher, HashError> {
        let digest = Hasher::get_digest(algo)?;
        Ok(Hasher { digest })
    }

    pub fn compute(mut self, bytes: &[u8]) -> String {
        self.digest.update(bytes);
        return encode_upper(self.digest.finalize());
    }

    fn get_digest(algo: &str) -> Result<Box<dyn digest::DynDigest>, HashError> {
        return Ok(match algo.to_lowercase().as_str() {
            "md5" => Box::new(Md5::default()),
            "sha1" => Box::new(Sha1::default()),
            "sha256" => Box::new(Sha256::default()),
            "sha384" => Box::new(Sha384::default()),
            "sha512" => Box::new(Sha512::default()),
            "sha3-224" => Box::new(Sha3_224::new()),
            "sha3-256" => Box::new(Sha3_256::new()),
            "sha3-384" => Box::new(Sha3_384::new()),
            "sha3-512" => Box::new(Sha3_512::new()),
            "ripemd160" => Box::new(Ripemd160::default()),
            "ripemd320" => Box::new(Ripemd320::default()),
            "blake2b512" => Box::new(Blake2b512::default()),
            "blake2s256" => Box::new(Blake2s256::default()),
            "groestl256" => Box::new(Groestl256::default()),
            "tiger" => Box::new(Tiger::default()),
            "whirlpool" => Box::new(Whirlpool::default()),
            "streebog256" => Box::new(Streebog256::default()),
            "streebog512" => Box::new(Streebog512::default()),
            "shabal192" => Box::new(Shabal192::new()),
            "shabal224" => Box::new(Shabal224::new()),
            "shabal256" => Box::new(Shabal256::new()),
            "shabal384" => Box::new(Shabal384::new()),
            "shabal512" => Box::new(Shabal512::new()),
            "sm3" => Box::new(Sm3::new()),
            "gost" => Box::new(Gost94CryptoPro::default()),
            "groestl" => Box::new(Groestl256::default()),
            "fsb160" => Box::new(Fsb160::default()),
            "fsb224" => Box::new(Fsb224::default()),
            "fsb256" => Box::new(Fsb256::default()),
            "fsb384" => Box::new(Fsb384::default()),
            "fsb512" => Box::new(Fsb512::default()),
            _ => return Err(HashError::InvalidHashAlgorithm(algo.to_string())),
        });
    }
}