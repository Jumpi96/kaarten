use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::header::HeaderMap;
use lambda_runtime::{handler_fn, Context, Error};
use simple_logger::SimpleLogger;
use log::LevelFilter;

pub mod handlers;
pub mod clients;
pub mod entities;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(LevelFilter::Debug).with_utc_timestamps().init().unwrap();

    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: ApiGatewayProxyRequest, _: Context) -> Result<ApiGatewayProxyResponse, Error> {
    let body = &event.body;

    let empty_res = ApiGatewayProxyResponse {
        status_code: 201,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(String::from(""))),
        is_base64_encoded: Some(false),
    };

    let body: serde_json::Value =
        serde_json::from_str(body.as_ref().unwrap_or(&String::from("{}"))).unwrap();

    match body.get("message") {
        None => log::error!("Message doesn't exist!"),
        msg => {
            let command = msg.unwrap().as_str().unwrap();
            match command {
                command if command.starts_with("/add") => handlers::add_handler(&body).await,
                _ => log::info!("Discarding unknown input...")
            }
        }
    }
    Ok(empty_res)
}
