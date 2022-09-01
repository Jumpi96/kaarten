use std::collections::HashMap;

#[warn(dead_code)]
pub struct Collector {
    pub user_id: i64,
    pub chat_id: i64,
    pub stickers: HashMap<String, Vec<u64>>
}

pub const TEAMS: [&str; 32] = [
    "QAT", "ECU", "SEN", "NED",
    "ENG", "IRN", "USA", "WAL",
    "ARG", "KSA", "MEX", "POL",
    "FRA", "AUS", "DEN", "TUN",
    "ESP", "CRC", "GER", "JPN",
    "BEL", "CAN", "MAR", "CRO",
    "BRA", "SRB", "SUI", "CMR",
    "POR", "GHA", "URU", "KOR"
];
pub const CARDS_PER_TEAM: (u8, u8) = (1, 20);

pub const SPECIAL_STICKERS: [&str; 1] = ["FWC"];
pub const NON_TEAM_CARDS: (u8, u8) = (0, 29); 

pub fn validate_sticker(s: &str) -> Option<&str> {
    if s.len() > 3 {
        let mut s_chars = s.chars();
        let mut prefix = String::from("");
        let mut number = String::from("");
        for _ in 0..3 {
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
                Ok(n) if n >= CARDS_PER_TEAM.0 && n <= CARDS_PER_TEAM.1 => Some(s),
                _ => None
            },
            m if SPECIAL_STICKERS.contains(m) => match number.parse::<u8>() {
                Ok(n) if n >= NON_TEAM_CARDS.0 && n <= NON_TEAM_CARDS.1 => Some(s),
                _ => None
            },
            _ => None
        }
    }
    None
}

impl Collector {
    pub fn stickers_as_groups(&self) -> HashMap<String, HashMap<String, u8>> {
        let mut groups: HashMap<String, HashMap<String, u8>> = HashMap::new();
        for sticker in self.stickers.keys() {
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
                Some(g) => {g.insert(number, self.stickers.get(sticker).unwrap().len() as u8);},
                None => {
                    let mut group_map = HashMap::new();
                    group_map.insert(number, self.stickers.get(sticker).unwrap().len() as u8);
                    groups.insert(prefix, group_map);
                }
            }
        }
        groups
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_validate_sticker() {
        let good_team_sticker = validate_sticker("ARG1");
        assert!(good_team_sticker.is_some());
        assert!(good_team_sticker.unwrap() == "ARG1");

        assert!(validate_sticker("sdfsfsd").is_none());
        assert!(validate_sticker("ARG1231").is_none());
        
        let good_special_sticker = validate_sticker("FWC0");
        assert!(good_special_sticker.is_some());
        assert!(good_special_sticker.unwrap() == "FWC0");        
    }
    
    #[test]
    fn test_collector_as_groups() {
        let mut stickers: HashMap<String, Vec<u64>> = HashMap::new();
        stickers.insert(String::from("ARG1"), vec![1661975120, 1661975130]);
        stickers.insert(String::from("ARG2"), vec![1661975120]);
        stickers.insert(String::from("NED10"), vec![1661975120]);
        let c = Collector{
            user_id: 1,
            chat_id: 1,
            stickers
        };
        let groups = c.stickers_as_groups();
        assert_eq!(groups.get("ARG").unwrap().get("1").unwrap(), &2);
        assert_eq!(groups.get("ARG").unwrap().get("2").unwrap(), &1);
        assert_eq!(groups.get("NED").unwrap().get("10").unwrap(), &1);
        assert_eq!(groups.get("NED").unwrap().get("0").is_none(), true);
        assert_eq!(groups.get("KSA").is_none(), true);
    }
}