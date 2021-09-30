use std::error::Error;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

struct Action<T> {
    state: T,
}

struct Connected {}

struct Challenge {}

impl Action<Connected> {
    fn new() -> Self {
        Action {
            state: Connected {},
        }
    }
    fn process(&self, line: &str) {
        println!("connected, line: {:?}", line)
    }
}
impl Action<Challenge> {
    fn process(line: &str) {
        println!("challenge, line: {:?}", line)
    }
}

async fn process<F>(lines: &mut Framed<TcpStream, LinesCodec>, action: F)
where
    F: FnOnce(&str),
{
    if let Some(result) = lines.next().await {
        match result {
            Ok(line) => action(&line),
            Err(err) => eprintln!("error reading from stream: {:?}", err),
        }
    }
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

            process(&mut data, |line| Action::<Connected>::new().process(line)).await;
            process(&mut data, |line| Action::<Challenge>::process(line)).await;

            println!("connection closed");
        });
    }
}
