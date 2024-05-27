#[cfg(test)]
mod tests {
  use super::voting::{Voter, VotingSystem};

  #[test]
  fn test_zkp_voting_protocol() {
    let vote = 1;
    let nonce = 123456;

    let voter = Voter::new(vote, nonce);
    let commitment = voter.get_commitment();

    let voting_system = VotingSystem::new(commitment);

    let (vote, nonce, proof_commitment) = voter.generate_zk_proof();
    let is_valid = voting_system.verify_zk_proof(vote, nonce, proof_commitment);

    assert!(is_valid);
  }

  #[test]
  fn test_zkp_invalid_vote() {
    let vote = 1;
    let nonce = 123456;

    let voter = Voter::new(vote, nonce);
    let commitment = voter.get_commitment();

    let voting_system = VotingSystem::new(commitment.clone());

    let invalid_vote = 2;
    let is_valid = voting_system.verify_zk_proof(invalid_vote, nonce, commitment.clone());

    assert!(!is_valid);
  }

  #[test]
  fn test_zkp_invalid_nonce() {
    let vote = 1;
    let nonce = 123456;

    let voter = Voter::new(vote, nonce);
    let commitment = voter.get_commitment();

    let voting_system = VotingSystem::new(commitment.clone());

    let invalid_nonce = 654321;
    let is_valid = voting_system.verify_zk_proof(vote, invalid_nonce, commitment.clone());

    assert!(!is_valid);
  }
}
