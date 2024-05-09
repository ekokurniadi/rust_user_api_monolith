use diesel::r2d2::{self,ConnectionManager};
use diesel::PgConnection;
use std::env;


pub(crate) type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool()->PgPool{
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create")
}