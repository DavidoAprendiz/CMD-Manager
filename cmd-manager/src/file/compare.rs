use crate::utils;
use std::io::{BufRead, BufReader};

pub fn main() {
    utils::clear_screen();
    start_menu();
    get_files()
}

/// Run the menu layout.
fn start_menu() {
    println!("###############################################");
    println!("#                                             #");
    println!("#                Compare Files!               #");
    println!("#                                             #");
    println!("###############################################");
}

fn get_files() {
    utils::get_files_from_folder();
    println!("- First file:");
    let file_1 = utils::get_user_input();
    println!("- Second file to compare:");
    let file_2 = utils::get_user_input();
    get_results(file_1, file_2);
}

fn get_results(file_1: String, file_2: String) {
    let file1_path = utils::get_file_path(file_1);
    let file2_path = utils::get_file_path(file_2);
    println!("\n###############################################\n");
    println!(
        "The results for:\nFile 1 ->  {}\nFile 2 ->  {}\n",
        &file1_path, &file2_path
    );

    let mut file1_vector: Vec<String> = Vec::new();
    let mut file2_vector: Vec<String> = Vec::new();

    let mut count_total: u64 = 0;

    let file1_path = std::fs::File::open(file1_path);
    let reader = BufReader::new(file1_path.unwrap());
    for line1 in reader.lines() {
        if let Ok(l1) = line1 {
            file1_vector.push(l1);
            count_total += 1;
        } else {
            println!("Failed to get line from file.");
        }
    }

    println!();

    let file2_path = std::fs::File::open(file2_path);
    let reader = BufReader::new(file2_path.unwrap());
    for line2 in reader.lines() {
        if let Ok(l2) = line2 {
            file2_vector.push(l2);
            count_total += 1;
        } else {
            println!("Failed to get line from file.");
        }
    }

    let big = if file1_vector.len() > file2_vector.len() {
        &file1_vector
    } else {
        &file2_vector
    };

    for i in 0..=big.len() {
        if file1_vector.get(i) != file2_vector.get(i) {
            println!("diff detected:   {:?}", big.get(i));
        }
    }

    println!("Total diffs detected ->  {}", count_total);
    println!("\n###############################################");
    println!("vector 1:         {:?}", file1_vector);
    println!("vector 2:         {:?}", file2_vector);

    // println!("- {count_total} occurrences in total.");
    // println!("###############################################\n");

    //
    // Just to hold the result until key is pressed...
    utils::get_user_input();
    utils::clear_screen()
}
