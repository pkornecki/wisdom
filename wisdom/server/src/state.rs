use super::db::Db;
use super::Response;
use common::challenge::Challenge;
use common::command::Command;

use rand::Rng;
use simple_error::{bail, SimpleError};

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

    fn process(&mut self, line: &str, _db: &Db, difficulty: u8) -> Result<Option<Response>, SimpleError> {
        println!("connected, got: {:?}", line);
        if line == Command::GET.to_string() {
            self.challenge = Some(Challenge::new(difficulty));
            return Ok(Some(format!(
                "{} {}",
                Command::SLV,
                self.challenge.as_ref().unwrap().to_string()
            )));
        }

        bail!("invalid command");
    }
}

impl ConnectionState<ChallengeSent> {
    fn process(&mut self, line: &str, db: &Db, _difficulty: u8) -> Result<Option<Response>, SimpleError> {
        println!("challenge, got: {:?}", line);
        if let Ok(()) = self
            .challenge
            .as_ref()
            .expect("challenge missing")
            .verify(line)
        {
            let quote = Self::get_quote(&db)?;
            return Ok(Some(format!("{} {}", Command::QUO, quote)));
        }

        bail!("challenge verificaton failed");
    }

    fn get_quote(db: &Db) -> Result<Response, SimpleError> {
        // ignore first row as it is a header
        let n = rand::thread_rng().gen_range(1..db.num_quotes());

        if let Some(quote) = db.get_quote(n) {
            return Ok(quote.to_string());
        }

        bail!("can't get quote from the db")
    }
}

impl ConnectionState<Done> {
    fn process(&mut self, line: &str, _db: &Db, _difficulty: u8) -> Result<Option<Response>, SimpleError> {
        println!("done, got: {:?}", line);
        Ok(None)
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

impl From<ConnectionState<Done>> for ConnectionState<Connected> {
    fn from(_val: ConnectionState<Done>) -> ConnectionState<Connected> {
        ConnectionState {
            challenge: None,
            state: Connected {},
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
            StateWrapper::Done(val) => StateWrapper::Connected(val.into()),
        }
    }
    pub fn process(&mut self, line: &str, db: &Db, difficulty: u8) -> Result<Option<Response>, SimpleError> {
        match self {
            StateWrapper::Connected(val) => val.process(line, db, difficulty),
            StateWrapper::ChallengeSent(val) => val.process(line, db, difficulty),
            StateWrapper::Done(val) => val.process(line, db, difficulty),
        }
    }
}
