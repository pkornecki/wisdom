use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "wisdom server",
    about = "a server providing words of wisdom to its clients
it requires that clients implement proof of work algorithm and solve the challenge"
)]
pub struct CommandLineArgs {
    /// port to run on
    #[structopt(short, long)]
    pub port: Option<u16>,

    /// difficulty setting
    #[structopt(short, long)]
    pub difficulty: Option<u8>,

    /// database location (txt)
    #[structopt(parse(from_os_str), short, long)]
    pub words: Option<PathBuf>
}

