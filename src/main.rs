#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use core::time::Duration;
use eframe::egui;
use std::sync::mpsc;
use std::thread;

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
    .unwrap();
}

#[derive(Clone)]
enum State {
    // Clear the screen and prepare to fetch
    BeginFetch,
    // Spawn a thread and fetch in the background
    Fetching,
    // Fetch is complete, show the results
    DoneFetch { text: Option<String> },
}

struct MyApp {
    projects: Vec<String>,
    project: String,
    state: State,
    tx: mpsc::Sender<Option<String>>,
    rx: mpsc::Receiver<Option<String>>,
}

impl Default for MyApp {
    fn default() -> Self {
        let projects = projects();
        let project = get_first_project(projects.clone());
        let text = get_next(project.clone());
        let (tx, rx) = mpsc::channel();

        Self {
            state: State::DoneFetch { text },
            projects,
            project,
            tx,
            rx,
        }
    }
}

#[allow(clippy::collapsible_else_if)]
#[allow(clippy::collapsible_if)]
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label("The One Thing");
                ui.label(String::new());

                ui.vertical_centered(|ui| match self.state.clone() {
                    State::BeginFetch => {
                        ui.add(egui::Spinner::new());
                        spawn_complete_task(self.project.clone(), self.tx.clone());
                        self.state = State::Fetching;
                    }

                    State::Fetching => {
                        ui.add(egui::Spinner::new());

                        if let Ok(text) = self.rx.try_recv() {
                            self.state = State::DoneFetch { text };
                        }
                    }

                    State::DoneFetch { text } => {
                        if let Some(text) = text {
                            ui.heading(text);
                            ui.label(String::new());
                            if ui.button("Complete ✔").clicked() {
                                self.state = State::BeginFetch;
                            }
                            ui.label(String::new());
                            if ui.input(|i| i.key_pressed(egui::Key::C)) {
                                self.state = State::BeginFetch;
                            }
                        } else {
                            ui.heading(String::from("\nNo tasks remaining"));
                            ui.label(String::new());
                            ui.label(String::new());
                        };
                        if ui.button("Hide Project 🗙").clicked() {
                            hide(self.project.clone(), self);
                        }
                        if ui.input(|i| i.key_pressed(egui::Key::H)) {
                            hide(self.project.clone(), self);
                        }
                    }
                });
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
                            self.state = State::DoneFetch {
                                text: get_next(self.project.clone()),
                            };
                        }
                    }
                }
            });
        });
        ctx.request_repaint_after(Duration::new(0, 100));
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

fn get_next(project: String) -> Option<String> {
    match config::get_or_create(None) {
        Ok(config) => projects::next_item(config, &project).unwrap(),
        Err(_e) => None,
    }
}

fn spawn_complete_task(project: String, tx: mpsc::Sender<Option<String>>) {
    thread::spawn(|| complete(project, tx));
}

fn complete(project: String, tx: mpsc::Sender<Option<String>>) {
    match config::get_or_create(None) {
        Ok(config) => {
            request::complete_item(config).unwrap();
            match get_next(project) {
                Some(text) => tx.send(Some(text)),
                None => tx.send(None),
            }
        }
        Err(_e) => tx.send(None),
    }
    .unwrap();
}

fn hide(project: String, state: &mut MyApp) {
    let projects: Vec<String> = state
        .projects
        .clone()
        .into_iter()
        .filter(|s| s != &project)
        .collect();

    let project = get_first_project(projects.clone());

    state.projects = projects;
    state.project = project.clone();
    state.state = State::DoneFetch {
        text: get_next(project),
    };
}

fn get_first_project(projects: Vec<String>) -> String {
    projects
        .first()
        .map(|f| f.to_string())
        .unwrap_or_else(|| String::from("No projects found"))
}
