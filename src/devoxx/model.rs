use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use std::time::UNIX_EPOCH;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Speaker {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub company: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct Tag {
    pub name: String
}

impl Talk {
    pub fn get_title(&self) -> &str {
        match &self.talk_title {
            Some(title) => &title,
            _ => match &self.session_type_name {
                Some(name) => &name,
                _ => ""
            }
        }
    }

    pub fn local_from_date(&self) -> String {
        let tz: chrono_tz::Tz = self.timezone.parse().expect("Unknown timezone");
        self.from_date.with_timezone(&tz).to_rfc2822()
    }

    pub fn local_to_date(&self) -> String {
        let tz: chrono_tz::Tz = self.timezone.parse().expect("Unknown timezone");
        self.to_date.with_timezone(&tz).to_rfc2822()
    }

    pub fn get_description(&self) -> &str {
        match &self.talk_description {
            Some(description) => &description,
            _ => ""
        }
    }

    pub fn speaker_names(&self) -> String {
        self.speakers.as_ref()
            .map(|speakers| speakers.iter()
                .map(|speaker| format!("{} {}", speaker.first_name, speaker.last_name))
                .collect::<Vec<String>>()
                .join(", "))
            .unwrap_or(String::new())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_talk_title() {
        let mut talk = Talk::default();
        talk.talk_title = Some(String::from("some title"));

        assert_eq!(talk.get_title(), "some title");

        let mut talk = Talk::default();
        talk.session_type_name = Some(String::from("Session name"));

        assert_eq!(talk.get_title(), "Session name");
    }

    #[test]
    fn test_speaker_names() {
        let mut talk = Talk::default();
        let speaker1 = Speaker {
            id: 1,
            first_name: "First".to_string(),
            last_name: "Speaker".to_string(),
            company: Some("Acme Inc".to_string())
        };
        let speaker2 = Speaker {
            id: 2,
            first_name: "Second".to_string(),
            last_name: "Speaker".to_string(),
            company: Some("Acme Inc".to_string())
        };

        talk.speakers = Some(vec![speaker1, speaker2]);

        assert_eq!(talk.speaker_names(), "First Speaker, Second Speaker".to_string());
    }
}