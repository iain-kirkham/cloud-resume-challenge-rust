use aws_sdk_dynamodb::types::{AttributeValue, ReturnValue};
use aws_sdk_dynamodb::{Client, Error};
use lambda_http::tracing;

// Update the visitor count by one using attribute expressions, returning the new count from the
// same call so a separate get_item round trip to DynamoDB is not required.
pub async fn update_item(
    client: &Client,
    table_name: &str,
    item_id: &str,
) -> Result<Option<i32>, Box<Error>> {
    let result = client
        .update_item()
        .table_name(table_name)
        .key("ID", AttributeValue::S(item_id.to_string()))
        .update_expression("ADD visitors :inc")
        .expression_attribute_values(":inc", AttributeValue::N("1".to_string()))
        .return_values(ReturnValue::UpdatedNew)
        .send()
        .await
        .map_err(|e| Box::new(Error::from(e)))?;

    match result.attributes.and_then(|attrs| attrs.get("visitors").cloned()) {
        Some(AttributeValue::N(count)) => match count.parse::<i32>() {
            Ok(parsed_count) => Ok(Some(parsed_count)),
            Err(_) => {
                tracing::error!(table_name, item_id, raw_count = %count, "invalid visitor count value");
                Ok(None)
            }
        },
        _ => {
            tracing::error!(table_name, item_id, "visitors attribute missing after update");
            Ok(None)
        }
    }
}
