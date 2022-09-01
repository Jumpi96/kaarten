use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::clients::{get_collector, save_collector, send_message};
use crate::entities::{Collector, validate_sticker};
use crate::entities;

pub async fn add_handler(message: &serde_json::Value) {
    match get_collector_from_message(message).await {
        Some(mut collector) => {
            let stickers: Vec<&str> = message.get("text").unwrap().as_str().unwrap().split(' ').collect();
            let mut count_new = 0;
            let mut count_dup = 0;
            for s in stickers {
                match validate_sticker(s) {
                    Some(sticker) => {
                        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                        let time_vec: Vec<u64> = vec![time.as_secs()];
                        let s_vec = match collector.stickers.get(sticker) {
                            Some(v) => {count_dup += 1; [v.as_slice(), time_vec.as_slice()].concat()},
                            None => {count_new += 1; time_vec}
                        };
                        collector.stickers.insert(String::from(sticker), s_vec);
                    },
                    None if s != "/add" => log::warn!("Not a valid sticker: {}", s),
                    None => ()
                }
            }
            let chat_id = collector.chat_id;
            let message = &format!("🏆✍️ Great! {} new stickers and {} duplicated ones.", count_new, count_dup);
            match save_collector(collector).await {
                Ok(()) => match send_message(chat_id, message).await {
                    Ok(_) => (),
                    Err(e) => {log::error!("Error sending message: {}", e); return}
                },
                Err(e) => log::error!("Error saving Collector: {}", e)
            }
        },
        None => ()
    }
    
}

pub async fn remove_handler(message: &serde_json::Value) {
    match get_collector_from_message(message).await {
        Some(mut collector) => {
            let stickers: Vec<&str> = message.get("text").unwrap().as_str().unwrap().split(' ').collect();
            let mut count = 0;
            for s in stickers {
                match validate_sticker(s) {
                    Some(sticker) => {
                        match collector.stickers.get(sticker) {
                            Some(v) => {
                                count += 1;
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
            let chat_id = collector.chat_id;
            match save_collector(collector).await {
                Ok(()) => match send_message(chat_id, &format!("🏆❌ Done! {} stickers removed.", count)).await {
                    Ok(_) => (),
                    Err(e) => {log::error!("Error sending message: {}", e); return}
                },
                Err(e) => log::error!("Error saving Collector while removing: {}", e)
            }
            
        },
        None => ()
    }
}

pub async fn list_handler(message: &serde_json::Value) {
    match get_collector_from_message(message).await {
        Some(collector) => {
            let mut message = String::from("🏆 Your WK 2022 stickers ⚽\n");
            let groups = collector.stickers_as_groups();
            for group in groups.keys() {
                message.push_str(&format!("{} ", group));
                for sticker in groups.get(group).unwrap() {
                    message.push_str(&format!("{}{} ", sticker.0, number_to_emoji(sticker.1)))
                }
                message.push_str("\n");
            }
            match send_message(collector.chat_id, &message).await {
                Ok(_) => (),
                Err(e) => {log::error!("Error sending message: {}", e); return}
            };
        },
        None => ()
    }
}

pub async fn report_handler(message: &serde_json::Value) {
    match get_collector_from_message(message).await {
        Some(collector) => {
            let mut message = String::from("⚽🏆 Your WK 2022 report 📒⚽\n");
            let groups = collector.stickers_as_groups();
            let mut total: u16 = 0;
            let mut have: u16= 0;
            let mut repeated: u16 = 0;

            for group in entities::SPECIAL_STICKERS {
                let group_total = u16::from(entities::NON_TEAM_CARDS.1 - entities::NON_TEAM_CARDS.0 + 1);
                let mut group_have: u16 = 0;
                let mut group_repeated: u16 = 0;

                match groups.get(group) {
                    Some(v) => {
                        for i in entities::NON_TEAM_CARDS.0..entities::NON_TEAM_CARDS.1 {
                            match v.get(&i.to_string()) {
                                Some(n) => match n {
                                    1 => {group_have += 1},
                                    r => {group_have += 1; group_repeated += u16::from(*r) - 1}
                                },
                                None => ()
                            }
                        }
                    },
                    None => ()
                }

                total += group_total;
                have += group_have;
                repeated += group_repeated;

                let percentage = format_percentage((group_have as f32 / group_total as f32) * 100.0);
                message.push_str(&format!("{}: {} ({}/{}/{})\n", group, percentage, group_have, group_repeated, group_total));
            }

            for group in entities::TEAMS {
                let group_total = u16::from(entities::CARDS_PER_TEAM.1 - entities::CARDS_PER_TEAM.0 + 1);
                let mut group_have: u16 = 0;
                let mut group_repeated: u16 = 0;

                match groups.get(group) {
                    Some(v) => {
                        for i in entities::CARDS_PER_TEAM.0..entities::CARDS_PER_TEAM.1 {
                            match v.get(&i.to_string()) {
                                Some(n) => match n {
                                    1 => {group_have += 1},
                                    r => {group_have += 1; group_repeated += u16::from(*r) - 1}
                                },
                                None => ()
                            }
                        }
                    },
                    None => ()
                }

                total += group_total;
                have += group_have;
                repeated += group_repeated;

                let percentage = format_percentage((group_have as f32 / group_total as f32) * 100.0); 
                message.push_str(&format!("{}: {} ({}/{}/{})\n", group, percentage, group_have, group_repeated, group_total));
            }
            
            let percentage = format_percentage((have as f32 / total as f32) * 100.0); 
            message.push_str(&format!("\n🏆 {} ({}/{}/{})🏆", percentage, have, repeated, total));
            match send_message(collector.chat_id, &message).await {
                Ok(_) => (),
                Err(e) => {log::error!("Error sending message: {}", e); return}
            };
        },
        None => ()
    }
}

async fn get_collector_from_message(message: &serde_json::Value) -> Option<Collector> {
    let user_id = match get_id_from_message(message, "from") {
        Some(x) => x,
        _ => {log::error!("User ID doesn't exist!"); return None}
    };
    let chat_id = match get_id_from_message(message, "chat") {
        Some(x) => x,
        _ => {log::error!("Chat ID doesn't exist!"); return None}
    };
    match get_collector(user_id, chat_id).await {
        Ok(r) => match r {
            Some(c) => Some(c),
            None => Some(Collector {
                user_id,
                chat_id,
                stickers: HashMap::new(),
            })
        },
        Err(e) => {log::error!("Error getting Collector: {}", e); None}
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

fn format_percentage(p: f32) -> String {
    let emoji = match p {
        p if p == 100.0 => "🟩",
        p if p > 50.0 => "🟨",
        _ => "⬜"
    };
    return format!("{:.2}%{}", p, emoji);
}
