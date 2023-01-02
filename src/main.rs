#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
mod config;
mod items;
mod projects;
mod request;
mod time;

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
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { text: get_next() }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("The One Thing");
            ui.label(self.text.clone());

            if ui.button("Complete").clicked() {
                self.text = complete();
            }
        });
    }
}

fn get_next() -> String {
    match config::get_or_create(None) {
        Ok(config) => projects::next_item(config, "home")
            .unwrap_or_else(|_| "Could not get next item".to_string()),
        Err(e) => format!("Could not load config: {}", e),
    }
}

fn complete() -> String {
    match config::get_or_create(None) {
        Ok(config) => {
            request::complete_item(config).unwrap();
            get_next()
        }
        Err(e) => format!("Could not load config: {}", e),
    }
}
