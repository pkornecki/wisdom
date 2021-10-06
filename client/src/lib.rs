use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWriteExt};
use std::error::Error;

use common::challenge::Challenge;
use common::command::Command;

/// gets a quote using the stream provided
///
/// # arguments
///
/// * `stream` - read\write stream of data
pub async fn get_quote<T>(stream: &mut T) -> Result<String, Box<dyn Error>>
where
    T: AsyncBufRead + AsyncWriteExt + std::marker::Unpin,
{
        // send a "GET" request
        stream.write_all(format!("{}\n", Command::GET).as_bytes()).await?;

        // parse the response
        let challenge = parse(stream).await?;

        // solve the challenge
        let answer = solve(&challenge)?;

        // send the answer
        stream.write_all(answer.as_bytes()).await?;

        // parse the response
        let result = parse(stream).await?;

        // send the "THX" confirmation
        stream.write_all(format!("{}\n", Command::THX).as_bytes()).await?;

        Ok(result)
}

// parse the response from the server
async fn parse<T>(stream: &mut T) -> Result<String, Box<dyn Error>>
where
    T: AsyncBufRead + std::marker::Unpin,
{
    // read the line from a stream
    let mut line = String::new();
    stream.read_line(&mut line).await?;

    // decode the line into a command and contents
    let command = &line[0..3];
    let content = &line[4..];

    // parse the command
    let result = if command == Command::SLV.to_string() {
        content
    } else if command == Command::QUO.to_string() {
        // remove the new line
        &content[..content.len() - 1]
    } else {
        "unknown command"
    };

    Ok(result.to_string())
}

// solve the challenge
fn solve(challenge: &str) -> Result<String, Box<dyn Error>> {
    let answer = Challenge::solve(challenge)?;
    let answer = format!("{}\n", answer.to_string());
    Ok(answer)
}
