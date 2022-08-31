use std::collections::HashMap;

#[warn(dead_code)]
pub struct Collector {
    pub user_id: i64,
    pub chat_id: i64,
    pub stickers: HashMap<String, Vec<u64>>
}

const TEAMS: [&str; 32] = [
    "QAT", "ECU", "SEN", "NED",
    "ENG", "IRN", "USA", "WAL",
    "ARG", "KSA", "MEX", "POL",
    "FRA", "AUS", "DEN", "TUN",
    "ESP", "CRC", "GER", "JPN",
    "BEL", "CAN", "MAR", "CRO",
    "BRA", "SRB", "SUI", "CAM",
    "POR", "GHA", "URU", "KOR"
];
const CARDS_PER_TEAM: Vec<u8> = vec![1, 20];

const SPECIAL_STICKERS: [&str; 1] = ["FWC"];
const NON_TEAM_CARDS: Vec<u8> = vec![0, 29]; 

pub fn validate_sticker(s: &str) -> Option<&str> {
    if s.len() > 3 {
        let mut s_chars = s.chars();
        let mut prefix = String::from("");
        let mut number = String::from("");
        for _ in 0..2 {
            prefix.push(s_chars.next().unwrap());
        }
        loop {
            match s_chars.next() {
                Some(c) => number.push(c),
                None => {break;}
            }
        }
        return match &prefix.as_str() {
            m if TEAMS.contains(m) => match number.parse::<u8>() {
                Ok(n) if n >= CARDS_PER_TEAM[0] && n <= CARDS_PER_TEAM[1] => Some(s),
                _ => None
            },
            m if SPECIAL_STICKERS.contains(m) => match number.parse::<u8>() {
                Ok(n) if n >= NON_TEAM_CARDS[0] && n <= NON_TEAM_CARDS[1] => Some(s),
                _ => None
            },
            _ => None
        }
    }
    None
}