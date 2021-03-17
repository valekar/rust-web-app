#[allow(dead_code)]
use serde::{Deserialize, Serialize};
use serde_json::json;
//use serde_json::json;
//use sqlx::prelude::Row;
use sqlx::Pool;
use sqlx::{postgres::PgConnectOptions, query_as};
use sqlx::{postgres::Postgres, query, PgPool};
use std::env::var;
use uuid::Uuid;
//use tide::Body;
use tide::Response;
use tide::{http::StatusCode, Body, Request, Server};

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    //let db_url = std::env::var("DATABASE_URL")?;
    //let pool: PgPool = Pool::<Postgres>::connect(&db_url).await?;

    let pool_options = get_pg_pool();

    let db_pool: PgPool = Pool::<Postgres>::connect_with(pool_options).await.unwrap();

    let app = server(db_pool).await;

    println!("Hello, world!");

    app.listen("127.0.0.1:8080").await.unwrap();
}

#[derive(Debug, Clone)]
struct State {
    db_pool: PgPool,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error(transparent)]
    DBError(#[from] sqlx::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    VarError(#[from] std::env::VarError),
}

async fn server(db_pool: PgPool) -> Server<State> {
    let mut app: Server<State> = Server::with_state(State { db_pool });

    app.at("/").get(|req: Request<State>| async move {
        let pool = &req.state().db_pool;
        let rows = query!("select count(*) from users").fetch_one(pool).await?;

        //let count: Record = &rows;

        dbg!(&rows);

        //let json = ([1, 2, 3]);

        //let res = Response::new(StatusCode::Ok);
        //res.set_body(Body::from_json(&json))
        //Ok(res.body_json(&json)?)
        //Ok("Hello world")

        // let user = User {
        //     count: rows.count.unwrap(),
        // };

        let user = json!({
            "count": rows.count.unwrap()
        });

        let mut res: Response = Response::new(StatusCode::Ok);
        &res.set_body(Body::from_json(&user)?);

        Ok(res)
    });

    app.at("/users").get(|req: Request<State>| async move {
        let pool = &req.state().db_pool;
        let rows = query_as!(User, "Select * from users")
            .fetch_all(pool)
            .await?;

        let mut res: Response = Response::new(StatusCode::Ok);
        &res.set_body(Body::from_json(&rows)?);

        Ok(res)
    });

    app
}

fn get_pg_pool() -> PgConnectOptions {
    let host = var("HOST").unwrap();
    let port: u16 = var("PORT").unwrap().parse::<u16>().unwrap();
    let password = var("PASSWORD").unwrap();
    let username = var("USERNAME").unwrap();
    let database = var("DATABASE").unwrap();

    let pool_options = PgConnectOptions::new()
        .host(&host)
        .port(port)
        .username(&username)
        .database(&database)
        .password(&password);

    pool_options
}

// fn build_response<T>(object: &T) -> Response {
//     let mut res: Response = Response::new(StatusCode::Ok);
//     &res.set_body(Body::from_json(&object)?);
//     res
// }

#[derive(Debug, Deserialize, Serialize)]
struct User {
    id: Uuid,
    username: String,
}

#[cfg(test)]
mod tests;
