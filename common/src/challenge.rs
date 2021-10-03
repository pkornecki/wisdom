use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;
use simple_error::bail;
use rand::{distributions::Alphanumeric, Rng};
use sha2::{Sha256, Digest};

/// a struct that specifies the challenge
pub struct Challenge {
    // a minimum number of zero bytes
    num_zeros: u8,
    // some text
    text: String,
    // a counter to increment
    counter: u128,
}

impl Challenge {
    /// creates a new challenge
    pub fn new(num_zeros: u8) -> Self {
        Challenge {
            num_zeros,
            text: Self::generate_random_text(),
            counter: 0,
        }
    }

    /// verifies if the answer is correct
    ///
    /// # arguments
    ///
    /// * `answer` - represented as a string
    pub fn verify(&self, answer: &str) -> Result<(), Box<dyn Error>> {
        // create Challenge object from answer provided as string
        let answer = Challenge::from_str(answer)?;

        // compare the structure(ignore the counter)
        if self.num_zeros != answer.num_zeros || self.text != answer.text {
            bail!("answer does not match the challenge");
        }

        // calculate the digest of the answer
        let digest = Sha256::digest(answer.to_string().as_bytes());

        // check if the digest satisfies the num_zeros requirement
        let pattern = (0..answer.num_zeros).map(|_| "0").collect::<String>();
        if !digest.starts_with(&pattern.as_bytes()) {
            bail!("verification failed");
        }

        Ok(())
    }

    /// solves the challenge
    ///
    /// # arguments
    ///
    /// * `challenge` - represented as a string
    pub fn solve(challenge: &str) -> Result<Self, Box<dyn Error>> {
        // parse the string
        let mut answer = Self::from_str(&challenge)?;

        // repeat the required number of zeros
        let pattern = (0..answer.num_zeros).map(|_| "0").collect::<String>();

        // create hasher instance
        let mut hasher = Sha256::new();

        loop {
            // add the current answer to the hasher
            hasher.update(answer.to_string().as_bytes());

            // compute the digest
            let digest = hasher.finalize_reset();

            // check if the condition is met
            if digest.starts_with(&pattern.as_bytes()) {
                break;
            }

            // increment the counter
            answer.counter += 1;
        }

        Ok(answer)
    }

    fn generate_random_text() -> String {
        rand::thread_rng().sample_iter(&Alphanumeric).take(30).map(char::from).collect()
    }
}

// create a string from the Challenge
impl ToString for Challenge {
    fn to_string(&self) -> String {
        format!("{}:{}:{}", self.num_zeros, self.text, self.counter)
    }
}

// create a Challenge from the string
impl FromStr for Challenge {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // split the string into parts
        let parts: Vec<&str> = s.trim().split(':').collect();

        // check the structure
        if parts.len() != 3 {
            // currently, there is no way of constructing ParseIntError directly
            return Err("".parse::<u8>().expect_err("parse error"));
        }

        // construct the result
        let num_zeros = parts[0].parse::<u8>()?;
        let text = parts[1].to_string();
        let counter = parts[2].parse::<u128>()?;

        Ok(Challenge { num_zeros, text, counter })
    }
}
