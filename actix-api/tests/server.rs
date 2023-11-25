mod common;

use actix_api::{get_app, CalcResult};
use actix_web::test;

/// Test login inserts pkce_challenge, pkce_verifier, csrf_state
/// And returns a login url with the pkce_challenge
///

#[actix_web::test]
async fn test_multiplication() {
    let db_url = std::env::var("PG_URL").unwrap();
    println!("DB_URL: {}", db_url);
    common::dbmate_up(&db_url);
    let mut app = test::init_service(get_app()).await;
    let req = test::TestRequest::get().uri("/multiplication?a=3&b=4").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 200);
    // Get body as json
    let body = test::read_body(resp).await;
    let body = String::from_utf8(body.to_vec()).unwrap();
    let body: CalcResult = serde_json::from_str(&body).unwrap();
    assert!((body.result - 12.0).abs() < 0.0001);
}

#[actix_web::test]
async fn test_multiplication_with_decimals() {
    let db_url = std::env::var("PG_URL").unwrap();
    println!("DB_URL: {}", db_url);
    common::dbmate_up(&db_url);
    let mut app = test::init_service(get_app()).await;
    let req = test::TestRequest::get().uri("/multiplication?a=3.5&b=4.3").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 200);
    // Get body as json
    let body = test::read_body(resp).await;
    let body = String::from_utf8(body.to_vec()).unwrap();
    let body: CalcResult = serde_json::from_str(&body).unwrap();
    assert!((body.result - 15.05).abs() < 0.0001);
}

#[actix_web::test]
async fn test_addition() {
    let db_url = std::env::var("PG_URL").unwrap();
    println!("DB_URL: {}", db_url);
    common::dbmate_up(&db_url);
    let mut app = test::init_service(get_app()).await;
    let req = test::TestRequest::get().uri("/addition?a=3&b=4").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 200);
    // Get body as json
    let body = test::read_body(resp).await;
    let body = String::from_utf8(body.to_vec()).unwrap();
    let body: CalcResult = serde_json::from_str(&body).unwrap();
    assert!((body.result - 7.0).abs() < 0.0001);
}

#[actix_web::test]
async fn addition_with_overflow() {
    let db_url = std::env::var("PG_URL").unwrap();
    println!("DB_URL: {}", db_url);
    common::dbmate_up(&db_url);
    let mut app = test::init_service(get_app()).await;
    let max_f32 = std::f32::MAX;
    let req = test::TestRequest::get().uri(&format!("/addition?a={}&b=1", max_f32)).to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 200);
    // Get body as json
    let body = test::read_body(resp).await;
    let body = String::from_utf8(body.to_vec()).unwrap();
    let body: CalcResult = serde_json::from_str(&body).unwrap();
    assert_eq!(body.result, max_f32);
}

#[actix_web::test]
async fn test_substraction() {
    let db_url = std::env::var("PG_URL").unwrap();
    println!("DB_URL: {}", db_url);
    common::dbmate_up(&db_url);
    let mut app = test::init_service(get_app()).await;
    let req = test::TestRequest::get().uri("/substraction?a=3&b=4").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 200);
    // Get body as json
    let body = test::read_body(resp).await;
    let body = String::from_utf8(body.to_vec()).unwrap();
    let body: CalcResult = serde_json::from_str(&body).unwrap();
    assert!((body.result - -1.0).abs() < 0.0001);
}

#[actix_web::test]
async fn test_substraction_underflow() {
    let db_url = std::env::var("PG_URL").unwrap();
    println!("DB_URL: {}", db_url);
    common::dbmate_up(&db_url);
    let mut app = test::init_service(get_app()).await;
    let min_f32 = std::f32::MIN;
    let req = test::TestRequest::get().uri(&format!("/substraction?a={}&b=1", min_f32)).to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 200);
    // Get body as json
    let body = test::read_body(resp).await;
    let body = String::from_utf8(body.to_vec()).unwrap();
    let body: CalcResult = serde_json::from_str(&body).unwrap();
    assert_eq!(body.result, min_f32);
}

#[actix_web::test]
async fn test_division() {
    let db_url = std::env::var("PG_URL").unwrap();
    println!("DB_URL: {}", db_url);
    common::dbmate_up(&db_url);
    let mut app = test::init_service(get_app()).await;
    let req = test::TestRequest::get().uri("/division?a=3&b=4").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 200);
    // Get body as json
    let body = test::read_body(resp).await;
    let body = String::from_utf8(body.to_vec()).unwrap();
    let body: CalcResult = serde_json::from_str(&body).unwrap();
    assert!((body.result - 0.75).abs() < 0.0001);
}

#[actix_web::test]
async fn test_division_by_zero() {
    let db_url = std::env::var("PG_URL").unwrap();
    println!("DB_URL: {}", db_url);
    common::dbmate_up(&db_url);
    let mut app = test::init_service(get_app()).await;
    let req = test::TestRequest::get().uri("/division?a=3&b=0").to_request();
    let resp = test::call_service(&mut app, req).await;
    let body = test::read_body(resp).await;
    let body = String::from_utf8(body.to_vec()).unwrap();
    assert_eq!(body, "Division by zero");
}