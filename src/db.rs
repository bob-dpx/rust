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

pub fn connection() -> Result<DbConnection,CustomError>{
    POOL.get().map_err(|e| CustomError::new(500,format!("获取数据路连接失败:{}",e)))
}

pub fn init() {
    lazy_static::initialize(&POOL);
    let conn = connection().expect("获取数据路连接失败");
    embed_migrations::run(&conn).unwrap();
}