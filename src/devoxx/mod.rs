use chrono::Weekday;

use model::Talk;

pub mod model;

const DEVOXX_HOST: &str = "dvbe19.cfp.dev";

pub fn read_talks() -> Result<Vec<String>, failure::Error> {
    unimplemented!()
}

pub fn get_talks_by_weekday(day: &Weekday, _offline: bool) -> Result<Vec<Talk>, failure::Error> {
    let day = match day {
        Weekday::Mon => "monday",
        Weekday::Tue => "tuesday",
        Weekday::Wed => "wednesday",
        Weekday::Thu => "thursday",
        Weekday::Fri => "friday",
        _ => "monday"
    };

    // TODO use offline flag

    get_talks_by_day_file(day)
}

pub fn get_talks() -> Result<Vec<Talk>, failure::Error> {
    unimplemented!()
}


pub fn get_talks_by_day_file(_day: &str) -> Result<Vec<Talk>, failure::Error> {
    unimplemented!()
}

pub fn get_talks_by_day_api(_day: &str) -> Result<Vec<Talk>, failure::Error> {
    unimplemented!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_talks() {
        let items = read_talks();
        assert_eq!(items.is_ok(), true, "{:?}", items.err());
        if let Ok(talks) = items {
            assert_eq!(talks.len(), 12);
            assert_eq!(&talks.get(0).unwrap()[..12], "Monty Python");
        }
    }

    #[test] #[ignore]
    fn test_structured_data() {
        let items = get_talks();
        assert_eq!(items.is_ok(), true, "{:?}", items.err());
        if let Ok(items) = items {
            assert_eq!(items.len(), 12);
            assert_eq!(items.get(0).unwrap().talk_title.as_ref().unwrap(), "Monty Python meets the Cloud of Doom");
        }
    }

    #[test] #[ignore]
    fn test_get_talks() {
        verify_get_talks(get_talks_by_day_file("monday"));
    }

    #[test] #[ignore]
    fn test_get_talks_api() {
        verify_get_talks(get_talks_by_day_api("monday"));
    }

    fn verify_get_talks(result: Result<Vec<Talk>, failure::Error>) {
        assert_eq!(result.is_ok(), true, "{:?}", result.err());
        let mut found_rust_lab = false;
        let expected_title = Some(String::from("Rust for Java Developers"));
        if let Ok(talks) = result {
            for talk in &talks {
                match talk {
                    Talk { talk_title: title, .. } if title == &expected_title => found_rust_lab = true,
                    _ => ()
                };
            }
        }
        assert_eq!(found_rust_lab, true);
    }
}