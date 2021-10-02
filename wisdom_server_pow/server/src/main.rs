use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;

use server::db::Db;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // create a listener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3962").await?;

    // create a shared database instance
    // once crated, there is no need to guard it behind a mutex,
    // as all tasks need read-only access
    let db = Arc::new(Db::new(PathBuf::from("words_of_wisdom.txt")));

    server::run(listener, db).await
}
