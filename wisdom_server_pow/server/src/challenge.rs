use simple_error::{bail, SimpleError};

pub fn create() -> String {
    "GIV me the number".to_string()
}

pub fn verify(answer: &str) -> Result<(), SimpleError> {
    if answer != "123" {
        bail!("verification failed");
    }
    Ok(())
}
