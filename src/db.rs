use create::error_handler::CustomError;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2;
use std::env;
use diesel_migrations::embed_migrations;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!();

lazy_static!{
    static ref POOL:Pool = {
        let db_url = env::var("DATABASE_URL").expect("数据库url没设置");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Pool data connect failed");
    };
}

