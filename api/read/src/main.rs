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
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(read);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn read(event: ApiGatewayProxyRequest, _: Context) -> Result<Value, Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    println!("{}", event.path_parameters["id"]);
    let item = Item {
        table: "learn-rust".to_string(),
        pk: event.path_parameters["id"].to_string(),
    };
    let _val: aws_sdk_dynamodb::output::QueryOutput = query_item(&client, item).await?;
    Ok(json!({
        "statusCode": 200,
        "headers": { "Access-Control-Allow-Origin": "*", "Content-Type": "application/json" },
        "body": "success".to_string(),
    }))
}

async fn query_item(
    client: &Client,
    item: Item,
) -> Result<aws_sdk_dynamodb::output::QueryOutput, SdkError<aws_sdk_dynamodb::error::QueryError>> {
    let value = &item.pk;
    let key = "PK";
    let pk = AttributeValue::S(value.to_string());

    match client
        .query()
        .table_name(item.table)
        .key_condition_expression("#key = :value".to_string())
        .expression_attribute_names("#key".to_string(), key.to_string())
        .expression_attribute_values(":value".to_string(), pk)
        .send()
        .await
    {
        Ok(resp) => Ok(resp),
        Err(e) => Err(e),
    }
}
