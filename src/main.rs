#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
mod config;
mod items;
mod projects;
mod request;
mod test;
mod time;

use config::Config;

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "The One Thing",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    text: String,
    projects: Vec<String>,
    project: String,
}

impl Default for MyApp {
    fn default() -> Self {
        let projects = projects();
        let project = projects
            .first()
            .map(|f| f.to_string())
            .unwrap_or_else(|| String::from("No projects found"));

        Self {
            text: get_next(project.clone()),
            projects,
            project,
        }
    }
}

#[allow(clippy::collapsible_else_if)]
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label("The One Thing");
                ui.label(String::new());
                ui.heading(self.text.clone());
                ui.label(String::new());
                if ui.button("Complete ✔").clicked() {
                    self.text = complete(self.project.clone());
                }
            });

            ui.with_layout(egui::Layout::left_to_right(egui::Align::BOTTOM), |ui| {
                for project in self.projects.iter() {
                    if *project.clone() == self.project {
                        if ui.add_enabled(false, egui::Button::new(project)).clicked() {
                            unreachable!();
                        }
                    } else {
                        if ui.button(project).clicked() {
                            self.project = project.to_string();
                            self.text = get_next(self.project.clone());
                        }
                    }
                }
            });
        });
    }
}

fn projects() -> Vec<String> {
    match config::get_or_create(None) {
        Ok(Config { projects, .. }) => {
            let mut projects = projects
                .keys()
                .map(|k| k.to_owned())
                .collect::<Vec<String>>();

            projects.sort();
            projects
        }
        Err(e) => vec![e],
    }
}

fn get_next(project: String) -> String {
    match config::get_or_create(None) {
        Ok(config) => projects::next_item(config, &project)
            .unwrap_or_else(|_| "Could not get next item".to_string()),
        Err(e) => format!("Could not load config: {}", e),
    }
}

fn complete(project: String) -> String {
    match config::get_or_create(None) {
        Ok(config) => {
            request::complete_item(config).unwrap();
            get_next(project)
        }
        Err(e) => format!("Could not load config: {}", e),
    }
}
