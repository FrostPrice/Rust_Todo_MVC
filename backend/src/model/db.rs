use sqlx::{Pool, Postgres};

pub type Db = Pool<Postgres>;

async fn new_db_pool(host: &str, user:&str, pwd: &str, max_con: u32) -> Result<Db, sqlx::Error> 

{}