use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;
use simple_error::bail;
use rand::{distributions::Alphanumeric, Rng};
use sha1::Sha1;

pub struct Challenge {
    num_zeros: u8,
    text: String,
    counter: u128,
}

impl Challenge {
    pub fn new(num_zeros: u8) -> Self {
        Challenge {
            num_zeros,
            text: Self::generate_random_text(),
            counter: 0,
        }
    }

    pub fn verify(&self, answer: &str) -> Result<(), Box<dyn Error>> {
        // create Challenge object from answer provided as string
        let answer = Challenge::from_str(answer)?;

        // compare the structure(ignore the counter)
        if self.num_zeros != answer.num_zeros || self.text != answer.text {
            bail!("answer does not match the challenge");
        }

        // calculate the sha1 of the answer
        let mut sha1 = Sha1::new();
        sha1.update(answer.to_string().as_bytes());
        let digest = sha1.digest().to_string();

        // check if the digest satisfies the num_zeros requirement
        let pattern = (0..answer.num_zeros).map(|_| "0").collect::<String>();
        if !digest.starts_with(&pattern) {
            bail!("verification failed");
        }

        Ok(())
    }

    pub fn solve(challenge: &str) -> Result<Self, Box<dyn Error>> {
        let mut answer = Self::from_str(&challenge)?;
        let pattern = (0..answer.num_zeros).map(|_| "0").collect::<String>();
        let mut sha1 = Sha1::new();

        loop {
            sha1.reset();
            sha1.update(answer.to_string().as_bytes());
            let digest = sha1.digest().to_string();

            if digest.starts_with(&pattern) {
                break;
            }

            answer.counter += 1;
        }

        Ok(answer)
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

impl FromStr for Challenge {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split(':').collect();
        if parts.len() != 3 {
            // currently, there is no way of constructing ParseIntError directly
            return Err("".parse::<u8>().expect_err("parse error"));
        }

        let num_zeros = parts[0].parse::<u8>()?;
        let text = parts[1].to_string();
        let counter = parts[2].parse::<u128>()?;

        Ok(Challenge { num_zeros, text, counter })
    }
}
