mod get_visitors;
mod update_visitors;

use aws_config::{BehaviorVersion, Region};
use aws_sdk_dynamodb::Client;
use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(_req: Request) -> Result<Response<Body>, Error> {

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new("eu-west-2"))
        .load()
        .await;

    let client = Client::new(&config);

    let table_name = "cloud-resume-challenge";
    let item_id = "blog";

    update_visitors::update_item(&client, table_name, item_id).await?;
    let total_visitors = get_visitors::get_item(&client, table_name, item_id).await?;

    let message = match total_visitors {
        Some(count) => format!("{{\"message\": \"visitors: {}\"}}", count),
        None => "{\"message\": \"no visitor count available.\"}".to_string(),
    };

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*") // Allow any origin
        .header("Access-Control-Allow-Methods", "GET") // Allow GET method
        .header("Access-Control-Allow-Headers", "Content-Type")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let handler = service_fn(function_handler);

    run(handler).await
}
