use aws_config::{BehaviorVersion, Region};
use aws_sdk_dynamodb::Client;
use cloud_resume_challenge_rust::get_visitors;
use cloud_resume_challenge_rust::update_visitors;
use cloud_resume_challenge_rust::utils::{
    build_cors_layer, handle_favicon_request, reject_non_get_method,
};
use lambda_http::tower::ServiceBuilder;
use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};

const REGION: &str = "eu-west-2";
const TABLE_NAME: &str = "cloud-resume-challenge";

async fn function_handler(req: Request) -> Result<Response<Body>, Error> {
    if req.uri().path() == "/favicon.ico" {
        return Ok(handle_favicon_request());
    }

    // Initialise AWS Config
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(REGION))
        .load()
        .await;

    let client = Client::new(&config);

    let item_id = "blog";

    if let Some(res) = reject_non_get_method(&req) {
        return Ok(res);
    }

    update_visitors::update_item(&client, TABLE_NAME, item_id).await?;
    let total_visitors = get_visitors::get_item(&client, TABLE_NAME, item_id).await?;

    // Create response message from total visitors, if there isn't a visitor count return error to the user
    let message = match total_visitors {
        Some(count) => format!("{{\"visitors\": {}}}", count),
        None => {
            return Ok(Response::builder()
                .status(500) // Internal Server Error status code
                .header("content-type", "application/json")
                .body("{\"error\": \"Visitor count unavailable.\"}".into())
                .unwrap());
        }
    };

    // Send OK response with the visitor count in a JSON format.
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

    let cors_layer = build_cors_layer();

    let handler = ServiceBuilder::new()
        .layer(cors_layer)
        .service(service_fn(function_handler));

    run(handler).await
}
