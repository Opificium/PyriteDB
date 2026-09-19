mod database;

use database::db::Db;
use std::sync::Arc;
use crate::database::server;

fn main() -> std::io::Result<()>{
    let db = Arc::new(Db::new());
    server::run("127.0.0.1:7878", db)
}
