use super::challenge::Challenge;
use super::db::Db;
use super::Response;

use simple_error::{bail, SimpleError};
use rand::Rng;

pub struct ConnectionState<T> {
    challenge: Option<Challenge>,
    state: T,
}

pub struct Connected {}
pub struct ChallengeSent {}
pub struct Done {}

impl ConnectionState<Connected> {
    fn new() -> Self {
        ConnectionState {
            challenge: None,
            state: Connected {},
        }
    }
    fn process(&mut self, line: &str, _db: &Db) -> Result<Response, SimpleError> {
        println!("connected, got: {:?}", line);
        if line == "GET" {
            self.challenge = Some(Challenge::new(5));
            return Ok(format!("SLV {}", self.challenge.as_ref().unwrap().to_string()));
        }
        bail!("invalid command");
    }
}

impl ConnectionState<ChallengeSent> {
    fn process(&mut self, line: &str, db: &Db) -> Result<Response, SimpleError> {
        println!("challenge, got: {:?}", line);
        if let Ok(()) = self.challenge.as_ref().expect("challenge missing").verify(line) {
            return Self::get_quote(&db);
        }
        bail!("challenge verificaton failed");
    }

    fn get_quote(db: &Db) -> Result<Response, SimpleError> {
        let n = rand::thread_rng().gen_range(0..db.num_quotes());
        if let Some(quote) = db.get_quote(n) {
            return Ok(quote.to_string());
        }
        bail!("can't get quote from the db")
    }
}

impl From<ConnectionState<Connected>> for ConnectionState<ChallengeSent> {
    fn from(val: ConnectionState<Connected>) -> ConnectionState<ChallengeSent> {
        ConnectionState {
            challenge: val.challenge,
            state: ChallengeSent {},
        }
    }
}

impl From<ConnectionState<ChallengeSent>> for ConnectionState<Done> {
    fn from(val: ConnectionState<ChallengeSent>) -> ConnectionState<Done> {
        ConnectionState {
            challenge: val.challenge,
            state: Done {},
        }
    }
}

pub enum StateWrapper {
    Connected(ConnectionState<Connected>),
    ChallengeSent(ConnectionState<ChallengeSent>),
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
            StateWrapper::Connected(val) => StateWrapper::ChallengeSent(val.into()),
            StateWrapper::ChallengeSent(val) => StateWrapper::Done(val.into()),
            StateWrapper::Done(val) => StateWrapper::Done(val),
        }
    }
    pub fn process(&mut self, line: &str, db: &Db) -> Result<Response, SimpleError> {
        match self {
            StateWrapper::Connected(val) => val.process(line, db),
            StateWrapper::ChallengeSent(val) => val.process(line, db),
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

