use std::error::Error;
use simple_error::{bail, SimpleError};
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

mod state;

use crate::state::State;

async fn process<F>(lines: &mut Framed<TcpStream, LinesCodec>, action: F) -> Result<(), SimpleError>
where
    F: FnOnce(&str) -> Result<(), SimpleError>,
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
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3962").await?;

    loop {
        // wait for an inbound socket
        let (socket, addr) = listener.accept().await?;

        // handle connections
        tokio::spawn(async move {
            println!("connection established for {}", addr);

            // decode the data
            let mut data = Framed::new(socket, LinesCodec::new());

            let mut current = State::new();

            while !current.state.done() { 
                match process(&mut data, |line| current.state.process(line)).await {
                    Ok(()) => current.state = current.state.next(),
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
