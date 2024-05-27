use sha2::{Sha256, Digest};

pub fn create_commitment(vote: u8, nonce: u64) -> Vec<u8> {
  let mut hasher = Sha256::new();
  hasher.update(&[vote]);
  hasher.update(&nonce.to_le_bytes());
  hasher.finalize().to_vec()
}

pub fn verify_commitment(vote: u8, nonce: u64, commitment: &[u8]) -> bool {
  create_commitment(vote, nonce) == commitment
}
