use tokio::io::BufReader;
use tokio::net::TcpStream;
use std::error::Error;
use std::io;
use structopt::StructOpt;
use text_io::read;

mod cli;

use cli::CommandLineArgs;

/// entry point of the application
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
        // in "interactive mode" run in a loop until user decides to quit

        if interactive {
            print!("quote? (yes/no): ");
            // no new line was printed therefore must flush manually
            io::Write::flush(&mut io::stdout())?;

            // get user's input
            let choice: String = read!("{}\n");
            let choice = choice.trim().to_lowercase();
            if choice != "y" && choice != "yes" {
                break;
            }
        }

        // get the quote
        let quote = client::get_quote(&mut stream).await?;

        // print it to stdout
        println!("{}", quote);

        // in non-interactive mode quit immediatelly,
        // otherwise, repeat the loop
        if !interactive {
            break;
        }
    }

    Ok(())
}
