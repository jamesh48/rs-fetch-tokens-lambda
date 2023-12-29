use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{model::AttributeValue, Client, Region};
use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string, Value as JsonValue};
use std::collections::HashMap;
use std::{env, error::Error};
use urlencoding::decode;

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct QueryStringParams {
    pub athlete_id: String,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct InputParams {
    pub query_string_parameters: QueryStringParams,
}

#[derive(Serialize)]
pub struct ResponseBody {
    pub body: String,
    pub request_id: String,
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

// Convert AttributeValue to serde_json::Value
fn convert_attribute_value(value: AttributeValue) -> JsonValue {
    match value {
        AttributeValue::S(s) => JsonValue::String(s),
        AttributeValue::N(n) => {
            // Use serde_json::Number::from_f64 for the conversion
            JsonValue::Number(serde_json::Number::from_f64(n.parse::<f64>().unwrap()).unwrap())
        }
        AttributeValue::Bool(b) => JsonValue::Bool(b),
        // Add other conversions as needed
        _ => JsonValue::Null,
    }
}

// Convert HashMap<String, AttributeValue> to HashMap<String, serde_json::Value>
fn convert_attribute_values(input: HashMap<String, AttributeValue>) -> HashMap<String, JsonValue> {
    input
        .into_iter()
        .map(|(k, v)| (k, convert_attribute_value(v)))
        .collect()
}

async fn get_tokens(
    athlete_id: String,
    dynamodb: Client,
    table_name: String,
) -> Result<JsonValue, Box<dyn Error>> {
    let result = dynamodb
        .get_item()
        .table_name(table_name)
        .key("athleteId", AttributeValue::S(athlete_id.to_string()))
        .send()
        .await?;

    let result_json = convert_attribute_values(result.item.unwrap_or_default());

    // Uncomment for Lambda/ALB Integration
    // let body = to_string(&result_json).expect("Failed to stringify JSON");

    return Ok(json!({
        "data": result_json
    // uncomment for Lambda/ALB Integration
    // "body": body,
    //   "isBase64Encoded": false,
    //   "statusCode": 200,
    //   "statusDescription": "200 OK",
    //   "headers": { "Content-Type": "application/json" },
    }));
}

async fn handler(event: LambdaEvent<InputParams>) -> Result<JsonValue, Box<dyn Error>> {
    let table_name: String = "srg-token-table".to_string();

    println!("{:#?}", table_name);
    let region_provider =
        RegionProviderChain::first_try(env::var("APPLICATION_REGION").ok().map(Region::new))
            .or_default_provider()
            .or_else(Region::new("us-east-1"));

    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    let (event, _context) = event.into_parts();

    let athlete_id = decode(&event.query_string_parameters.athlete_id)
        .expect("UTF-8")
        .to_string();

    let response_json = get_tokens(athlete_id, client, table_name).await;
    if let Ok(ref json_value) = response_json {
        println!("Final Response: {:#}", json_value);
        return response_json;
    }
    return response_json;
}
