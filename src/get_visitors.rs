use aws_sdk_dynamodb::{Client, Error};
use aws_sdk_dynamodb::types::AttributeValue;

pub async fn get_item(client: &Client, table_name: &str, item_id: &str) -> Result<Option<i32>, Error> {
    let result = client.get_item()
        .table_name(table_name)
        .key("ID", AttributeValue::S(item_id.to_string()))
        .send()
        .await?;

    // Check if the item was found and extract the visitors attribute
    if let Some(item) = result.item {
        if let Some(AttributeValue::N(count)) = item.get("visitors") {
            match count.parse::<i32>() {
                Ok(parsed_count) => Ok(Some(parsed_count)),
                Err(_) => {
                    //eprintln!("Error: 'visitors' attribute is not a valid number.");
                    Ok(None) //TODO: add visitors attribute is not a number logging
                }
            }
        } else {
            Ok(None) //TODO: add visitors attribute missing log
        }
    } else {
        Ok(None) //TODO: add cannot find item log
    }
}