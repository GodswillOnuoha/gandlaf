/* Test database setup module */

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use once_cell::sync::Lazy;
use std::fs::read_to_string;
use std::sync::Arc;
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt,
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
};
use tokio::sync::Mutex;
use tokio_postgres::NoTls;

use gandalf::config::database::PgPool;

// Store both container and pool for persistence
pub struct TestDatabase {
    _container: ContainerAsync<GenericImage>, // Keep container alive by storing it
    pub pool: Arc<PgPool>,
}

// Use tokio::sync::Mutex instead of std::sync::Mutex for async-aware locking
static DB_INSTANCE: Lazy<Arc<Mutex<Option<TestDatabase>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

// Get or initialize the shared test database
pub async fn get_test_db() -> Arc<PgPool> {
    let mut guard = DB_INSTANCE.lock().await;

    // Return cloned pool if already initialized
    if let Some(db) = guard.as_ref() {
        return db.pool.clone();
    }

    // Otherwise initialize the database
    let db = init_database().await;
    let pool = db.pool.clone();
    *guard = Some(db);

    pool
}

// Initialize the database container and set up schemas
async fn init_database() -> TestDatabase {
    println!("Initializing test database...");

    let container = GenericImage::new("postgres", "17.4")
        .with_exposed_port(5432.tcp())
        .with_wait_for(WaitFor::message_on_stdout(
            "database system is ready to accept connections",
        ))
        .with_env_var("POSTGRES_DB", "test_db")
        .with_env_var("POSTGRES_USER", "postgres")
        .with_env_var("POSTGRES_PASSWORD", "postgres")
        .start()
        .await
        .expect("Failed to start Postgres test container");

    // Get connection details
    let host = &container.get_host().await.unwrap();
    let port = container
        .get_host_port_ipv4(5432.tcp())
        .await
        .expect("Failed to get port");

    // Create connection pool
    let connection_string = format!("postgres://postgres:postgres@{}:{}/test_db", host, port);

    let manager =
        PostgresConnectionManager::new_from_stringlike(&connection_string, NoTls).unwrap();
    let pool = Pool::builder().max_size(5).build(manager).await.unwrap();

    let conn = pool.get().await.expect("Failed to get DB connection");

    let init_script =
        read_to_string("local/sql/dev_initial/0000_initial.sql").expect("Failed to read init SQL");

    conn.batch_execute(&init_script)
        .await
        .expect("Failed to run init script");

    TestDatabase {
        _container: container, // Store container to keep it alive
        pool: Arc::new(pool.clone()),
    }
}

// Reset the database between tests
pub async fn reset_database(
    pool: Arc<PgPool>,
    tables: impl IntoIterator<Item = &'static str>,
) -> Result<(), tokio_postgres::Error> {
    let mut client = pool.get().await.unwrap();

    // Start transaction
    let transaction = client.transaction().await.unwrap();

    for table in tables {
        let query = format!("TRUNCATE TABLE {} RESTART IDENTITY CASCADE", table);
        transaction.execute(query.as_str(), &[]).await?;
    }

    transaction.commit().await?;
    Ok(())
}

// setup test db
pub async fn get_test_db_pool() -> Arc<PgPool> {
    let pool = get_test_db().await;

    // tables to reset per test
    let tables = ["auth.users"];
    reset_database(pool.clone(), tables).await.unwrap();
    pool
}
