use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::clients::{get_collector, save_collector, send_message};
use crate::entities::{Collector, validate_sticker};

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
        match validate_sticker(s) {
            Some(sticker) => {
                let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                let time_vec: Vec<u64> = vec![time.as_secs()];
                let s_vec = match collector.stickers.get(sticker) {
                    Some(v) => [v.as_slice(), time_vec.as_slice()].concat(),
                    None => time_vec
                };
                collector.stickers.insert(String::from(sticker), s_vec);
            },
            None if s != "/add" => log::warn!("Not a valid sticker: {}", s),
            None => ()
        }
    }
    match save_collector(collector).await {
        Ok(()) => (),
        Err(e) => log::error!("Error saving Collector: {}", e)
    }
}

pub async fn remove_handler(message: &serde_json::Value) {
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
            None => (Collector {
                user_id,
                chat_id,
                stickers: HashMap::new(),
            })
        },
        Err(e) => {log::error!("Error getting Collector: {}", e); return}
    };
    let stickers: Vec<&str> = message.get("text").unwrap().as_str().unwrap().split(' ').collect();
    for s in stickers {
        match validate_sticker(s) {
            Some(sticker) => {
                match collector.stickers.get(sticker) {
                    Some(v) => {
                        let mut new_v: Vec<u64> = vec![];
                        for i in 0..v.len()-1 {
                            new_v.push(*v.get(i).unwrap());
                        }
                        match new_v.len() {
                            0 => collector.stickers.remove(sticker),
                            _ => collector.stickers.insert(String::from(sticker), new_v)
                        }
                    },
                    None => None
                };
                
            },
            None if s != "/remove" => log::warn!("Not a valid sticker: {}", s),
            None => ()
        }
    }
    match save_collector(collector).await {
        Ok(()) => (),
        Err(e) => log::error!("Error saving Collector while removing: {}", e)
    }
}

pub async fn list_handler(message: &serde_json::Value) {
    let user_id = match get_id_from_message(message, "from") {
        Some(x) => x,
        _ => {log::error!("User ID doesn't exist!"); return}
    };
    let chat_id = match get_id_from_message(message, "chat") {
        Some(x) => x,
        _ => {log::error!("Chat ID doesn't exist!"); return}
    };
    let collector = match get_collector(user_id, chat_id).await {
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
    let mut message = String::from("🏆 Your WK 2022 stickers ⚽\n");
    let mut groups: HashMap<String, HashMap<String, u8>> = HashMap::new();
    for sticker in collector.stickers.keys() {
        let mut sticker_chars = sticker.chars();
        let mut prefix = String::from("");
        let mut number = String::from("");
        for _ in 0..3 {
            prefix.push(sticker_chars.next().unwrap());
        }
        loop {
            match sticker_chars.next() {
                Some(c) => number.push(c),
                None => {break;}
            }
        }
        match groups.get_mut(&prefix) {
            Some(g) => match g.get_mut(&number) {
                Some(n) => {let c = *n + 1; g.insert(number, c);},
                None => {g.insert(number, 1);}
            },
            None => {
                let mut group_map = HashMap::new();
                group_map.insert(number, 1);
                groups.insert(prefix, group_map);
            }
        }
    }
    for group in groups.keys() {
        message.push_str(&format!("{} ", group));
        for sticker in groups.get(group).unwrap() {
            message.push_str(&format!("{}{} ", sticker.0, number_to_emoji(sticker.1)))
        }
        message.push_str("\n");
    }
    match send_message(chat_id, &message).await {
        Ok(_) => (),
        Err(e) => {log::error!("Error sending message: {}", e); return}
    };
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

fn number_to_emoji(n: &u8) -> String {
    String::from(
        match n {
            1 => "",
            2 => "x2️⃣",
            3 => "x3️⃣",
            4 => "x4️⃣",
            5 => "x5️⃣",
            6 => "x6️⃣",
            7 => "x7️⃣",
            8 => "x8️⃣",
            9 => "x9️⃣",
            _ => "x🔟+"
        }
    )
}