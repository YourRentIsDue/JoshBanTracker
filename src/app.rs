use eframe::{egui, epi};
use chrono::prelude::*;
use std::fs::{File, OpenOptions, self};
use std::io::{Write, BufReader, BufRead, Error, stdout};
use serde::{Deserialize, Serialize};




#[derive(Debug, Deserialize, Serialize)]
pub struct Ban {
    days: String,
    hours: String,
    mins: String,
    start: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    username: String,
    is_banned: bool,
    ban: Option<Ban>,
}



pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    value: f32,
    popup: bool,
    user: User,
    ban: Ban,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            popup: false,
            user: User {
                username: "".to_string(),
                is_banned: false,
                ban: None,
            },
            ban: Ban { days: "0".to_string(), hours: "0".to_string(), mins: "0".to_string(), start: "".to_string() }
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


    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    
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

        
        if self.popup {
            window.open(&mut self.popup)
                .default_height(20.0)
                .resize(|r| r.max_size([100.0,40.0]))
                .show(ctx, |ui| {

                    ui.with_layout(egui::Layout::left_to_right(), |ui| { //username and banned checkbox
                        ui.add_sized([100.0, 20.0], egui::TextEdit::singleline(&mut self.user.username).hint_text("Username"));
                        ui.checkbox(&mut self.user.is_banned, "Is this account currently banned?");           
                    });

                    if self.user.is_banned {

                        let mut temp_ban = Ban {
                            days: "0".to_string(),
                            hours: "0".to_string(),
                            mins: "0".to_string(),
                            start: chrono::Local::now().to_rfc2822(),
                        };
                            
                        ui.with_layout(egui::Layout::left_to_right(), |ui| {
                            ui.separator();
                            ui.add_sized([100.0, 20.0], egui::TextEdit::singleline(&mut self.ban.days).hint_text("Days"));
                            ui.add_sized([100.0, 20.0], egui::TextEdit::singleline(&mut self.ban.hours).hint_text("Hours"));
                            ui.add_sized([100.0, 20.0], egui::TextEdit::singleline(&mut self.ban.mins).hint_text("Mins"));
                            
                            temp_ban.days = "".to_string();
                            temp_ban.days += &self.ban.days;
                            temp_ban.hours = "".to_string();
                            temp_ban.hours += &self.ban.hours;
                            temp_ban.mins = "".to_string();
                            temp_ban.mins += &self.ban.mins;

                            self.user.ban = Some(temp_ban);
                        });                           
                    }

                    ui.separator();

                    ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                        if ui.add_sized([100.0, 40.0], egui::Button::new("Save")).clicked() {

                            if let Err (_err) = write_file(&self.user){
                                ui.label("There was an error writing the file");
                            }            
                        }
                    });                   
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui|{
                ui.separator();
                let mut users:Vec<User> = Vec::new();
                if let Err (_err) = read_file(&mut users){
                    ui.label("there was an error reading the file");         
                }
                display_users(&mut users, ui);
            }); 
        });
    }
}

pub fn write_file(user: &User) -> std::result::Result<(), Box<dyn std::error::Error>>{
    let mut output_json = serde_json::to_string(user)?;
    output_json += "\n";
    
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("smurfs.json")?;
    file.write_all(output_json.as_bytes())?;
    file.flush()?;
    
    Ok(())
}

pub fn read_file(users: &mut Vec<User>) -> std::result::Result<(), Box<dyn std::error::Error>> {
    
    let file = File::open("smurfs.json")?;
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        users.push(serde_json::from_str(&line)?);
    }
    Ok(())
}
pub fn display_users(users: &mut Vec<User>, ui:&mut eframe::egui::Ui) {
    for user in users {
        ui.allocate_ui_with_layout( 
            egui::vec2(500.0, 10.0),
            egui::Layout::left_to_right(),
            |ui| {
            
            ui.add_sized([100.0, 10.0], egui::Label::new("Username: ".to_owned() + &user.username));
            ui.separator();
            
            if user.is_banned {
                ui.add_sized([100.0, 10.0], egui::Label::new("Banned: Yes"));
                ui.separator();
                let days:i32 = user.ban.as_ref().unwrap().days.parse().unwrap();
                let hours:i32 = user.ban.as_ref().unwrap().hours.parse().unwrap();
                let mins:i32 = user.ban.as_ref().unwrap().mins.parse().unwrap();

                let dur_min = days*24*60 + hours*60 + mins;

                let duration = chrono::Duration::minutes(dur_min.into());
                
                let today_millis = chrono::Local::now().timestamp_millis();

                let start_milis = chrono::DateTime::parse_from_rfc2822(&user.ban.as_ref()
                    .unwrap().start).unwrap().timestamp_millis();

                let time_left = start_milis + duration.num_milliseconds() - today_millis;

                let mut secs_left:i64 = time_left/1000;

                let mut mins_left:i64 = secs_left/60;

                secs_left = secs_left - mins_left*60;

                let mut hours_left:i64 = mins_left/60;

                mins_left = mins_left - hours_left*60;

                let days_left:i64 = hours_left/24;

                hours_left = hours_left - days_left*24;

                if time_left > 0 {
                    ui.add_sized([100.0, 10.0], egui::Label::new("Days left: ".to_owned() + &days_left.to_string()));
                    ui.separator();
                    ui.add_sized([100.0, 10.0], egui::Label::new("hours left: ".to_owned() + &hours_left.to_string()));
                    ui.separator();
                    ui.add_sized([100.0, 10.0], egui::Label::new("mins left: ".to_owned() + &mins_left.to_string()));
                    ui.separator();
                    ui.add_sized([100.0, 10.0], egui::Label::new("secs left: ".to_owned() + &secs_left.to_string()));
                    
                } 
            }
            else {
                ui.add_sized([100.0, 10.0], egui::Label::new("Banned: No"));
            }
        });
        ui.separator();
    }
}

