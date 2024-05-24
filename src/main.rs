use rand::Rng;

struct Prover {
  a: i32,
  b: i32,
}

struct Verifier {
  c: i32,
  challenge: Option<i32>,
}

struct Message {
  value: i32,
}

impl Prover {
  fn new(a: i32, b: i32) -> Self {
    Prover { a, b }
  }

  fn generate_message(&self) -> Message {
    // message is just the sum of a and b
    Message { value: self.a + self.b }
  }

  fn respond_to_challenge(&self, challenge: i32) -> Message {
    // respond to the verifier's challenge
    Message { value: challenge }
  }
}

impl Verifier {
  fn new(c: i32) -> Self {
    Verifier { c, challenge: None }
  }

  fn generate_challenge(&mut self) -> i32 {
    // generate a random challenge
    let mut rng = rand::thread_rng();
    self.challenge = Some(rng.gen_range(1..10));
    self.challenge.unwrap()
  }

  fn verify(&self, message: Message) -> bool {
    // verify the prover's response
    message.value == self.c
  }
}

fn main() {
  let a = 3;
  let b = 4;
  let c = 7; // a + b

  let prover = Prover::new(a, b);
  let mut verifier = Verifier::new(c);

  let initial_message = prover.generate_message();
  println!("Prover's initial message: {}", initial_message.value);

  let challenge = verifier.generate_challenge();
  println!("Verifier's challenge: {}", challenge);

  let response_message = prover.respond_to_challenge(challenge);
  println!("Prover's response to challenge: {}", response_message.value);

  let is_valid = verifier.verify(response_message);
  println!("Verification result: {}", is_valid);
}
