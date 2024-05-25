use rand::Rng;

#[derive(Debug, PartialEq)]
struct Prover {
    a: i32,
    b: i32,
}

#[derive(Debug, PartialEq)]
struct Verifier {
    c: i32,
    challenge: Option<i32>,
}

#[derive(Debug, PartialEq)]
struct Message {
    value: i32,
}

impl Prover {
    fn new(a: i32, b: i32) -> Self {
        Prover { a, b }
    }

    fn generate_message(&self) -> Message {
        Message { value: self.a + self.b }
    }

    fn respond_to_challenge(&self, challenge: i32) -> Message {
        Message { value: challenge }
    }
}

impl Verifier {
    fn new(c: i32) -> Self {
        Verifier { c, challenge: None }
    }

    fn generate_challenge(&mut self) -> i32 {
        let mut rng = rand::thread_rng();
        self.challenge = Some(rng.gen_range(1..10));
        self.challenge.unwrap()
    }

    fn verify(&self, message: Message) -> bool {
        message.value == self.c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prover_generate_message() {
        let prover = Prover::new(3, 4);
        let message = prover.generate_message();
        assert_eq!(message, Message { value: 7 });
    }

    #[test]
    fn test_verifier_generate_challenge() {
        let mut verifier = Verifier::new(7);
        let challenge = verifier.generate_challenge();
        assert!(challenge >= 1 && challenge < 10);
        assert_eq!(verifier.challenge, Some(challenge));
    }

    #[test]
    fn test_prover_respond_to_challenge() {
        let prover = Prover::new(3, 4);
        let challenge = 5;
        let response = prover.respond_to_challenge(challenge);
        assert_eq!(response, Message { value: challenge });
    }

    #[test]
    fn test_verifier_verify_correct_message() {
        let verifier = Verifier::new(7);
        let message = Message { value: 7 };
        assert!(verifier.verify(message));
    }

    #[test]
    fn test_verifier_verify_incorrect_message() {
        let verifier = Verifier::new(7);
        let message = Message { value: 8 };
        assert!(!verifier.verify(message));
    }

    #[test]
    fn test_interaction() {
        let a = 3;
        let b = 4;
        let c = 7;

        let prover = Prover::new(a, b);
        let mut verifier = Verifier::new(c);

        let initial_message = prover.generate_message();
        assert_eq!(initial_message, Message { value: 7 });

        let challenge = verifier.generate_challenge();
        assert!(challenge >= 1 && challenge < 10);

        let response_message = prover.respond_to_challenge(challenge);
        assert_eq!(response_message, Message { value: challenge });

        let is_valid = verifier.verify(initial_message);
        assert!(is_valid);
    }
}
