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
                ban: Some(Ban {
                    days: "0".to_string(),
                    hours: "0".to_string(),
                    mins: "0".to_string(),
                }),
            }
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
                            
                        ui.with_layout(egui::Layout::left_to_right(), |ui| {
                            ui.separator();
                            ui.add_sized([100.0, 20.0], egui::TextEdit::singleline(self.user.ban.as_ref().unwrap().days).hint_text("Days"));
                            ui.add_sized([100.0, 20.0], egui::TextEdit::singleline(&mut self.user.ban.unwrap().hours).hint_text("Hours"));
                            ui.add_sized([100.0, 20.0], egui::TextEdit::singleline(&mut self.user.ban.unwrap().mins).hint_text("Mins"));
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
                //if let Err (_err) = read_file(ui){
                //    ui.label("there was an error reading the file");
                //}
            });
            
        });
    }
}

pub fn write_file(user: &User) -> std::result::Result<(), Box<dyn std::error::Error>>{

    let output_JSON = serde_json::to_string(user)?;
    
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("smurfs.json")?;
    file.write_all(output_JSON.as_bytes())?;
    file.flush()?;
    


    Ok(())
}

pub fn read_file(ui: &mut eframe::egui::Ui) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let file = File::open("smurfs.txt")?;
    let reader = BufReader::new(file);

    let mut users:Vec<User> = Vec::new();
    let mut user_data:Vec<String> = Vec::new();
    let mut user_2d:Vec<Vec<String>> = Vec::new(); 

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if line == "--ENDSMURF--" {
            user_2d.push(user_data);
            user_data = Vec::new();
        }
        else {
            user_data.push(line);
        }
    }

    for i in 0..user_2d.len() {
        let mut temp = User {
            username: "".to_string(),
            is_banned: true,
            ban: None
        };

        let mut temp_ban = Ban {
            days: "".to_string(),
            hours: "".to_string(),
            mins: "".to_string() 
        };

        for j in 0..user_2d[i].len(){
            if user_2d[i][j].contains("username") {
                temp.username = user_2d[i][j].split("= ").collect::<Vec<&str>>()[1].to_string();
            }

            if user_2d[i][j].contains("is banned") {
                if user_2d[i][j].split("= ").collect::<Vec<&str>>()[1] == "true" {
                    temp.is_banned = true;
                }
                else {
                    temp.is_banned = false;
                }
            }

            if user_2d[i][j].contains("days") {
                temp_ban.days = user_2d[i][j].split("= ").collect::<Vec<&str>>()[1].to_string();
                
            }
            if user_2d[i][j].contains("hours") {
                temp_ban.hours = user_2d[i][j].split("= ").collect::<Vec<&str>>()[1].to_string();
            }
            if user_2d[i][j].contains("mins") {
                temp_ban.mins = user_2d[i][j].split("= ").collect::<Vec<&str>>()[1].to_string();
                
            }
        }

        if (temp_ban.days != ""){
            temp.ban = Some(temp_ban);
        }
        users.push(temp);
    }

    for i in 0..users.len(){
        
        
        ui.allocate_ui_with_layout( 
            egui::vec2(400.0, 50.0),
            egui::Layout::left_to_right(),
            |ui| {

                ui.add_sized([100.0, 10.0], egui::Label::new("Username: ".to_owned() + &users[i].username));
                if users[i].is_banned{
                    ui.add_sized([100.0, 10.0], egui::Label::new("Banned: Yes"));
                    
                    let end_day:u32 = Local::now().day() + users[i].ban.as_ref().unwrap().days.parse::<u32>().unwrap();
                    let end_month = Local::now().month();
                    //if ()

                    println!("{}", end_day);
                    //ui.add_sized([100.0, 10.0], egui::Label::new("Banned: Yes"));
                }
                else {
                    ui.add_sized([100.0, 10.0], egui::Label::new("Banned: No"));
                }

                
            });
            
        ui.separator();
        
        
    }
        

    Ok(())
}

