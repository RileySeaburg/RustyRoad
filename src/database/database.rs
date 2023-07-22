use sqlx::mysql::{MySqlConnectOptions, MySqlPool};
use sqlx::postgres::{PgConnectOptions, PgPool};
use sqlx::sqlite::SqlitePool;
use std::error::Error;
use std::fs;
use std::sync::Arc;
use toml::Value;

use super::databasetype::DatabaseType;

#[derive(Debug, Clone)]
pub struct Database {
    pub name: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_type: DatabaseType,
}

#[derive(Debug, Clone)]
pub enum DatabaseConnection {
    Pg(Arc<PgPool>),
    MySql(Arc<MySqlPool>),
    Sqlite(Arc<SqlitePool>),
}

impl Database {
    pub fn new(
        name: String,
        username: String,
        password: String,
        host: String,
        port: u16,
        database_type: &str,
    ) -> Database {
        Database {
            name,
            username,
            password,
            host,
            port,
            database_type: match database_type {
                "postgres" => DatabaseType::Postgres,
                "mysql" => DatabaseType::Mysql,
                "sqlite" => DatabaseType::Sqlite,
                _ => DatabaseType::Mysql,
            },
        }
    }

    pub async fn create_database_connection(
        &self,
    ) -> Result<DatabaseConnection, Box<dyn Error + Send>> {
        match &self.database_type {
            DatabaseType::Mysql => {
                let options = MySqlConnectOptions::new()
                    .username(&self.username)
                    .password(&self.password)
                    .database(&self.name)
                    .host(&self.host)
                    .port(self.port);
                let pool = MySqlPool::connect_with(options)
                    .await
                    .unwrap_or_else(|_| panic!("Failed to create connection pool."));
                Ok(DatabaseConnection::MySql(Arc::new(pool)))
            }
            DatabaseType::Sqlite => {
                let pool = SqlitePool::connect(&format!("{}.db", self.name))
                    .await
                    .unwrap_or_else(|_| panic!("Could not connect to the SQLite database."));
                Ok(DatabaseConnection::Sqlite(Arc::new(pool)))
            }
            DatabaseType::Postgres => {
                let options = PgConnectOptions::new()
                    .username(&self.username)
                    .password(&self.password)
                    .database(&self.name)
                    .host(&self.host)
                    .port(self.port);
                let pool = PgPool::connect_with(options)
                    .await
                    .unwrap_or_else(|_| panic!("Failed to connect to Postgres database."));
                Ok(DatabaseConnection::Pg(Arc::new(pool)))
            }
            DatabaseType::Mongo => todo!(),
        }
    }

    pub fn get_database_from_rustyroad_toml() -> Result<Database, std::io::Error> {
        let file = fs::read_to_string("rustyroad.toml")
            .unwrap_or_else(|_| panic!("Error: Could not find rustyroad.toml"));
        let toml: Value = toml::from_str(&file).unwrap();
        let database_table = toml["database"].as_table().unwrap();
        Ok(Database::new(
            database_table["database_name"]
                .as_str()
                .unwrap()
                .to_string(),
            database_table["database_user"]
                .as_str()
                .unwrap()
                .to_string(),
            database_table["database_password"]
                .as_str()
                .unwrap()
                .to_string(),
            database_table["database_host"]
                .as_str()
                .unwrap()
                .to_string(),
            database_table["database_port"]
                .as_str()
                .unwrap()
                .parse::<u16>()
                .unwrap(),
            database_table["database_type"].as_str().unwrap(),
        ))
    }
}
