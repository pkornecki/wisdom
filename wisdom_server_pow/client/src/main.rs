use tokio::io::BufReader;
use tokio::net::TcpStream;
use std::error::Error;
use std::io;
use text_io::read;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // establish a connection with the server
    let stream = TcpStream::connect("127.0.0.1:3962").await?;
    // create an object for reading and writing from/to the stream
    let mut stream = BufReader::new(stream);

    loop {
        print!("quote? (yes/no): ");
        io::Write::flush(&mut io::stdout())?;

        let choice: String = read!("{}\n");
        let choice = choice.trim().to_lowercase();
        if choice != "y" && choice != "yes" {
            break;
        }

        let quote = client::get_quote(&mut stream).await?;
        println!("{}", quote);
    }

    Ok(())
}
