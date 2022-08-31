extern crate simple_error;

use std::env;
use std::collections::HashMap;
use aws_sdk_dynamodb::{Client, Error, model::AttributeValue};
use simple_error::SimpleError;
use reqwest::header::CONTENT_TYPE;

use aws_config;
use reqwest;
use crate::entities;

const TABLE: &str = "Collectors";
const TELEGRAM_URL: &str = "https://api.telegram.org";


pub async fn get_collector(user_id: i64, chat_id: i64) -> Result<Option<entities::Collector>, Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let item = client
        .get_item()
        .table_name(TABLE)
        .key(
            "UserId",
            AttributeValue::N(user_id.to_string()),
        )
        .key(
            "ChatId",
            AttributeValue::N(chat_id.to_string()),
        )
        .send()
        .await?;

    log::info!("{:?}", item);
    match item.item {
        Some(i) => {
            Ok(Some(entities::Collector {
                user_id: user_id,
                chat_id: chat_id,
                stickers: deserialize_stickers(i.get("Stickers").unwrap())
            }))
        },
        None => Ok(None)
    }
}

pub async fn save_collector(collector: entities::Collector) -> Result<(), SimpleError> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let request = client
        .put_item()
        .table_name(TABLE)
        .item("UserId", AttributeValue::N(collector.user_id.to_string()))
        .item("ChatId", AttributeValue::N(collector.chat_id.to_string()))
        .item("Stickers", serialize_stickers(collector.stickers));

    match request.send().await {
        Ok(_) => Ok(()),
        Err(e) => Err(SimpleError::new(format!("{}", e)))
    }
}

pub async fn send_message(chat_id: i64, msg: &str) -> Result<(), SimpleError> {
    let client = reqwest::Client::new();
    let token = env::var("TELEGRAM_TOKEN").unwrap_or(String::from(""));
    match client.post(format!("{}/bot{}/sendMessage", TELEGRAM_URL, token))
        .body(format!("{{\"chat_id\": {}, \"text\": \"{}\"}}", chat_id, msg))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await {
            Ok(resp) => {log::debug!("{:#?}", resp); Ok(())}
            Err(e) => Err(SimpleError::new(format!("{}", e)))
        }
}

fn deserialize_stickers(value: &AttributeValue) -> HashMap<String, Vec<u64>> {
    let mut stickers: HashMap<String, Vec<u64>> = HashMap::new();
    for sticker in value.as_m().unwrap() {
        let mut s_dates: Vec<u64> = vec![];
        let list = sticker.1.as_l().unwrap();
        for v in list {
            s_dates.push(v.as_n().unwrap().parse::<u64>().unwrap());
        }
        stickers.insert(sticker.0.to_string(), s_dates);
    }
    stickers
}

fn serialize_stickers(stickers: HashMap<String, Vec<u64>>) -> AttributeValue {
    let mut s_stickers: HashMap<String, AttributeValue> = HashMap::new();
    for sticker in stickers {
        let mut time_list: Vec<AttributeValue> = vec![];
        for time in sticker.1 {
            time_list.push(AttributeValue::N(time.to_string()));
        }
        s_stickers.insert(sticker.0, AttributeValue::L(time_list));
    }
    AttributeValue::M(s_stickers)
}