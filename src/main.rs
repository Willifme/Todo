#[macro_use]
extern crate clap;

extern crate app_dirs;
extern crate chrono;
extern crate config;
extern crate csv;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod cli;
mod todo;
mod todos;

//use chrono::prelude::*;
//use app_dirs::{AppInfo, AppDataType, get_app_dir};
//use todos::Todos;

//const APP_INFO: AppInfo = AppInfo{name: APP_NAME, author: APP_AUTHOR};

fn main() {
    cli::CLI::new().run();
    //println!("{}", get_app_dir(AppDataType::UserData, &APP_INFO, "todos.csv")
    //.unwrap().to_str().unwrap());

    /*let mut todos = Todos::new();

    todos.prepopulate();

    todos.sort();

    //println!("{}", todos);
    
    println!("Overdue: ");

    println!("{}", todos.overdue());
    
    println!();
    
    println!("Soon: ");

    println!("{}", todos.soon(Local::now().date()));

    Ok(())
    */
}
