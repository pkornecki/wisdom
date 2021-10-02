use futures::SinkExt;
use simple_error::{bail, SimpleError};
use std::error::Error;
use std::io;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

pub mod db;
mod state;

use crate::db::Db;
use crate::state::State;

// a Response is just a simple string
// it could be a struct encapsulating some specific fields as well
type Response = String;

pub async fn run(listener: TcpListener, db: Arc<Db>) -> Result<(), Box<dyn Error>> {
   // keep the track of a number of connections established
    let mut connection_id: u128 = 0;

    loop {
        // wait for an inbound connection
        let (socket, addr) = listener.accept().await?;
        connection_id += 1;

        let db = db.clone();

        // handle each connection in a separate, asynchronous task
        tokio::spawn(async move {
            println!("[{}] connection established for {}", connection_id, addr);

            // decode the data
            let mut data = Framed::new(socket, LinesCodec::new());

            // repeat until client disconnects
            'outer: loop {
                // initiate a new state
                let mut current = State::new();

                // iterate through the states
                'inner: loop {
                    match process(&mut data, |line| current.state.process(line, &db), connection_id,) .await {
                        Ok(response) => {
                            if let Some(response) = response {
                                if let Err(err) = data.send(response.as_str()).await {
                                    eprintln!("[{}] error: {}", connection_id, err);
                                    break 'inner;
                                }
                            }
                            current.state = current.state.next();
                        }
                        Err(err) => {
                            eprintln!("[{}] error: {}", connection_id, err);
                            break 'outer;
                        }
                    }
                }
            }

            println!("[{}] connection closed", connection_id);
        });
    }
}

async fn process<F>(
    lines: &mut Framed<TcpStream, LinesCodec>,
    action: F,
    connection_id: u128,
) -> Result<Option<Response>, SimpleError>
where
    F: FnOnce(&str) -> Result<Option<Response>, SimpleError>,
{
    if let Some(result) = lines.next().await {
        print!("[{}] ", connection_id);
        // no new line printed therefore must flush manually
        io::Write::flush(&mut io::stdout()).expect("flush failed");

        match result {
            Ok(line) => return action(&line),
            Err(err) => bail!("error reading from stream: {:?}", err),
        }
    }
    bail!("error reading data")
}

