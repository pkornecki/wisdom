use futures::SinkExt;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use simple_error::{bail, SimpleError};
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

mod db;
mod state;

use crate::db::Db;
use crate::state::State;

// a Response is just a simple string
// it could be a struct encapsulating some specific fields as well
type Response = String;

async fn process<F>(lines: &mut Framed<TcpStream, LinesCodec>, action: F) -> Result<Response, SimpleError>
where
    F: FnOnce(&str) -> Result<Response, SimpleError>,
{
    if let Some(result) = lines.next().await {
        match result {
            Ok(line) => return action(&line),
            Err(err) => bail!("error reading from stream: {:?}", err),
        }
    }
    bail!("error reading data")
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // create a listener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3962").await?;

    // create a shared database instance
    // there is no need to guard it behind a mutex, as all tasks need only read access
    let db = Arc::new(Db::new(PathBuf::from("words_of_wisdom.txt")));

    loop {
        // wait for an inbound connection
        let (socket, addr) = listener.accept().await?;

        let db = db.clone();

        // handle the connection in a separate asynchronous task
        tokio::spawn(async move {
            println!("connection established for {}", addr);

            // decode the data
            let mut data = Framed::new(socket, LinesCodec::new());

            // initiate a new state
            let mut current = State::new();

            // iterate through the states
            while !current.state.done() { 
                match process(&mut data, |line| current.state.process(line, &db)).await {
                    Ok(response) => {
                        if let Err(err) = data.send(response.as_str()).await {
                            eprintln!("error: {}", err);
                            break;
                        }
                        current.state = current.state.next();
                    },
                    Err(err) => {
                        eprintln!("error: {}", err);
                        break;
                    }
                }
            }

            println!("connection closed");
        });
    }
}
