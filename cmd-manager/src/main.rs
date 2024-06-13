mod menu;

use chrono::{Local, Timelike};
use std::io::{self, Write};
use std::{fs, process, thread, time};

fn main() {
    clear_screen();
    menu::start_menu(true);

    'main_loop: loop {
        let mut user_input = String::new();
        println!("Enter your option: ");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read 'user input'.");

        match user_input.trim() {
            "1" => add_task(),
            "2" => remove_task(),
            "3" => show_task(),
            "4" => is_task_completed(),
            _ => {
                if exit_program(&user_input) {
                    break 'main_loop;
                }
                clear_screen()
            }
        }
        menu::start_menu(false);
    }
}

fn remove_task() {
    clear_screen();
    println!("###############################################");
    println!("#                  FUNC rem!                  #");
    println!("###############################################");
}
fn is_task_completed() {
    clear_screen();
    println!("###############################################");
    println!("#                  FUNC compl!                #");
    println!("###############################################");
}

/// Get all tasks in folder 'src\Tasks'
fn show_task() {
    clear_screen();
    println!("###############################################");
    println!("#                  All Tasks!                 #");
    println!("###############################################");
    get_tasks_from_folder();
    sleep_for(1500)
}

/// Create a new task with the user input. Generate field 'task_name' and 'task'.
fn add_task() {
    clear_screen();
    println!("###############################################");
    println!("#              Create a new task!             #");
    println!("###############################################");
    println!("- Please insert the name of the task:");
    let mut task_name: String = String::new();
    io::stdin()
        .read_line(&mut task_name)
        .expect("Failed to get 'task_name'.");
    println!("- Please insert the task:");
    let mut task: String = String::new();
    io::stdin()
        .read_line(&mut task)
        .expect("Failed to get 'task'");
    println!("###############################################");
    save_task(task_name, task);
}

/// Save task to disk via User inputs.
fn save_task(user_task_name: String, user_task: String) {
    println!("\n###############################################");
    println!("#                                             #");
    println!(
        "\n  - {}\n  - {}\n",
        &user_task_name.trim(),
        &user_task.trim()
    );
    check_folder();
    let current_time = get_current_time();
    let path = format!(
        "{}\\Tasks\\{}.txt",
        std::env::current_dir()
            .expect("Failed to access current directory.")
            .display(),
        &current_time
    );
    let mut file = fs::File::create(path).expect("Failed to create text file!");
    file.write_fmt(format_args!("{}\n{}", &user_task_name, &user_task))
        .expect("Failed to save task in file!");
    println!("###############################################");
    println!("#                 TASK saved!                 #");
    println!("###############################################\n\n");
    clear_screen();
    show_task()
}

/// Boolean to call 'mainloop break' to exit the program.
fn exit_program(input: &str) -> bool {
    if input.trim().to_lowercase().starts_with('q') || input.trim().to_lowercase().starts_with('e')
    {
        println!("###############################################");
        println!("#                  Thank you!                 #");
        println!("###############################################");
        return true;
    }
    false
}

/// Clean the screen via cls.
fn clear_screen() {
    process::Command::new("cmd")
        .args(["/c", "cls"])
        .status()
        .expect("Failed to clear the screen!");
}

/// Iniciate a timer with milliseconds as input.
fn sleep_for(milliseconds: u64) {
    thread::sleep(time::Duration::from_millis(milliseconds));
}

/// Calculate the Hours and Minutes from 'seconds_from_midnight'. Return a String and is used in 'save_task(), as exemple.'.
fn get_current_time() -> String {
    let hours = (Local::now().num_seconds_from_midnight() as f32) / 3600.00;
    let minutes = (&hours % 1.0) * 60.0;
    let hours = hours.abs() as i32;

    if hours < 10 && minutes < 10.0 {
        format!("0{:1.0}_0{:0.1}", hours, &minutes.to_string(),).to_string()
    } else if hours < 10 {
        format!("0{:1.0}_{:0.2}", hours, &minutes.to_string(),).to_string()
    } else if minutes < 10.0 {
        format!("{:0.0}_0{:0.1}", hours, &minutes.to_string(),).to_string()
    } else {
        format!("{:0.0}_{:0.2}", hours, &minutes.to_string(),).to_string()
    }
}

/// Loop to get all tasks in specific a folder 'src\Tasks'
fn get_tasks_from_folder() {
    check_folder();

    let path = format!(
        "{}\\Tasks",
        std::env::current_dir()
            .expect("Failed to access current directory.")
            .display(),
    );
    println!();
    match fs::read_dir(path) {
        Ok(tasks) => {
            for file in tasks {
                match file {
                    Ok(task) => {
                        println!("{:?}", task.file_name());
                    }
                    Err(e) => {
                        println!("Failed to read 'File Name' in folder 'Tasks'.\nError: {e}")
                    }
                }
            }
        }
        Err(e) => println!("Failed to read 'Tasks' folder content.\nError: {e}"),
    }
    println!();
}

/// Check if folder 'Tasks' exists in root folder.
fn check_folder() {
    fs::create_dir_all(format!(
        "{}\\Tasks\\",
        std::env::current_dir()
            .expect("Failed to access current directory.")
            .display()
    ))
    .expect("Failed to create folder 'Tasks'.");
}
