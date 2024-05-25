use crate::commitment::create_commitment;

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

  pub fn respond_to_challenge(&self) -> (u8, u64) {
    (self.vote, self.nonce)
  }
}

impl VotingSystem {
  pub fn new(expected_commitment: Vec<u8>) -> Self {
    VotingSystem { expected_commitment }
  }

  pub fn verify(&self, vote: u8, nonce: u64) -> bool {
    let commitment = create_commitment(vote, nonce);
    commitment == self.expected_commitment
  }
}
