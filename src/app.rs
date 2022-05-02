use eframe::{egui, epi};
use chrono::prelude::*;
use std::fs::{File, OpenOptions, self};
use std::io::{Write, BufReader, BufRead, Error, stdout};
use serde::{Deserialize, Serialize};




#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Ban {
    days: String,
    hours: String,
    mins: String,
    start: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
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
    edit_popup: bool,
    user: User,
    ban: Ban,
    users: Vec<User>,
    selected: usize,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            popup: false,
            edit_popup: false,
            user: User {
                username: "".to_string(),
                is_banned: false,
                ban: None,
            },
            ban: Ban { days: "0".to_string(), hours: "0".to_string(), mins: "0".to_string(), start: "".to_string() },
            users: Vec::new(),
            selected: 0,
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
        read_file(&mut self.users);

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
                ui.add_space(3.0);
                edit_users(&mut self.users, ctx, ui, &mut self.edit_popup, &mut self.selected);
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
                            self.users.push(self.user.clone());
                            if let Err (_err) = write_file(&self.user, true){
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
                let mut edited = false;
                if let Err (_err) = read_file(&mut users){
                    ui.label("there was an error reading the file");         
                }
                let edited = display_users(&mut users, ui);
                
                if (edited){
                    println!("runs");
                    if let Err (_err) = write_file(&users[0], false) {
                        ui.label("There was an error writing the file");
                    }
                
                
                    for i in 1..users.len() {
                        if let Err (_err) = write_file(&users[i], true){
                            ui.label("There was an error writing the file");
                        }
                    }

                }
                

            }); 
        });
    }
}

pub fn write_file(user: &User, append: bool) -> std::result::Result<(), Box<dyn std::error::Error>>{

    let mut output_json = serde_json::to_string(user)?;
    //println!("{} ", output_json);
    output_json += "\n";

    if (append){      
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("smurfs.json")?;
        file.write_all(output_json.as_bytes())?;
        file.flush()?;
    }
    else {
        let mut file = fs::File::create("smurfs.json")?;

        file.write_all(output_json.as_bytes())?;
        file.flush()?;
    }

    
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
pub fn display_users(users: &mut Vec<User>, ui:&mut eframe::egui::Ui) -> bool{

    let mut edited = false;

    for user in users {
        ui.allocate_ui_with_layout( 
            egui::vec2(200.0, 10.0),
            egui::Layout::left_to_right(),
            |ui| {
            
            ui.add_sized([100.0, 10.0], egui::Label::new("Username: ".to_owned() + &user.username));

            
        });

        ui.add_space(20.0);
    

        ui.allocate_ui_with_layout( 
            egui::vec2(500.0, 10.0),
            egui::Layout::left_to_right(),
            |ui| {
            
                
            if user.is_banned {

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

                    ui.add_sized([100.0, 10.0], egui::Label::new("Banned: Yes"));
                    ui.separator();
                    ui.add_sized([100.0, 10.0], egui::Label::new("Days left: ".to_owned() + &days_left.to_string()));
                    ui.separator();
                    ui.add_sized([100.0, 10.0], egui::Label::new("hours left: ".to_owned() + &hours_left.to_string()));
                    ui.separator();
                    ui.add_sized([100.0, 10.0], egui::Label::new("mins left: ".to_owned() + &mins_left.to_string()));
                    ui.separator();
                    ui.add_sized([100.0, 10.0], egui::Label::new("secs left: ".to_owned() + &secs_left.to_string()));
                    ui.separator();
                    
                    
                } 
                else {
                    ui.add_sized([100.0, 10.0], egui::Label::new("Banned: No"));
                    user.is_banned = false;
                    user.ban = None;
                    edited = true;
                }
            }
            else {
                ui.add_sized([100.0, 10.0], egui::Label::new("Banned: No"));
            }  
        });
        
    
        ui.separator();
        
    }
    return edited;
}  

pub fn edit_users (users: &mut Vec<User>, ctx: &egui::Context, ui: &mut egui::Ui, popup: &mut bool, selected: &mut usize) {

    if ui.add_sized([100.0, 40.0], egui::Button::new("Edit Accounts")).clicked() {
        *popup = true;
    }

    

    if *popup {
        egui::Window::new("Edit").open(popup)
        .default_height(20.0)
        .resize(|r| r.max_size([150.0,40.0]))
        .show(ctx, |ui| {

            let mut names = Vec::new();

            for user in users.clone() {
                names.push(user.username.clone());
            }

            egui::ComboBox::from_label("Is being edited").show_index(
                ui,
                selected,
                names.len(),
                |i| names[i].to_owned());
            

         
            ui.with_layout(egui::Layout::left_to_right(), |ui| { //username and banned checkbox
                ui.add_sized([100.0, 20.0], egui::TextEdit::singleline(&mut users[*selected].username).hint_text("Username"));
                ui.checkbox(&mut users[*selected].is_banned, "Is this account currently banned?");           
            });
            

            if users[*selected].is_banned {     
                
                match users[*selected].ban {
                    None => {
                        users[*selected].ban = Some(Ban { days: "0".to_string(), hours: "0".to_string(), mins: "0".to_string(), start: "0".to_string() }) 
                    }
                    _ => {}
                }

                ui.with_layout(egui::Layout::left_to_right(), |ui| {
                    ui.separator();
                    ui.add_sized([100.0, 20.0], egui::TextEdit::singleline(&mut users[*selected].ban.as_mut().unwrap().days).hint_text("Days"));
                    ui.add_sized([100.0, 20.0], egui::TextEdit::singleline(&mut users[*selected].ban.as_mut().unwrap().hours).hint_text("Hours"));
                    ui.add_sized([100.0, 20.0], egui::TextEdit::singleline(&mut users[*selected].ban.as_mut().unwrap().mins).hint_text("Mins"));
                });  
                                     
            }
 
            ui.separator();

            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                if ui.add_sized([100.0, 40.0], egui::Button::new("Save")).clicked() {

                        
                    if users[0].is_banned {

                        match users[0].ban {
                            Some(_) =>{
                                if let Err (_err) = write_file(&users[0], false){
                                    println!("There was an error writing to the file");  
                                }
                            }
                            _=> {}
                        } 
                    }
                    else {
                        if let Err (_err) = write_file(&users[0], false){
                            println!("There was an error writing to the file");  
                        }
                    }


                    for i in 1..users.len(){

                        if users[i].is_banned {
                            
                            match users[i].ban {
                                Some(_) => {
                                    if let Err (_err) = write_file(&users[i], true){
                                        println!("There was an error writing to the file");  
                                    }
                                }
                                _=> {}
                            }
                        }
                        else {
                            if let Err (_err) = write_file(&users[i], true){
                                println!("There was an error writing to the file");  
                            } 
                        }
                    }
                                   
                }
            });                
    });
    }


}
