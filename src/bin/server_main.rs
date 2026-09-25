use persistent_db_w_query_language::database::db::Db;
use std::sync::Arc;
use persistent_db_w_query_language::database::server;

fn main() -> std::io::Result<()>{
    let db = Arc::new(Db::new());
    server::run("127.0.0.1:7878", db)
}
