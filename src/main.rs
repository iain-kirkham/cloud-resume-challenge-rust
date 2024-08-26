mod get_visitors;
mod update_visitors;

use aws_config::{BehaviorVersion, Region};
use aws_sdk_dynamodb::Client;
use lambda_http::http::Method;
use lambda_http::tower::ServiceBuilder;
use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};
use tower_http::cors::{Any, CorsLayer};



async fn function_handler(req: Request) -> Result<Response<Body>, Error> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new("eu-west-2"))
        .load()
        .await;

    let client = Client::new(&config);

    let table_name = "cloud-resume-challenge";
    let item_id = "blog";

    if req.method() != Method::GET {
        return Ok(Response::builder()
        .status(405)
        .body("Method Not Allowed".into())
        .unwrap());
}
    update_visitors::update_item(&client, table_name, item_id).await?;
    let total_visitors = get_visitors::get_item(&client, table_name, item_id).await?;

    let message = match total_visitors {
        Some(count) => format!("{{\"visitors\": {}}}", count),
        None => "{\"message\": \"no visitor count available.\"}".to_string(),
    };

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(message.into())
        .map_err(Box::new)?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let origins = ["http://localhost:8000".parse().unwrap(),
        "https://www.google.com".parse().unwrap()];

    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(origins)
        .allow_headers(Any);

        let handler = ServiceBuilder::new()
            .layer(cors_layer)
            .service(service_fn(function_handler));

    run(handler).await
}