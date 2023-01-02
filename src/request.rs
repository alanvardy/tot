use reqwest::blocking::Client;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use serde_json::json;
use uuid::Uuid;

use crate::config::Config;
use crate::items;
use crate::items::Item;

#[cfg(test)]
use mockito;

// TODOIST URLS
const PROJECT_DATA_URL: &str = "/sync/v9/projects/get_data";
const SYNC_URL: &str = "/sync/v9/sync";

const FAKE_UUID: &str = "42963283-2bab-4b1f-bad2-278ef2b6ba2c";

/// Get a vector of all items for a project
pub fn items_for_project(config: &Config, project_id: &str) -> Result<Vec<Item>, String> {
    let url = String::from(PROJECT_DATA_URL);
    let body = json!({ "project_id": project_id });
    let json = post_todoist_sync(config.token.clone(), url, body)?;
    items::json_to_items(json)
}

/// Complete the last item returned by "next item"
pub fn complete_item(config: Config) -> Result<String, String> {
    let body = json!({"commands": [{"type": "item_close", "uuid": new_uuid(), "temp_id": new_uuid(), "args": {"id": config.next_id}}]});
    let url = String::from(SYNC_URL);

    post_todoist_sync(config.token.clone(), url, body)?;

    if !cfg!(test) {
        config.clear_next_id().save()?;
    }

    // Does not pass back an item
    Ok(String::from("✓"))
}

/// Post to Todoist via sync API
fn post_todoist_sync(
    token: String,
    url: String,
    body: serde_json::Value,
) -> Result<String, String> {
    #[cfg(not(test))]
    let todoist_url: &str = "https://api.todoist.com";

    #[cfg(test)]
    let todoist_url: &str = &mockito::server_url();

    let request_url = format!("{}{}", todoist_url, url);

    let response = Client::new()
        .post(request_url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .json(&body)
        .send()
        .or(Err("Did not get response from server"))?;

    if response.status().is_success() {
        Ok(response.text().or(Err("Could not read response text"))?)
    } else {
        Err(format!("Error: {:#?}", response.text()))
    }
}

/// Create a new UUID, required for Todoist API
fn new_uuid() -> String {
    if cfg!(test) {
        String::from(FAKE_UUID)
    } else {
        Uuid::new_v4().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_complete_an_item() {
        let _m = mockito::mock("POST", "/sync/v9/sync")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&test::responses::sync())
            .create();

        let config = Config::new("12341234")
            .unwrap()
            .set_next_id(String::from("112233"));
        let response = complete_item(config);
        assert_eq!(response, Ok(String::from("✓")));
    }
}
