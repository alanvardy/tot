#[cfg(test)]
pub mod helpers {
    use crate::config;
    use crate::config::Config;
    use crate::items::{DateInfo, Item};
    use std::collections::HashMap;

    pub fn item_fixture() -> Item {
        Item {
            id: String::from("222"),
            content: String::from("Get gifts for the twins"),
            checked: false,
            description: String::from(""),
            due: Some(DateInfo {
                date: String::from("2061-11-13"),
                is_recurring: false,
                timezone: Some(String::from("America/Los_Angeles")),
            }),
            priority: 3,
            is_deleted: false,
        }
    }

    pub fn config_fixture() -> Config {
        Config {
            token: String::from("alreadycreated"),
            projects: HashMap::new(),
            path: config::generate_path().unwrap(),
            next_id: None,
            last_version_check: None,
            timezone: Some(String::from("US/Pacific")),
        }
    }
}
