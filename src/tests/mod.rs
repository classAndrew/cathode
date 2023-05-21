use axum::{http::{Request, self}, body::{Body, HttpBody}};
use tower::ServiceExt;
use serde_json::json;

use crate::{get_app, models::c2_s::{SubmitWarAttemptC2S, Tower}};

#[tokio::test]
async fn submit_war() {
    let (_, app) = get_app().await.unwrap();
    
    let sample_war = SubmitWarAttemptC2S {
        class: "Mage".to_string(),
        uuid: "andrew_uuid".to_string(),
        name: "andrew".to_string(),
        tower: Tower {
            owner: "Test Guild".to_string(),
            attack_speed: 4,
            defense: 4,
            damage: "4-8".to_string(),
            health: 4,
            territory: "Test Territory".to_string()
        },
    };

    let mut response = app
        .oneshot(
            Request::builder()
                    .method(http::Method::POST)
                    .uri("/submit_war_attempt")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_vec(&json!(
                            sample_war
                        )).unwrap()
                    ))
                    .unwrap()
        ).await.unwrap();

    println!("{:?}", response.data().await.unwrap());
}