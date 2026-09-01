use aws_config::{BehaviorVersion, Region};
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error};
use cloud_resume_challenge_rust::update_visitors;

const TABLE_NAME: &str = "cloud-resume-challenge-test";
const REGION: &str = "eu-west-2";
const TEST_ITEM_ID: &str = "test_item";

// Integration test for visitor update using a test table and values,
#[tokio::test]
async fn test_update_visitors() -> Result<(), Box<Error>> {
    // Initialise AWS Config
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(REGION))
        .load()
        .await;

    let client = Client::new(&config);

    // Add test item to the test table
    client
        .put_item()
        .table_name(TABLE_NAME)
        .item("ID", AttributeValue::S(TEST_ITEM_ID.to_string()))
        .item("visitors", AttributeValue::N("0".to_string()))
        .send()
        .await
        .map_err(|e| Box::new(Error::from(e)))?;

    // Take the visitors from the table, then update the visitor item and take the new visitor value
    // returned directly from the update call itself.
    let visitors_current = get_test_visitors(&client, TABLE_NAME, TEST_ITEM_ID).await?;
    let visitors_updated = update_visitors::update_item(&client, TABLE_NAME, TEST_ITEM_ID)
        .await?
        .expect("Expected updated visitor count to be returned");

    // Assert that the updated visitor count is the previous +1, if not print the values that were returned
    assert_eq!(
        visitors_updated,
        visitors_current + 1,
        "Visitor count did not increment as expected: {}, got {}",
        visitors_current + 1,
        visitors_updated
    );

    cleanup_item(&client, TABLE_NAME, TEST_ITEM_ID).await?;

    Ok(())
}

// Get the visitors from the table, ensuring that it is not empty and is a number
async fn get_test_visitors(
    client: &Client,
    table: &str,
    item_id: &str,
) -> Result<i32, Box<Error>> {
    let response = client
        .get_item()
        .table_name(table)
        .key("ID", AttributeValue::S(item_id.to_string()))
        .send()
        .await
        .map_err(|e| Box::new(Error::from(e)))?;

    let visitors = response
        .item
        .and_then(|item| match item.get("visitors") {
            Some(AttributeValue::N(n)) => n.parse::<i32>().ok(),
            _ => None,
        })
        .expect("Expected item with a valid 'visitors' field");

    Ok(visitors)
}

// Utility function for cleaning up the test table
async fn cleanup_item(client: &Client, table: &str, item_id: &str) -> Result<(), Box<Error>> {
    client
        .delete_item()
        .table_name(table)
        .key("ID", AttributeValue::S(item_id.to_string()))
        .send()
        .await
        .map_err(|e| Box::new(Error::from(e)))?;
    Ok(())
}
