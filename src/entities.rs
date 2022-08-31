use std::collections::HashMap;

#[warn(dead_code)]
pub struct Collector {
    pub user_id: i64,
    pub chat_id: i64,
    pub stickers: HashMap<String, Vec<u64>>
}