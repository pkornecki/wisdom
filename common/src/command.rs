use std::fmt::{Display, Formatter, Result};

/// an enum with command names
#[derive(Debug)]
pub enum Command {
    GET, // get
    QUO, // quote
    SLV, // solve
    THX, // thanks
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
