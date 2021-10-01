use super::challenge;
use super::db::Db;
use super::Response;

use simple_error::{bail, SimpleError};

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
    fn process(&self, line: &str, _db: &Db) -> Result<Response, SimpleError> {
        println!("connected, got: {:?}", line);
        if line == "GET" {
            return Ok(challenge::create());
        }
        bail!("invalid command");
    }
}

impl ConnectionState<Challenge> {
    fn process(&self, line: &str, db: &Db) -> Result<Response, SimpleError> {
        println!("challenge, got: {:?}", line);
        if let Ok(()) = challenge::verify(line) {
            return Self::get_quote(&db);
        }
        bail!("challenge verificaton failed");
    }

    fn get_quote(db: &Db) -> Result<Response, SimpleError> {
        let n = 0;
        if let Some(quote) = db.get_quote(n) {
            return Ok(quote.to_string());
        }
        bail!("can't get quote from the db")
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
    pub fn next(self) -> Self {
        match self {
            StateWrapper::Connected(val) => StateWrapper::Challenge(val.into()),
            StateWrapper::Challenge(val) => StateWrapper::Done(val.into()),
            StateWrapper::Done(val) => StateWrapper::Done(val),
        }
    }
    pub fn process(&self, line: &str, db: &Db) -> Result<Response, SimpleError> {
        match self {
            StateWrapper::Connected(val) => val.process(line, db),
            StateWrapper::Challenge(val) => val.process(line, db),
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

