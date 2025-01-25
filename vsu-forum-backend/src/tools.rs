use sha3::{Digest, Sha3_256};

pub fn hash_text(text: String) -> Vec<u8> {
    let mut hasher = Sha3_256::new();
    hasher.update(text.as_bytes());
    hasher.finalize().to_vec()
}
