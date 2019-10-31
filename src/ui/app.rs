use chrono::Weekday;
use crate::devoxx::model::Talk;

#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Filtered,
    Search
}

#[allow(dead_code)]
pub struct App {
    pub day: Weekday,
    pub talks: Vec<Talk>,
    pub selected: Option<usize>,
    pub search_text: String,
    pub mode: Mode,
    pub should_quit: bool,
    pub offline: bool
}

impl App {
    pub fn new(offline: bool) -> Result<App, failure::Error> {
        // TODO
        let talks = vec![
            Talk {
                talk_title: Some("TODO: load talks".into()),
                ..Talk::default()
            }
        ];

        Ok(App {
            day : Weekday::Mon,
            talks,
            search_text: String::new(),
            selected: Some(0),
            mode : Mode::Normal,
            should_quit: false,
            offline
        })
    }

    pub fn advance(&mut self) {
        //tick
    }

    pub fn get_selected(&self) -> Option<&Talk> {
        self.talks()
            .get(self.selected.unwrap_or_default())
            .map(|x| *x)
    }

    pub fn talks(&self) -> Vec<&Talk> {
        self.talks
            .iter()
            // TODO
            .collect()
    }

    pub fn talk_titles(&self) -> Vec<&str> {
        self.talks()
            .iter()
            // TODO
            .map(|talk| talk.talk_title.as_ref().map_or("", String::as_str))
            .collect()
    }

    // TODO a filter function for search talk titles

    pub fn next_tab(&mut self) -> Result<(), failure::Error> {
        let new_day = if self.day == Weekday::Fri { Weekday::Mon } else { self.day.succ() };
        self.set_current_day(new_day)
    }

    pub fn previous_tab(&mut self) -> Result<(), failure::Error> {
        let new_day = if self.day == Weekday::Mon { Weekday::Fri } else { self.day.pred() };
        self.set_current_day(new_day)
    }

    fn set_current_day(&mut self, day: Weekday) -> Result<(), failure::Error>{
        self.day = day;
        // TODO reload the talks
        self.selected = Some(0);
        Ok(())
    }

    pub fn next_talk(&mut self) {
        self.selected = if let Some(selected) = self.selected {
            if selected >= self.talks().len() - 1 {
                Some(0)
            } else {
                Some(selected + 1)
            }
        } else {
            Some(0)
        }
    }

    pub fn previous_talk(&mut self) {
        self.selected = if let Some(selected) = self.selected {
            if selected > 0 {
                Some(selected - 1)
            } else {
                Some(self.talks().len() - 1)
            }
        } else {
            Some(0)
        }
    }
}