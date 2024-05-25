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

pub fn run() {
  let a = 3;
  let b = 4;
  let c = 7; // a + b

  let prover = Prover::new(a, b);
  let mut verifier = Verifier::new(c);

  // Step 1: Prover sends initial message
  let initial_message = prover.generate_message();
  println!("Prover's initial message: {}", initial_message.value);

  // Step 2: Verifier generates a challenge
  let challenge = verifier.generate_challenge();
  println!("Verifier's challenge: {}", challenge);

  // Step 3: Prover responds to the challenge
  let response_message = prover.respond_to_challenge(challenge);
  println!("Prover's response to challenge: {}", response_message.value);

  // Step 4: Verifier verifies the response
  let is_valid = verifier.verify(initial_message);
  println!("Verification result: {}", is_valid);
}
