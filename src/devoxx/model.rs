use chrono::{DateTime, Utc};
use std::time::UNIX_EPOCH;

pub struct Talk {
    pub talk_title: Option<String>,
    pub talk_description: Option<String>,
    pub tags: Option<Vec<Tag>>,
    pub room_name: String,
    pub from_date: DateTime<Utc>,
    pub to_date: DateTime<Utc>,
    pub session_type_name: Option<String>,
    pub speakers: Option<Vec<Speaker>>,
    pub timezone: String
}

pub struct Speaker {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub company: Option<String>
}

pub struct Tag {
    pub name: String
}

impl Talk {
    pub fn get_title(&self) -> &str {
        "TODO"
    }

    pub fn local_from_date(&self) -> String {
        "TODO".into()
    }

    pub fn local_to_date(&self) -> String {
        "TODO".into()
    }

    pub fn get_description(&self) -> &str {
        "TODO"
    }

    pub fn speaker_names(&self) -> String {
        "TODO".into()
    }
}

impl Default for Talk {
    fn default() -> Self {
        Self {
            talk_title: None,
            talk_description: None,
            tags: None,
            room_name: "".to_string(),
            from_date: DateTime::<Utc>::from(UNIX_EPOCH),
            to_date: DateTime::<Utc>::from(UNIX_EPOCH),
            session_type_name: None,
            speakers: None,
            timezone: "".to_string()
        }
    }
}
