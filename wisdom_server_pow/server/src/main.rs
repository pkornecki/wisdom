use tokio::io::{Error};
use tokio_util::codec::{Framed, LinesCodec};
use tokio_stream::StreamExt;

fn process(line: &str) {
    println!("line: {:?}", line)
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3962").await?;

    loop {
        // wait for an inbound socket
        let (socket, addr) = listener.accept().await?;
        
        // handle connections
        tokio::spawn(async move {
            println!("connection established for {}", addr);

            // decode the stream
            let mut lines = Framed::new(socket, LinesCodec::new());

            // iterate through the data
            while let Some(result) = lines.next().await {
                match result {
                    Ok(line) => process(&line),
                    Err(err) => eprintln!("socket closed with error: {:?}", err),
                }
            }

            println!("socket received FIN packet and closed connection");
        });
    }
}
