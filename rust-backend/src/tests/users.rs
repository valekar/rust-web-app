#[allow(dead_code)]
//use std::{thread, time};
use assert_json_diff::assert_json_include;
use chrono::Utc;
use uuid::Uuid;

use crate::tests::test_helpers::*;
use crate::User;

#[async_std::test]
async fn get_users_count() {
    let mut server = test_setup().await;

    let (json, status, _) = get(&format!("/")).send(&mut server).await;

    assert_eq!(status, 200);

    assert_json_include!(
        actual: json,
        expected :{
            json!({
                "count" :0
            })
        }
    );
}

#[async_std::test]
async fn get_all_users() {
    let mut server = test_setup().await;

    let (json, status, _) = get("/users").send(&mut server).await;

    assert_eq!(status, 200);

    assert_json_include!(
        actual : json,
        expected : {
            json!([])
        }
    );
}

#[async_std::test]
async fn create_user() {
    let server = test_setup().await;

    let user: User = User {
        id: Uuid::new_v4(),
        username: "srini".to_string(),
        created_at: Utc::now(),
        modified_at: Utc::now(),
    };
    let (json, status, _) = post("/user", Some(user)).send(&server).await;

    assert_eq!(status, 200);
}
