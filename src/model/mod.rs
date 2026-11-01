use crate::config::Config;
use std::time::Duration;

mod printmodel;
mod usermodel;

pub async fn create_pool(config: &Config) -> sqlx::SqlitePool {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(config.database.max_connections)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(config.database.timeout))
        .connect(&config.database.url)
        .await
        .unwrap();

    sqlx::query(printmodel::PrintQueue::create_table_sql())
        .execute(&pool)
        .await
        .expect("创建print_queue表失败！！！");

    sqlx::query(usermodel::User::create_table_sql())
        .execute(&pool)
        .await
        .expect("创建user表失败!!!");

    pool
}
