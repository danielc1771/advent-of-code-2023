use std::{fs, process::Command};

const PATH: &str = "C:\\Users\\danie\\dev\\advent-of-code-2023";
const DEFALUT_FILE_PATH: &str = "C:\\Users\\danie\\dev\\advent-of-code-2023\\advent\\new_day.rs";
fn main() {

    let day_string = get_day();
    validate_day_does_not_exist(&day_string);
    create_day_dir(&day_string);
}

fn get_day() -> String {
    let day = std::env::args().nth(1)
        .expect("Please pass in day");
    
    let day_as_num: Result<usize, _> = day.parse();

    if day_as_num.is_err() {
        panic!("Provided day is not a number!");    
    }

    return day;
}

fn validate_day_does_not_exist(day: &str) {
    let day_string = "day".to_owned() + day;

    for entry in fs::read_dir(PATH).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();

        if file_name.to_str().unwrap() == day_string {
            panic!("Provided day already exists!");
        }
    }
}

fn create_day_dir(day: &str) {
    let day_string = "day".to_owned() + day;
    let new_dir_name = PATH.to_owned() + "\\" + &day_string;
    let new_dir = fs::create_dir(&new_dir_name);

    if new_dir.is_err() {
        panic!("Failed to create_dir");
    }

    if let Err(err) = std::env::set_current_dir(&new_dir_name) {
        panic!("Unable to navigate to new dir: {}", err);
    }

    fs::File::create("input").expect("Failed to create input file");

    Command::new("cargo")
        .arg("init")
        .output()
        .expect("Failed to execute cargo init");

    fs::remove_file("./src/main.rs").expect("Failed to delete main.rs");

    fs::copy(DEFALUT_FILE_PATH, "./src/main.rs").expect("Failed to create main.rs");
}
