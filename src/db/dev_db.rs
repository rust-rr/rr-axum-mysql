use anyhow::Result;
use dotenv::dotenv;
use sqlx::{
    mysql::{MySqlPool, MySqlPoolOptions},
    MySql, Pool,
};
use std::{env, fs, time::Duration};

type Db = Pool<MySql>;

const CREATE_DEV_DB_SQL: &str = "sql/create_dev_db.sql";

pub async fn init_dev_db() -> Result<()> {
    dotenv().ok();

    let db_url = &env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let root_db = init_db_pool(&db_url).await?;
    pexec(CREATE_DEV_DB_SQL, &root_db).await?;
    Ok(())
}

async fn init_db_pool(db_url: &str) -> Result<MySqlPool, sqlx::Error> {
    MySqlPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_millis(500))
        .connect(db_url)
        .await
}

async fn pexec(file: &str, db: &Db) -> Result<(), sqlx::Error> {
    let sql_content = fs::read_to_string(file)?;
    let sqls: Vec<&str> = sql_content.split(";").collect();

    for sql in sqls {
        sqlx::query(sql).execute(db).await?;
    }

    Ok(())
}
