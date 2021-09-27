use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::SdkError;
use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;
#[derive(Clone, Serialize)]
struct Item {
    table: String,
    pk: String,
    sk: String,
}
#[derive(Deserialize)]
pub struct Request {
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(create);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn add_item(
    client: &Client,
    item: Item,
) -> Result<(), SdkError<aws_sdk_dynamodb::error::PutItemError>> {
    let pk = AttributeValue::S(item.pk);
    let sk = AttributeValue::S(item.sk);

    match client
        .put_item()
        .item("PK", pk)
        .item("SK", sk)
        .table_name(item.table)
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

async fn create(event: Value, _: Context) -> Result<Value, Error> {
    let request: Request = serde_json::from_value(event).unwrap();
    let v: Value = serde_json::from_str(&request.body)?;
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let item = Item {
        table: "learn-rust".to_string(),
        pk: Uuid::new_v4().to_string(),
        sk: v["text"].as_str().unwrap().to_string(),
    };
    add_item(&client, item.clone()).await?;
    Ok(json!({
        "statusCode": 200,
        "headers": { "Access-Control-Allow-Origin": "*", "Content-Type": "application/json" },
        "body": "success".to_string(),
    }))
}
