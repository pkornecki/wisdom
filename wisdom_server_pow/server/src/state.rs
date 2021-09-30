use simple_error::{bail, SimpleError};
use super::Response;
use super::challenge;

pub struct ConnectionState<T> {
    state: T,
}

pub struct Connected {}
pub struct Challenge {}
pub struct Done {}

impl ConnectionState<Connected> {
    fn new() -> Self {
        ConnectionState {
            state: Connected {},
        }
    }
    fn process(&self, line: &str) -> Result<Response, SimpleError> {
        println!("connected, got: {:?}", line);
        if line == "GET" {
            return Ok(challenge::create());
        }
        bail!("invalid command");
    }
}
impl ConnectionState<Challenge> {
    fn process(&self, line: &str) -> Result<Response, SimpleError> {
        println!("challenge, got: {:?}", line);
        if let Ok(()) = challenge::verify(line) {
            return Ok("Correct".to_string());
        }
        bail!("challenge verificaton failed");
    }
}

impl From<ConnectionState<Connected>> for ConnectionState<Challenge> {
    fn from(val: ConnectionState<Connected>) -> ConnectionState<Challenge> {
        ConnectionState {
            state: Challenge {}
        }
    }
}

impl From<ConnectionState<Challenge>> for ConnectionState<Done> {
    fn from(val: ConnectionState<Challenge>) -> ConnectionState<Done> {
        ConnectionState {
            state: Done {}
        }
    }
}

pub enum StateWrapper {
    Connected(ConnectionState<Connected>),
    Challenge(ConnectionState<Challenge>),
    Done(ConnectionState<Done>),
}

pub struct State {
    pub state: StateWrapper,
}

impl State {
    pub fn new() -> Self {
        State {
            state: StateWrapper::Connected(ConnectionState::new()),
        }
    }
}

impl StateWrapper {
    pub fn next(mut self) -> Self {
        match self {
            StateWrapper::Connected(val) => StateWrapper::Challenge(val.into()),
            StateWrapper::Challenge(val) => StateWrapper::Done(val.into()),
            StateWrapper::Done(val) => StateWrapper::Done(val),
        }
    }
    pub fn process(&self, line: &str) -> Result<Response, SimpleError> {
        match self {
            StateWrapper::Connected(val) => val.process(line),
            StateWrapper::Challenge(val) => val.process(line),
            StateWrapper::Done(_) => Ok("BYE".to_string()),
        }
    }
    pub fn done(&self) -> bool {
        if let StateWrapper::Done(_) = self {
            return true;
        }
        false
    }
}

