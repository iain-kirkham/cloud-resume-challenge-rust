use lambda_http::http::Method;
use lambda_http::{Body, Request, Response};
use tower_http::cors::{Any, CorsLayer};

pub fn build_cors_layer() -> CorsLayer {
    let origins = [
        "http://localhost:4321".parse().expect("invalid CORS origin: http://localhost:4321"),
        "https://www.iainkirkham.dev".parse().expect("invalid CORS origin: https://www.iainkirkham.dev"),
        "https://iainkirkham.dev".parse().expect("invalid CORS origin: https://iainkirkham.dev"),
        "https://test.iainkirkham.dev".parse().expect("invalid CORS origin: https://test.iainkirkham.dev"),
    ];

    CorsLayer::new()
        .allow_methods([Method::POST, Method::OPTIONS])
        .allow_origin(origins)
        .allow_headers(Any)
}

pub fn reject_non_post_method(req: &Request) -> Option<Response<Body>> {
    // return OPTIONS preflight requests with 204 No Content and do not
    // increment visitor counter. For non-POST methods return 405.
    if req.method() == Method::OPTIONS {
        // Build a 204 response. If the builder fails,
        // fall back to an empty response to avoid panicking.
        let resp = Response::builder()
            .status(204)
            .body(Body::Empty)
            .unwrap_or_else(|_| Response::new(Body::Empty));
        Some(resp)
    } else if req.method() != Method::POST {
        let resp = Response::builder()
            .status(405) // HTTP Method not allowed status code
            .body("Method is Not Allowed".into())
            .unwrap_or_else(|_| Response::new(Body::from("Method is Not Allowed")));
        Some(resp)
    } else {
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::http::Method;
    use lambda_http::http::Request as HttpRequest;

    #[test]
    fn options_returns_204_no_content() {
        let req = HttpRequest::builder()
            .method(Method::OPTIONS)
            .body(Body::Empty)
            .expect("failed to build request");

        let res = reject_non_post_method(&req).expect("expected Some(Response) for OPTIONS");
        assert_eq!(res.status().as_u16(), 204);
    }

    #[test]
    fn post_returns_none() {
        let req = HttpRequest::builder()
            .method(Method::POST)
            .body(Body::Empty)
            .expect("failed to build request");

        assert!(reject_non_post_method(&req).is_none());
    }

    #[test]
    fn other_methods_return_405() {
        let req = HttpRequest::builder()
            .method(Method::GET)
            .body(Body::Empty)
            .expect("failed to build request");

        let res = reject_non_post_method(&req).expect("expected Some(Response) for GET");
        assert_eq!(res.status().as_u16(), 405);
    }

    #[test]
    fn build_cors_layer_constructs() {
        // Ensure the function returns a CorsLayer and
        // that construction doesn't panic at runtime.
        let _layer: CorsLayer = build_cors_layer();
    }
}
