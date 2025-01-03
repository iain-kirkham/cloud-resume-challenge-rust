use aws_config::{BehaviorVersion, Region};
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error};
use cloud_resume_challenge_rust::get_visitors::get_item;

const TABLE_NAME: &str = "cloud-resume-challenge-test";
const REGION: &str = "eu-west-2";

// Integration test for get visitors function, creates a test item to read from the test table
// Then checks that the item is the same as expected, and then removes the test item after tests.
#[tokio::test]
async fn test_get_item() -> Result<(), Error> {
    // Initialise AWS Config
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(REGION))
        .load()
        .await;

    let client = Client::new(&config);

    let item_id = "test";
    let visitor_count = 10;

    // Add test item to the table
    client
        .put_item()
        .table_name(TABLE_NAME)
        .item("ID", AttributeValue::S(item_id.to_string()))
        .item("visitors", AttributeValue::N(visitor_count.to_string()))
        .send()
        .await?;

    // Retrieve test item using get_item function
    let result = get_item(&client, TABLE_NAME, item_id).await?;

    // Assert that the retrieved visitor count matches the value expected.
    assert_eq!(
        result,
        Some(visitor_count),
        "Expected visitor count to be {}, but got {:?}",
        visitor_count,
        result
    );

    cleanup_item(&client, TABLE_NAME, item_id).await?;

    Ok(())
}

// Utility function for cleaning up the test table
async fn cleanup_item(client: &Client, table: &str, item_id: &str) -> Result<(), Error> {
    client
        .delete_item()
        .table_name(table)
        .key("ID", AttributeValue::S(item_id.to_string()))
        .send()
        .await?;
    Ok(())
}
