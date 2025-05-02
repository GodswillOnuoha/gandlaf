/*
This module contains config settings for the database connection.
*/
use std::env;

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio::sync::OnceCell;
use tokio_postgres::NoTls;

use super::defaults;

pub type PgPool = Pool<PostgresConnectionManager<NoTls>>;

#[derive(Clone, Debug)]
pub struct DBConfig {
    pub database_url: String,
    pub max_db_connections: u32,
}

impl DBConfig {
    pub fn from_env() -> Self {
        let database_url = env::var("DB_URL").expect("DB_URL must be set");
        let max_db_connections: u32 = env::var("MAX_DB_CONNECTIONS")
            .unwrap_or_else(|_| defaults::MAX_DB_CONNECTIONS.to_string())
            .parse()
            .expect("MAX_DB_CONNECTIONS must be a number");

        DBConfig {
            database_url,
            max_db_connections,
        }
    }
}

// Global database instance (Singleton)
static DB_INSTANCE: tokio::sync::OnceCell<PgPool> = OnceCell::const_new();

pub async fn get_db_connection_pool() -> &'static PgPool {
    DB_INSTANCE
        .get_or_init(|| async {
            let config = DBConfig::from_env();

            let manager =
                PostgresConnectionManager::new_from_stringlike(&config.database_url, NoTls)
                    .unwrap();
            Pool::builder()
                .max_size(config.max_db_connections)
                .build(manager)
                .await
                .unwrap()
        })
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    fn set_env(key: &str, value: &str) {
        unsafe {
            env::set_var(key, value);
        }
    }

    #[test]
    #[serial]
    fn test_db_config_from_env() {
        // Set environment variables
        set_env("DB_URL", "postgres://testuser:testpass@localhost/testdb");
        set_env("MAX_DB_CONNECTIONS", "10");

        let config = DBConfig::from_env();

        assert_eq!(
            config.database_url,
            "postgres://testuser:testpass@localhost/testdb"
        );
        assert_eq!(config.max_db_connections, 10);
    }
}
