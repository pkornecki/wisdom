use structopt::StructOpt;

/// a struct encapsulating command line arguments
#[derive(Debug, StructOpt)]
#[structopt(
    name = "wisdom client",
    about = "a client for retreiving words of wisdom from the wisdom server
it implements a proof of work algorithm by solving a challenge sent by the server"
)]
pub struct CommandLineArgs {
    /// ip address of the server
    #[structopt(short, long)]
    pub addr: Option<String>,

    /// port to connect to
    #[structopt(short, long)]
    pub port: Option<u16>,

    /// interactive mode
    #[structopt(short, long)]
    pub interactive: bool,
}

