use crate::config::Config;
use crate::{items, projects, request};

pub fn project_id(config: &Config, project_name: &str) -> Result<String, String> {
    let project_id = config
        .projects
        .get(project_name)
        .ok_or(format!(
            "Project {} not found, please add it to config",
            project_name
        ))?
        .to_string();

    Ok(project_id)
}

/// Get the next item by priority and save its id to config
pub fn next_item(config: Config, project_name: &str) -> Result<String, String> {
    let project_id = projects::project_id(&config, project_name)?;
    let items = request::items_for_project(&config, &project_id)?;
    let filtered_items = items::filter_not_in_future(items, &config)?;
    let maybe_item = items::sort_by_value(filtered_items, &config)
        .first()
        .map(|item| item.to_owned());

    match maybe_item {
        Some(item) => {
            config.set_next_id(item.id.clone()).save()?;
            Ok(item.fmt(&config))
        }
        None => Ok(String::from("No items on list")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;
    use mockito;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_list_projects() {
        let config = Config::new("123123")
            .unwrap()
            .add_project(String::from("first"), 1)
            .add_project(String::from("second"), 2);

        let str = if test::helpers::supports_coloured_output() {
            "\u{1b}[32mProjects\u{1b}[0m\n - first\n - second"
        } else {
            "Projects\n - first\n - second"
        };

        assert_eq!(list(config), Ok(String::from(str)));
    }

    #[test]
    fn should_display_scheduled_items() {
        let _m = mockito::mock("POST", "/sync/v9/projects/get_data")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&test::responses::items())
            .create();

        let config = Config::new("12341234")
            .unwrap()
            .add_project(String::from("good"), 1);

        let config_with_timezone = Config {
            timezone: Some(String::from("US/Pacific")),
            ..config
        };

        assert_eq!(
            scheduled_items(&config_with_timezone, "test"),
            Err(String::from(
                "Project test not found, please add it to config"
            ))
        );

        let str = if test::helpers::supports_coloured_output() {
            "\u{1b}[32mSchedule for good\u{1b}[0m\n\n\u{1b}[33mPut out recycling\u{1b}[0m\nDue: 15:59 ↻"
        } else {
            "Schedule for good\n\nPut out recycling\nDue: 15:59 ↻"
        };
        assert_eq!(
            scheduled_items(&config_with_timezone, "good"),
            Ok(String::from(str))
        );
    }
}
