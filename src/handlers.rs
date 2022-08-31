use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::clients::{get_collector, save_collector};
use crate::entities::Collector;

pub async fn add_handler(message: &serde_json::Value) {
    let user_id = match get_id_from_message(message, "from") {
        Some(x) => x,
        _ => {log::error!("User ID doesn't exist!"); return}
    };
    let chat_id = match get_id_from_message(message, "chat") {
        Some(x) => x,
        _ => {log::error!("Chat ID doesn't exist!"); return}
    };
    let mut collector = match get_collector(user_id, chat_id).await {
        Ok(r) => match r {
            Some(c) => c,
            None => Collector {
                user_id,
                chat_id,
                stickers: HashMap::new(),
            }
        },
        Err(e) => {log::error!("Error getting Collector: {}", e); return}
    };
    let stickers: Vec<&str> = message.get("text").unwrap().as_str().unwrap().split(' ').collect();
    for s in stickers {
        if s != "/add" { // TODO: filter this and not-messages
            let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            let time_vec: Vec<u64> = vec![time.as_secs()];
            let s_vec = match collector.stickers.get(s) {
                Some(v) => [v.as_slice(), time_vec.as_slice()].concat(),
                None => time_vec
            };
            collector.stickers.insert(String::from(s), s_vec);
        }
    }
    match save_collector(collector).await {
        Ok(()) => (),
        Err(e) => log::error!("Error saving Collector: {}", e)
    }
}

fn get_id_from_message(message: &serde_json::Value, first_level: &str) -> Option<i64> {
    let user_id = match message.get(first_level) {
        Some(serde_json::Value::Object(x)) => x.get("id"),
        _ => None
    };
    match user_id {
        Some(serde_json::Value::Number(x)) => Some(x.as_i64().unwrap()),
        _ => None
    }
}