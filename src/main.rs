use egui::{CentralPanel, TextEdit};
use eframe::{egui, App, NativeOptions};
use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug, Clone)]
struct CommandData {
    name: String,
    description: String,
    usage: String,
    language: String,
}

fn load_commands(filepath: &str) -> Result<Vec<CommandData>, Box<dyn Error>> {
    let path = Path::new(filepath);
    let contents = fs::read_to_string(path)?;
    let commands: Vec<CommandData> = serde_json::from_str(&contents)?;
    Ok(commands)
}

struct ClickrApp {
    commands: Vec<CommandData>,
    search_query: String,
    filtered_commands: Vec<CommandData>,
    selected_language: String,
}

impl ClickrApp {
    fn new() -> Self {
        let commands = load_commands("data/commands.json").unwrap();
        Self {
            commands: commands.clone(),
            search_query: String::new(),
            filtered_commands: commands.clone(),
            selected_language: String::new(),
        }
    }

    fn filter_commands(&mut self) {
        self.filtered_commands = self.commands.clone();
        if !self.search_query.is_empty() {
            self.filtered_commands.retain(|cmd| {
                cmd.name.to_lowercase().contains(&self.search_query.to_lowercase())
                    || cmd.description.to_lowercase().contains(&self.search_query.to_lowercase())
                    || cmd.language.to_lowercase().contains(&self.search_query.to_lowercase())
            });
        }
        if !self.selected_language.is_empty() {
            self.filtered_commands.retain(|cmd| {
                cmd.language.to_lowercase() == self.selected_language.to_lowercase()
            });
        }
    }
}

impl App for ClickrApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Clickr GUI");

            ui.horizontal(|ui| {
                ui.label("Search:");
                let response = ui.add(TextEdit::singleline(&mut self.search_query));
                if response.changed() {
                    self.filter_commands();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Language Filter:");
                let response = ui.add(TextEdit::singleline(&mut self.selected_language));
                if response.changed() {
                    self.filter_commands();
                }
            });

            ui.separator();

            for cmd in &self.filtered_commands {
                ui.label(format!("Name: {}", cmd.name));
                ui.label(format!("Description: {}", cmd.description));
                ui.label(format!("Usage: {}", cmd.usage));
                ui.separator();
            }
        });
    }
}

fn main() {
    let native_options = NativeOptions::default();
    let _ = eframe::run_native("Clickr GUI", native_options, Box::new(|_| Box::new(ClickrApp::new())));
}