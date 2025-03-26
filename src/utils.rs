use lambda_http::http::Method;
use lambda_http::{Body, Request, Response};
use tower_http::cors::{Any, CorsLayer};

pub fn build_cors_layer() -> CorsLayer {
    let origins = [
        "http://localhost:4321".parse().unwrap(),
        "https://www.iainkirkham.dev".parse().unwrap(),
        "https://iainkirkham.dev".parse().unwrap(),
    ];

    CorsLayer::new()
        .allow_methods([Method::POST, Method::OPTIONS])
        .allow_origin(origins)
        .allow_headers(Any)
}

pub fn reject_non_post_method(req: &Request) -> Option<Response<Body>> {
    if req.method() != Method::POST {
        Some(
            Response::builder()
                .status(405) // HTTP Method not allowed status code
                .body("Method Not Allowed".into())
                .unwrap(),
        )
    } else {
        None
    }
}
