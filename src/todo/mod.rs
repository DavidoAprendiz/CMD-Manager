use crate::utils;
use std::{env, thread, time};
mod new_task;
mod remove_task;
mod show_task;
mod view_task;

pub fn main() {
    start_menu();
    'main_loop: loop {
        println!("Enter your option: ");
        let user_input = utils::get_user_input();

        match user_input.trim() {
            "1" => new_task::add_task(),
            "2" => remove_task::remove_task(),
            "3" => show_task::show_task(),
            "4" => view_task::view_task(),
            _ => {
                if utils::exit_program(&user_input) {
                    break 'main_loop;
                }
            }
        }
        start_menu();
    }
}

/// Run the menu layout.
fn start_menu() {
    utils::clear_screen();
    utils::create_folder(tasks_folder());
    println!("###############################################");
    println!("#                                             #");
    println!("#                To-do Manager                #");
    println!("#                                             #");
    println!("###############################################");
    println!("#                                             #");
    println!("#  Select an operation:                       #");
    println!("#                                             #");
    println!("#     '1' -> New Task                         #");
    println!("#     '2' -> Remove Task                      #");
    println!("#     '3' -> Show All Tasks                   #");
    println!("#     '4' -> View Task                        #");
    println!("#                                             #");
    println!("#     'Q' -> Exit                             #");
    println!("###############################################");
}

fn tasks_folder() -> String {
    if env::consts::OS.contains("windows") {
        "\\Project\\Tasks\\".to_string()
    } else {
        "/Project/Tasks/".to_string()
    }
}

/// Initiate a timer with milliseconds as input.
fn sleep_for(milliseconds: u64) {
    thread::sleep(time::Duration::from_millis(milliseconds));
}
