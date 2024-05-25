#[cfg(test)]
mod tests {
  use super::voting::{Voter, VotingSystem};

  #[test]
  fn test_voting_protocol() {
    let vote = 1;
    let nonce = 123456;

    let voter = Voter::new(vote, nonce);
    let commitment = voter.get_commitment();

    let voting_system = VotingSystem::new(commitment);

    let (vote, nonce) = voter.respond_to_challenge();
    let is_valid = voting_system.verify(vote, nonce);

    assert!(is_valid);
  }

  #[test]
  fn test_invalid_vote() {
    let vote = 1;
    let nonce = 123456;

    let voter = Voter::new(vote, nonce);
    let commitment = voter.get_commitment();

    let voting_system = VotingSystem::new(commitment);

    let invalid_vote = 2;
    let is_valid = voting_system.verify(invalid_vote, nonce);

    assert!(!is_valid);
  }

  #[test]
  fn test_invalid_nonce() {
    let vote = 1;
    let nonce = 123456;

    let voter = Voter::new(vote, nonce);
    let commitment = voter.get_commitment();

    let voting_system = VotingSystem::new(commitment);

    let invalid_nonce = 654321;
    let is_valid = voting_system.verify(vote, invalid_nonce);

    assert!(!is_valid);
  }
}
