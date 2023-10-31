mod main;

use std::path::PathBuf;
use glob::glob;
use std::ffi::OsStr;

fn main() -> std::io::Result<()>{
    let drivers = vec!["/docs-kotlin", 
    "/docs-golang", 
    "/docs-node",
    "/docs-java", 
    "/docs-csharp",
    "/docs-rust"];
    let base_dir = "../Repositories".to_string();
    let sub_dir = "/source".to_string();
    let txt_files = "/**/*.txt";
    let mut possible_files: Vec<PathBuf> = Vec::new();
    //get_all_names(&drivers, &base_dir, &sub_dir, txt_files, &mut possible_files);
    
    Ok(())
    
}

pub fn get_all_names(drivers: &Vec<&str>, base_dir: &String, sub_dir: &String, txt_files: &str, possible_files: &mut Vec<PathBuf>) {
        let driver = "/docs-golang";
        let path = format!("{}{}{}{}", base_dir, driver, sub_dir, txt_files);
        for entry in glob(&path).expect("Reading glob pattern") {
            match entry {
                Ok(path) => {
                        println!("\n\nGetting info for {:?}", path);
                        test_cases(path.file_name().unwrap(), drivers, base_dir, sub_dir, txt_files, possible_files);
                        let return_file = main::get_recently_created(possible_files);
                        let first_file_vec = main::get_file_vector(return_file.expect("Converting path to string").to_str().unwrap());
                        let second_created = main::get_second_recently_created(possible_files);
                        let second_file_vec = main::get_file_vector(second_created.to_str().unwrap());
                        let diff = main::calculate_percent_diff(first_file_vec, second_file_vec);
                        possible_files.clear();
                },
                Err(e) => println!("{:?}", e),
            }
        }
    }

pub fn test_cases(name: &OsStr, drivers: &Vec<&str>, base_dir: &String, sub_dir: &String, txt_files: &str, possible_files: &mut Vec<PathBuf>) {
    for driver in drivers.iter() {
        let path = format!("{}{}{}{}", base_dir, driver, sub_dir, txt_files);
        for entry in glob(&path).expect("Reading glob pattern") {
            match entry {
                Ok(path) => {
                    if path.file_name().unwrap().eq(name) {
                        possible_files.push(path);
                        
                    }
                },
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
