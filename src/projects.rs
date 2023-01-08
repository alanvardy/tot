use crate::config::Config;
use crate::{items, projects, request};

pub fn project_id(config: &Config, project_name: &str) -> Result<String, String> {
    let project_id = config
        .projects
        .get(project_name)
        .ok_or(format!(
            "Project {project_name} not found, please add it to config"
        ))?
        .to_string();

    Ok(project_id)
}

/// Get the next item by priority and save its id to config
pub fn next_item(config: Config, project_name: &str) -> Result<Option<String>, String> {
    let project_id = projects::project_id(&config, project_name)?;
    let items = request::items_for_project(&config, &project_id)?;
    let filtered_items = items::filter_not_in_future(items, &config)?;
    let maybe_item = items::sort_by_value(filtered_items, &config)
        .first()
        .map(|item| item.to_owned());

    match maybe_item {
        Some(item) => {
            config.set_next_id(item.id.clone()).save()?;
            Ok(Some(item.fmt(&config)))
        }
        None => Ok(None),
    }
}
