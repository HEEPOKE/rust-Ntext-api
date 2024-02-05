use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};

pub fn db_connection(database_url: &str) -> Pool<ConnectionManager<PgConnection>>{
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    return pool;
}
