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
#[cfg(test)]
pub mod responses {

    pub fn sync() -> String {
        String::from(
            "{
    \"creator_id\": \"2671355\",
    \"created_at\": \"2019-12-11T22:36:50.000000Z\",
    \"assignee_id\": \"2671362\",
    \"assigner_id\": \"2671355\",
    \"comment_count\": 10,
    \"is_completed\": false,
    \"content\": \"Buy Coffee\",
    \"description\": \"\",
    \"due\": {
        \"date\": \"2016-09-01\",
        \"is_recurring\": false,
        \"datetime\": \"2016-09-01T12:00:00.000000Z\",
        \"string\": \"tomorrow at 12\",
        \"timezone\": \"Europe/Moscow\"
    },
    \"id\": \"2995104339\",
    \"labels\": [\"Food\", \"Shopping\"],
    \"order\": 1,
    \"priority\": 1,
    \"project_id\": \"2203306141\",
    \"section_id\": \"7025\",
    \"parent_id\": \"2995104589\",
    \"url\": \"https://todoist.com/showTask?id=2995104339\"
}",
        )
    }
}
