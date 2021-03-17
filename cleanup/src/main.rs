use sqlx::{
    postgres::{PgConnectOptions, PgRow},
    query, Connection, PgConnection, Postgres, Row,
};
use std::env::var;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    let (test_db, pg_options) = get_pg_options();
    drop_db(&test_db, pg_options).await;
}

fn get_pg_options() -> (String, PgConnectOptions) {
    dotenv::dotenv().ok();

    let host = var("TEST_HOST").unwrap();
    let port: u16 = var("TEST_PORT").unwrap().parse::<u16>().unwrap();
    let password = var("TEST_PASSWORD").unwrap();
    let username = var("TEST_USER").unwrap();
    let database = var("TEST_DATABASE").unwrap();

    let pool_options = PgConnectOptions::new()
        .host(&host)
        .port(port)
        .username(&username)
        .database(&database)
        .password(&password);

    (database, pool_options)
}

pub async fn drop_db(test_db: &str, pg_options: PgConnectOptions) {
    println!("Cleaning up databases");

    let mut conn: PgConnection = PgConnection::connect_with(&pg_options).await.unwrap();

    let sql = format!(
        r#"SELECT pg_terminate_backend(pid)
        FROM pg_stat_activity
        WHERE pg_stat_activity.datname = '{db}'
        AND pid <> pg_backend_pid();"#,
        db = test_db
    );

    sqlx::query::<Postgres>(&sql)
        .execute(&mut conn)
        .await
        .unwrap();

    let collect_unused_dbs: String = format!(
        r#"select 'drop database "'||datname||'";'
        as column_name from pg_database
        where datistemplate=false AND datname<>'{test_db}' "#,
        test_db = test_db
    );

    let res: Vec<String> = query::<Postgres>(&collect_unused_dbs)
        .map(|row: PgRow| row.get("column_name"))
        .fetch_all(&mut conn)
        .await
        .unwrap();

    for row in res {
        sqlx::query::<Postgres>(&row)
            .execute(&mut conn)
            .await
            .unwrap();
    }
}
