mod commitment;
mod voting;

use voting::{Voter, VotingSystem};

fn main() {
  let vote = 1;
  let nonce = 123456;

  let voter = Voter::new(vote, nonce);
  let commitment = voter.get_commitment();

  println!("Voter's commitment: {:?}", commitment);

  let voting_system = VotingSystem::new(commitment.clone());

  let (vote, nonce, proof_commitment) = voter.generate_zk_proof();
  let is_valid = voting_system.verify_zk_proof(vote, nonce, proof_commitment);

  println!("Verification result: {}", is_valid);
}
