use tokio::io::BufReader;
use tokio::net::TcpStream;
use std::error::Error;
use std::io;
use structopt::StructOpt;
use text_io::read;

mod cli;

use cli::CommandLineArgs;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // get the command line arguments
    let CommandLineArgs { addr, port, interactive } = CommandLineArgs::from_args();
    let addr = addr.unwrap_or_else(|| "127.0.0.1".to_string());
    let port = port.unwrap_or_else(|| 3962);

    // establish a connection to the server
    let stream = TcpStream::connect(format!("{}:{}", addr, port)).await?;

    // create an object for reading and writing from/to the stream
    let mut stream = BufReader::new(stream);

    loop {
        if interactive {
            print!("quote? (yes/no): ");
            io::Write::flush(&mut io::stdout())?;

            let choice: String = read!("{}\n");
            let choice = choice.trim().to_lowercase();
            if choice != "y" && choice != "yes" {
                break;
            }
        }

        let quote = client::get_quote(&mut stream).await?;
        println!("{}", quote);

        if !interactive {
            break;
        }
    }

    Ok(())
}
