mod test_db;

use crate::State;
use crate::{server, Server};
use futures::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
pub use serde_json::{json, Value};
use std::collections::HashMap;
use std::pin::Pin;
use test_db::TestDb;
use tide::{
    http::{Method, Request, Response, Url},
    StatusCode,
};

pub async fn test_setup() -> TestServer {
    let test_db = TestDb::new().await;
    let db_pool = test_db.db();

    let server = server(db_pool).await;
    TestServer::new(server, test_db)
}

pub struct TestServer {
    server: Server<State>,
    //test_db: TestDb,
}

impl TestServer {
    fn new(server: Server<State>, _test_db: TestDb) -> Self {
        Self {
            server,
            // test_db
        }
    }

    pub async fn simulate(&self, req: Request) -> tide::Result<Response> {
        self.server.respond(req).await
    }
}

pub trait BodyJson {
    fn body_json<T: DeserializeOwned>(
        self,
    ) -> Pin<Box<dyn Future<Output = Result<T, Box<dyn std::error::Error>>>>>;
}

impl BodyJson for Response {
    fn body_json<T: DeserializeOwned>(
        mut self,
    ) -> Pin<Box<dyn Future<Output = Result<T, Box<dyn std::error::Error>>>>> {
        Box::pin(async move {
            let body = self.body_string().await?;
            println!("body = {}", body);
            Ok(serde_json::from_str(&body)?)
        })
    }
}

#[derive(Debug)]
pub struct TestRequest {
    url: String,
    headers: HashMap<String, String>,
    kind: TestRequestKind,
}

#[derive(Debug)]
pub enum TestRequestKind {
    Get,
    Post(Option<Value>),
}

impl TestRequest {
    pub async fn send(self, server: &TestServer) -> (Value, StatusCode, HashMap<String, String>) {
        let url = Url::parse(&format!("http://localhost:8080{}", self.url)).unwrap();

        let mut req = match self.kind {
            TestRequestKind::Get => Request::new(tide::http::Method::Get, url),
            TestRequestKind::Post(body) => {
                let mut req = Request::new(Method::Post, url);
                if let Some(body) = body {
                    req.set_body(body.to_string());
                    req.set_content_type("application/json".parse().unwrap());
                }
                req
            }
        };

        log_response(req.clone(), server).await;

        for (key, value) in self.headers {
            req.append_header(key.as_str(), value.as_str());
        }

        let res = server.simulate(req).await.unwrap();
        let status = res.status();
        let headers = res
            .iter()
            .flat_map(|(key, values)| {
                values
                    .iter()
                    .map(move |value| (key.as_str().to_string(), value.as_str().to_string()))
            })
            .collect::<HashMap<_, _>>();

        let json = res.body_json::<Value>().await.unwrap();

        (json, status, headers)
    }

    pub fn header(mut self, key: &str, value: impl ToString) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
}

async fn log_response(req_copy: Request, server: &TestServer) {
    let resp_copy = server.simulate(req_copy).await;
    if let Err(err) = resp_copy {
        print!("Error in the response {}", err)
    }
}

pub fn get(url: &str) -> TestRequest {
    TestRequest {
        url: url.to_string(),
        headers: HashMap::new(),
        kind: TestRequestKind::Get,
    }
}

pub fn post<T: Serialize>(url: &str, body: Option<T>) -> TestRequest {
    let body = body.map(|body| serde_json::to_value(body).unwrap());

    let kind = TestRequestKind::Post(body);
    TestRequest {
        url: url.to_string(),
        headers: HashMap::new(),
        kind,
    }
}
