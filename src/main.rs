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

    log::info!("Body received: {}", body.as_ref().unwrap_or(&String::from("no body")));

    let empty_res = ApiGatewayProxyResponse {
        status_code: 201,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(String::from(""))),
        is_base64_encoded: Some(false),
    };

    let body: serde_json::Value =
        serde_json::from_str(body.as_ref().unwrap_or(&String::from("{}"))).unwrap();
    
    log::info!("Message body received: {}", body);

    match body.get("message") {
        Some(message) => match message.as_object() {
            Some(message_obj) => match message_obj.get("text") {
                Some(text) => match text.as_str() {
                    Some(text) if text.starts_with("/add") => handlers::add_handler(message).await,
                    Some(text) if text.starts_with("/remove") => handlers::remove_handler(message).await,
                    Some(text) if text.starts_with("/list") => handlers::list_handler(message).await,
                    Some(text) if text.starts_with("/report") => handlers::report_handler(message).await,
                    Some(_) => log::info!("Discarding unknown input..."),
                    None => log::error!("Discarding invalid input...")
                },
                None => log::error!("Discarding invalid input...")
            },
            None => log::error!("Discarding invalid input...")
        },
        None => log::error!("Message doesn't exist!"),
    }
    Ok(empty_res)
}
