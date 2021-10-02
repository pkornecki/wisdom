use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use structopt::StructOpt;

mod cli;

use server::db::Db;
use cli::CommandLineArgs;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // get the command line arguments
    let CommandLineArgs { port, difficulty, words } = CommandLineArgs::from_args();
    let port = port.unwrap_or_else(|| 3962);
    let difficulty = difficulty.unwrap_or_else(|| 4);
    let words = words.unwrap_or_else(|| PathBuf::from("words_of_wisdom.txt"));

    // create a listener
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

    // create a shared database instance
    // once crated, there is no need to guard it behind a mutex,
    // as all tasks need read-only access
    let db = Arc::new(Db::new(words));

    server::run(listener, db, difficulty).await
}
