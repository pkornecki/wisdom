use super::db::Db;
use super::Response;
use common::challenge::Challenge;
use common::command::Command;

use rand::Rng;
use simple_error::{bail, SimpleError};

/// a struct that keeps track of the state the connection is in
pub struct ConnectionState<T> {
    challenge: Option<Challenge>,
    _state: T,
}

/// a client established a connection
pub struct Connected {}

/// the challenge was sent to the client
pub struct ChallengeSent {}

/// a quote was sent to the client
pub struct Done {}

impl ConnectionState<Connected> {
    fn new() -> Self {
        ConnectionState {
            challenge: None,
            _state: Connected {},
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

// implement the transitions for each state
// note: for transitions not listed here are lead to a compilation error
// for example: Connected -> Done will trigger a compiler error

// the transition from the Connected to the ChallengeSent state
impl From<ConnectionState<Connected>> for ConnectionState<ChallengeSent> {
    fn from(val: ConnectionState<Connected>) -> ConnectionState<ChallengeSent> {
        ConnectionState {
            challenge: val.challenge,
            _state: ChallengeSent {},
        }
    }
}

// the transition from the ChallengeSent to the Done state
impl From<ConnectionState<ChallengeSent>> for ConnectionState<Done> {
    fn from(val: ConnectionState<ChallengeSent>) -> ConnectionState<Done> {
        ConnectionState {
            challenge: val.challenge,
            _state: Done {},
        }
    }
}

// the transition from the Done to the Connected state
impl From<ConnectionState<Done>> for ConnectionState<Connected> {
    fn from(_val: ConnectionState<Done>) -> ConnectionState<Connected> {
        ConnectionState {
            challenge: None,
            _state: Connected {},
        }
    }
}

/// an enum specifying states
pub enum StateWrapper {
    Connected(ConnectionState<Connected>),
    ChallengeSent(ConnectionState<ChallengeSent>),
    Done(ConnectionState<Done>),
}

/// a struct that holds a StateWrapper instance
pub struct State {
    /// wrapper of the state
    pub state: StateWrapper,
}

impl State {
    /// creates a new State instance
    pub fn new() -> Self {
        State {
            state: StateWrapper::Connected(ConnectionState::new()),
        }
    }
}

impl StateWrapper {
    /// proceeds to the next state
    pub fn next(self) -> Self {
        match self {
            StateWrapper::Connected(val) => StateWrapper::ChallengeSent(val.into()),
            StateWrapper::ChallengeSent(val) => StateWrapper::Done(val.into()),
            StateWrapper::Done(val) => StateWrapper::Connected(val.into()),
        }
    }

    /// calls the `process` function for a relevant state
    pub fn process(&mut self, line: &str, db: &Db, difficulty: u8) -> Result<Option<Response>, SimpleError> {
        match self {
            StateWrapper::Connected(val) => val.process(line, db, difficulty),
            StateWrapper::ChallengeSent(val) => val.process(line, db, difficulty),
            StateWrapper::Done(val) => val.process(line, db, difficulty),
        }
    }
}
