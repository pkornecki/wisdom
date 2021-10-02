use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncBufRead, AsyncBufReadExt};
use tokio::io::BufReader;
use tokio::net::TcpStream;

use std::error::Error;

use common::challenge::Challenge;

// parse the response from the server
async fn parse<T>(stream: &mut T) -> Result<String, Box<dyn Error>>
where
    T: AsyncBufRead + std::marker::Unpin
{
    // read the line from a stream
    let mut line = String::new();
    stream.read_line(&mut line).await?;

    // decode the line into a command and contents
    let command = &line[0..3];
    let content = &line[4..];

    // parse the command
    let result = match command {
        "SLV" => content,
        "QUO" => {
            // remove the new line
            &content[..content.len()-1]
        },
        _ => "unknown command\n",
    };

    Ok(result.to_string())
}

// solve the challenge
fn solve(challenge: &str) -> Result<String, Box<dyn Error>> {
    let answer = Challenge::solve(&challenge)?;
    let answer = format!("{}\n", answer.to_string());
    Ok(answer)
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // establish a connection with the server
    let stream = TcpStream::connect("127.0.0.1:3962").await?;
    // create an object for reading and writing from/to the stream
    let mut stream = BufReader::new(stream);

    // send a request
    stream.write_all(b"GET\n").await?;

    // parse the response
    let challenge = parse(&mut stream).await?;

    // solve the challenge
    let answer = solve(&challenge)?;
    stream.write_all(answer.as_bytes()).await?;

    // parse the response
    let result = parse(&mut stream).await?;

    // present the result
    println!("{}", result);

    Ok(())
}
