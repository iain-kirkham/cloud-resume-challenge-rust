use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error};

// Update the visitor count by one using attribute expressions.
pub async fn update_item(client: &Client, table_name: &str, item_id: &str) -> Result<(), Error> {
    client
        .update_item()
        .table_name(table_name)
        .key("ID", AttributeValue::S(item_id.to_string()))
        .update_expression("ADD visitors :inc")
        .expression_attribute_values(":inc", AttributeValue::N("1".to_string()))
        .send()
        .await?;
    Ok(())
}
