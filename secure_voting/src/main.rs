mod commitment;
mod voting;

use voting::{Voter, VotingSystem};

fn main() {
  let vote = 1;
  let nonce = 123456;

  let voter = Voter::new(vote, nonce);
  let commitment = voter.get_commitment();

  println!("Voter's commitment: {:?}", commitment);

  let voting_system = VotingSystem::new(commitment);

  let (vote, nonce) = voter.respond_to_challenge();
  let is_valid = voting_system.verify(vote, nonce);

  println!("Verification result: {}", is_valid);
}
