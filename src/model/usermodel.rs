use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    // 用户名
    pub username: String,
    // 邮箱
    pub email: String,
    // 密码
    pub password: String,
    // 用户组
    pub roles: String,
    // 昵称
    pub avatar: String,
}

impl User {
    pub const TABLE_NAME: &'static str = "users";

    pub fn create_table_sql() -> &'static str {
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            roles TEXT NOT NULL,
            avatar TEXT,
            email TEXT
        )"#
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Default::default(),
            username: Default::default(),
            email: Default::default(),
            password: Default::default(),
            roles: "user".to_string(),
            avatar: Default::default(),
        }
    }
}
