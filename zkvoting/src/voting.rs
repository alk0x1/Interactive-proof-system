use crate::commitment::{create_commitment, verify_commitment};

#[derive(Debug)]
pub struct Voter {
  vote: u8,
  nonce: u64,
  commitment: Vec<u8>,
}

#[derive(Debug)]
pub struct VotingSystem {
  expected_commitment: Vec<u8>,
}

impl Voter {
  pub fn new(vote: u8, nonce: u64) -> Self {
    let commitment = create_commitment(vote, nonce);
    Voter { vote, nonce, commitment }
  }

  pub fn get_commitment(&self) -> Vec<u8> {
    self.commitment.clone()
  }

  pub fn generate_zk_proof(&self) -> (u8, u64, Vec<u8>) {
    (self.vote, self.nonce, self.commitment.clone())
  }
}

impl VotingSystem {
  pub fn new(expected_commitment: Vec<u8>) -> Self {
    VotingSystem { expected_commitment }
  }

  pub fn verify_zk_proof(&self, vote: u8, nonce: u64, commitment: Vec<u8>) -> bool {
    verify_commitment(vote, nonce, &commitment) && commitment == self.expected_commitment
  }
}
