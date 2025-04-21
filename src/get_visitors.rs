use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error};

// Get the item from the table using the passed in table and item ID, returning the corresponding value.
pub async fn get_item(
    client: &Client,
    table_name: &str,
    item_id: &str,
) -> Result<Option<i32>, Error> {
    let result = client
        .get_item()
        .table_name(table_name)
        .key("ID", AttributeValue::S(item_id.to_string()))
        .send()
        .await?;

    // Check if the item was found if no item is found return none, then try to find the visitor attribute,
    // if the attribute is not found, return none, then parse the visitor attribute and return the visitor count,
    // if the visitor attribute does not contain a number return none.
    if let Some(item) = result.item {
        match item.get("visitors") {
            Some(AttributeValue::N(count)) => {
                match count.parse::<i32>() {
                    Ok(parsed_count) => Ok(Some(parsed_count)),
                    Err(_) => {
                        // Add logging error: 'visitors' attribute is not a valid number
                        Ok(None)
                    }
                }
            }
            _ => {
                // Add logging Error: 'visitors' attribute is missing
                Ok(None)
            }
        }
    } else {
        // Add logging error: Item not found in response
        Ok(None)
    }
}
