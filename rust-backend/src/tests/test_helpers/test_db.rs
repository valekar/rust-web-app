#[allow(dead_code)]
//use sqlx::prelude::Connect;
use sqlx::{postgres::PgConnectOptions, Connection, PgConnection};
use sqlx::{postgres::Postgres, PgPool, Pool};
use std::env::var;

#[derive(Debug)]
pub struct TestDb {
    pg_options: PgConnectOptions,
    db_pool: Option<PgPool>,
    db_name: String,
    test_db_name: String,
}

impl TestDb {
    pub async fn new() -> Self {
        let db_url = &var("DATABASE_URL_TEST").unwrap();
        let db_name = generate_db_name("test");
        let test_db_name = String::from(&var("TEST_DATABASE").unwrap());
        println!("Generated DB Name :: {} ", db_name);
        create_db(db_url, &db_name).await;

        let pg_options = get_pg_options(&db_name);
        let pg_options_clone = pg_options.clone();

        run_migrations(&pg_options).await;

        let db_pool: PgPool = Pool::<Postgres>::connect_with(pg_options).await.unwrap();

        Self {
            pg_options: pg_options_clone,
            db_pool: Some(db_pool),
            db_name: db_name,
            test_db_name: test_db_name,
        }
    }

    pub fn db(&self) -> PgPool {
        self.db_pool.clone().unwrap()
    }
}

impl Drop for TestDb {
    fn drop(&mut self) {
        let _ = self.db_pool.take();
    }
}

async fn create_db(db_url: &str, db_name: &str) {
    println!("{}", db_url);
    let mut conn = PgConnection::connect(db_url).await.unwrap();

    let sql = format!(r#"CREATE DATABASE "{}""#, db_name);
    sqlx::query::<Postgres>(&sql)
        .execute(&mut conn)
        .await
        .unwrap();
}

async fn run_migrations(pg_options: &PgConnectOptions) {
    let mut conn = PgConnection::connect_with(pg_options).await.unwrap();
    let sql = async_std::fs::read_to_string("../bin/backend/setup.sql")
        .await
        .unwrap();
    sqlx::query::<Postgres>(&sql)
        .execute(&mut conn)
        .await
        .unwrap();
}

fn get_pg_options(db_name: &str) -> PgConnectOptions {
    let host = var("TEST_HOST").unwrap();
    let port: u16 = var("TEST_PORT").unwrap().parse::<u16>().unwrap();
    let password = var("TEST_PASSWORD").unwrap();
    let username = var("TEST_USER").unwrap();
    let database = db_name;

    let pool_options = PgConnectOptions::new()
        .host(&host)
        .port(port)
        .username(&username)
        .database(&database)
        .password(&password);

    pool_options
}

fn generate_db_name(db_url: &str) -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let rng = thread_rng();
    let suffix: String = rng.sample_iter(&Alphanumeric).take(16).collect();
    format!("{}_{}", db_url, suffix)
}
