mod rpn;

use actix_web::{post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct CalculateRequest {
    expression: String,
}

#[derive(Serialize, Deserialize)]
struct CalculateResponse {
    result: f64,
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

#[post("/calculate")]
async fn calculate(body: web::Json<CalculateRequest>) -> HttpResponse {
    match rpn::evaluate(&body.expression) {
        Ok(result) => HttpResponse::Ok().json(CalculateResponse { result }),
        Err(error) => HttpResponse::BadRequest().json(ErrorResponse { error }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_addr = "0.0.0.0:8080";
    println!("RPN calculator service listening on http://{bind_addr}");
    HttpServer::new(|| App::new().service(calculate))
        .bind(bind_addr)?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};

    #[actix_rt::test]
    async fn calculate_endpoint_happy_path() {
        let app = test::init_service(App::new().service(calculate)).await;
        let req = test::TestRequest::post()
            .uri("/calculate")
            .set_json(&CalculateRequest {
                expression: "3 4 +".to_string(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body: CalculateResponse = test::read_body_json(resp).await;
        assert_eq!(body.result, 7.0);
    }

    #[actix_rt::test]
    async fn calculate_endpoint_error_returns_400() {
        let app = test::init_service(App::new().service(calculate)).await;
        let req = test::TestRequest::post()
            .uri("/calculate")
            .set_json(&CalculateRequest {
                expression: "5 0 /".to_string(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body: ErrorResponse = test::read_body_json(resp).await;
        assert!(body.error.contains("division by zero"));
    }
}
