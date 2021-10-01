use simple_error::{bail, SimpleError};
use rand::{distributions::Alphanumeric, Rng};

pub struct Challenge {
    num_zeros: usize,
    text: String,
    counter: u128,
}

impl Challenge {
    pub fn new(num_zeros: usize) -> Self {
        Challenge {
            num_zeros,
            text: Self::generate_random_text(),
            counter: 0,
        }
    }

    pub fn verify(&self, answer: &str) -> Result<(), SimpleError> {
        if answer != "123" {
            bail!("verification failed");
        }
        Ok(())
    }

    fn generate_random_text() -> String {
        rand::thread_rng().sample_iter(&Alphanumeric).take(30).map(char::from).collect()
    }
}

impl ToString for Challenge {
    fn to_string(&self) -> String {
        format!("{}:{}:{}", self.num_zeros, self.text, self.counter)
    }
}

