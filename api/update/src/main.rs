use aws_lambda_events::event::apigw::ApiGatewayProxyRequest;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::SdkError;
use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{json, Value};
#[derive(Clone)]
struct Item {
    table: String,
    pk: String,
    sk: String,
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(update);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn update(event: ApiGatewayProxyRequest, _: Context) -> Result<Value, Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    println!("{}", event.path_parameters["id"]);
    let v: Value = serde_json::from_str(&event.body.unwrap())?;
    let item = Item {
        table: "learn-rust".to_string(),
        pk: event.path_parameters["id"].to_string(),
        sk: v["text"].as_str().unwrap().to_string(),
    };
    update_item(&client, item).await?;
    Ok(json!({
        "statusCode": 204,
        "headers": { "Access-Control-Allow-Origin": "*", "Content-Type": "application/json" },
        "body": "success".to_string(),
    }))
}

async fn update_item(
    client: &Client,
    item: Item,
) -> Result<
    aws_sdk_dynamodb::output::UpdateItemOutput,
    SdkError<aws_sdk_dynamodb::error::UpdateItemError>,
> {
    let value = &item.pk;
    let key = "PK";
    let pk = AttributeValue::S(value.to_string());

    match client
        .update_item()
        .table_name(&item.table)
        .key(key, pk)
        .send()
        .await
    {
        Ok(resp) => Ok(resp),
        Err(e) => Err(e),
    }
}
