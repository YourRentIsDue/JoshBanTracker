use eframe::{egui, epi};
use chrono::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader, BufRead, Error, stdout};





/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state

pub struct Ban {
    days: String,
    hours: String,
    mins: String,
}

pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    value: f32,
    popup: bool,
    username: String,
    is_banned: bool,
    ban: Ban,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            popup: false,
            username: "".to_owned(),
            is_banned: false,
            ban: Ban {
                days: "".to_string(),
                hours: "".to_string(),
                mins: "".to_string(),
            },
        }
    }
}




impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "eframe template"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }


    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let window = egui::Window::new("Add");

        egui::SidePanel::left("add_acc_panel")
            .show(ctx, |ui| {
                ui.add_space(3.0);
                if ui.add_sized([100.0, 40.0], egui::Button::new("Add account")).clicked() {
                    self.popup = true;
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.popup {
                window.open(&mut self.popup)
                    .default_height(20.0)
                    .resize(|r| r.max_size([100.0,40.0]))
                    .show(ctx, |ui| {
                        ui.with_layout(egui::Layout::left_to_right(), |ui| { //username and banned checkbox
                            ui.add_sized([100.0, 20.0], egui::TextEdit::singleline(&mut self.username).hint_text("Username"));
                            ui.checkbox(&mut self.is_banned, "Is this account currently banned?");
                            
                        });

                        if self.is_banned {
                            
                            ui.with_layout(egui::Layout::left_to_right(), |ui| {
                                ui.separator();
                                ui.add_sized([100.0, 20.0], egui::TextEdit::singleline(&mut self.ban.days).hint_text("Days"));
                                ui.add_sized([100.0, 20.0], egui::TextEdit::singleline(&mut self.ban.hours).hint_text("Hours"));
                                ui.add_sized([100.0, 20.0], egui::TextEdit::singleline(&mut self.ban.mins).hint_text("Mins"));
                            });
                            
                        }

                        ui.separator();

                        ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                            if ui.add_sized([100.0, 40.0], egui::Button::new("Save")).clicked() {
                                let mut output:String = "username = ".to_string();
                                output += &self.username;
                                output += "\n";
                                output += "is banned = ";

                                if self.is_banned {
                                    output += "true";
                                    output += "\n";
                                    output += "days = ";
                                    output += &self.ban.days;
                                    output += "\n";
                                    output += "hours = ";
                                    output += &self.ban.hours;
                                    output += "\n";
                                    output += "mins = ";
                                    output += &self.ban.mins;
                                }
                                else {
                                    output += "false";
                                }
                                output += "\n";
                                output += "--ENDSMURF--";
                                output += "\n";

                                println!("{}", output);
                                

                                if let Err (_err) = write_file(output){
                                    ui.label("There was an error writing the file");
                                }            
                            }
                        });                   
                });
            }

            
        });
    }
}

pub fn write_file(output: String) -> std::result::Result<(), Box<dyn std::error::Error>>{
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("smurfs.txt")?;
    file.write_all(output.as_bytes())?;
    file.flush()?;
    Ok(())
}

