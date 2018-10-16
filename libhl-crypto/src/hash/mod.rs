use CryptoError;

#[derive(Debug)]
pub enum HashAlgorithm {
    SHA2_256,
    SHA2_384,
    SHA2_512
}

pub fn hash(algorithm: HashAlgorithm, message: &[u8]) -> Result<Vec<u8>, CryptoError> {
    match algorithm {
        HashAlgorithm::SHA2_256 => {
            let mut hash = sha2::SHA256Hash::new();
            hash.update(message);
            hash.digest()
        },
        HashAlgorithm::SHA2_384 => {
            let mut hash = sha2::SHA384Hash::new();
            hash.update(message);
            hash.digest()
        },
        HashAlgorithm::SHA2_512 => {
            let mut hash = sha2::SHA512Hash::new();
            hash.update(message);
            hash.digest()
        }
    }
}

pub trait Hasher {
    fn new() -> Self where Self : Sized;
    fn reset(&mut self);
    fn update(&mut self, data: &[u8]);
    fn digest(&mut self) -> Result<Vec<u8>, CryptoError>;
}

pub mod sha2;
