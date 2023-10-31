mod format;

use clap::Parser;
use glob::glob;
use std::fs;
use std::process::Command;
use std::{thread, time};
use std::path::PathBuf;
use std::time::SystemTime;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use std::process;

#[derive(Parser)]
struct Cli {
    filename: String,
    category: String,
    base: String,
}

fn main() -> std::io::Result<()> {
    // Processes the command line arguments
    format::welcome_format();
    let args = Cli::parse();
    let drivers = vec!["/docs-kotlin", 
                        "/docs-golang", 
                        "/docs-node",
                        "/docs-java", 
                        "/docs-csharp",
                        "/docs-rust"];
    let base_dir = format!("../{}", args.base);
    let txt_files = "/**/*.txt";
    let sub_dir;

    // Verifies that the command line arguments fit the required format
    if !check_args(&args, &base_dir) {
        process::exit(1);
    }

    if args.category.ne("other") {
        sub_dir = format!("/source/{}", args.category);
    } else {
        sub_dir = "/source".to_string();
    }

    // Stores files with the same name across repos in a vector
    let mut possible_files: Vec<PathBuf> = Vec::new();
    get_name_matches(&args, drivers, base_dir, sub_dir, txt_files, &mut possible_files);

    // Stores the two most recently created files out of the name matches
    let recent_files = get_two_recently_created(&mut possible_files);
    let most_recent = recent_files.0;
    let second_most_recent = recent_files.1;

    // Retrieves the final reference file
    let return_file = get_match(&most_recent, &second_most_recent);
    
    // Opens the reference file
    open_match(return_file);

    // Stores the content of the two most recently created files as vectors
    let first_file_vec = get_file_vector(most_recent.to_str().unwrap());
    let second_file_vec = get_file_vector(second_most_recent.to_str().unwrap());
    
    // Retrieves the approx difference percentage between two vectors and prints info
    let diff = calculate_percent_diff(first_file_vec, second_file_vec);
    let avg_stats = calculate_avg_info(&args, diff);
    let avg = avg_stats.0;
    let tier = avg_stats.1;
    let description = get_tier_description(tier);
    format::stats_format(diff, tier, &args.filename, &args.category, avg, &description);
    Ok(())
}

fn check_args(args: &Cli, base_dir: &String) -> bool {
    let v: Vec<char> = args.filename.chars().rev().take(3).collect();
    let last_three: String = v.into_iter().collect();
    let cat = &args.category;
    let base = Path::new(&base_dir);

    if last_three.ne("txt") {
        println!("\nPlease provide the full name of your new file as an argument, in <file name>.txt format.");
        return false;
    } else if cat.ne("fundamentals") && cat.ne("usage-examples") && cat.ne("other") {
        println!("\nPlease provide either 'fundamentals', 'usage-examples', or 'other' as your file category.");
        return false;
    } else if !base.try_exists().expect("Checking base directory") {
        println!("\nDouble check the name of your repo's base directory and make sure it lives in the same directory as this app.");
        return false;
    } else {
        return true;
    }
}

fn get_name_matches(args: &Cli, drivers: Vec<&str>, base_dir: String, sub_dir: String, txt_files: &str, possible_files: &mut Vec<PathBuf>){
    for driver in drivers.iter() {
        let path = format!("{}{}{}{}", base_dir, driver, sub_dir, txt_files);
        for entry in glob(&path).expect("Reading glob pattern") {
            match entry {
                Ok(path) => {
                    if path.file_name().expect("REASON").to_str().unwrap().eq(&args.filename) {
                        possible_files.push(path);
                    }
                },
                Err(e) => println!("{:?}", e),
            }
        }
    } 
}

pub fn get_two_recently_created(possible_files: &mut Vec<PathBuf>) -> (PathBuf, PathBuf) {
    if possible_files.len() < 2 {
        println!("\nLooks like there aren't enough matching files to compute replacement stats. Sorry!");
        process::exit(1);
    }
    let mut times: Vec<(&PathBuf, SystemTime)> = vec![];
    for file in possible_files.iter() {
        let current_metadata = fs::metadata(file).ok();
        let Ok(time) = current_metadata.expect("Retrieving creation time").created() else {panic!("Can't retrieve creation time")};
        times.push((file, time));
    }

    times.sort_by(|a, b| b.1.cmp(&a.1));
    // println!("Second most recent file: {}", times.get(1).unwrap().0.display());

    return (times.get(0).unwrap().0.to_path_buf(), times.get(1).unwrap().0.to_path_buf());
}

pub fn get_match(most_recent: &PathBuf, second_most_recent: &PathBuf) -> PathBuf {
    let most_recent_len = (fs::metadata(most_recent).unwrap().len()) as f64;
    let next_recent_len = (fs::metadata(second_most_recent).unwrap().len()) as f64;

    let Ok(most_recent_modified) = fs::metadata(most_recent).unwrap().modified() else { panic!("Can't retrieve modification time") };
    let Ok(next_recent_modified) = fs::metadata(second_most_recent).unwrap().modified() else { panic!("Can't retrieve modification time") };

    if next_recent_len > most_recent_len && next_recent_modified > most_recent_modified {
        return second_most_recent.to_path_buf();
    } else {
        return most_recent.to_path_buf();
    }
}

pub fn check_empty(file: Option<&PathBuf>) {
    if file.is_none() {
        println!("\nLooks like there isn't a good reference file for you.");
        println!("You might be writing about an entirely new concept, or you mistyped your file name.");
        process::exit(1);
    }
}

pub fn open_match(return_file: PathBuf) {
    format::open_format();
    let path = format!("{}", return_file.to_string_lossy());
    
    let wait = time::Duration::from_millis(500);
    thread::sleep(wait);
    let _open_file = Command::new("open")
                                .arg(path)
                                .status()
                                .expect("Opening a file");
}

pub fn get_file_vector(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("Opening the specified file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Parsing each line"))
        .collect()
}

pub fn calculate_percent_diff(first_file_vec: Vec<String>, second_file_vec: Vec<String>) -> f64 {
    let first_file_set: HashSet<_> = first_file_vec.iter().clone().collect();
    let second_file_set: HashSet<_> = second_file_vec.iter().clone().collect();
    let diff: Vec<_> = first_file_set.difference(&second_file_set).collect();

    let total = (first_file_vec.len() + (second_file_vec.len() / 2)) as f64;
    let differences = (diff.len()) as f64;
    let percent = (differences/total) * 100.0;
    if percent >= 100.00 {
        return 100.00;
    } else {
        return percent;
    }
}

fn calculate_avg_info(args: &Cli, diff: f64) -> (f64, u64) {
    let std_dev;
    let avg;

    let fund = "fundamentals".to_string();
    let us = "usage-examples".to_string();

    if args.category.eq(&fund) {
        std_dev = 9.831; 
        avg = 32.114;
    } else if args.category.eq(&us) {
        std_dev = 10.425;
        avg = 31.697;
    } else {
        std_dev = 10.844; 
        avg = 27.63;
    }

    if diff <= avg - std_dev {
        return (avg, 1);
    } else if diff <= avg {
        return (avg, 2);
    } else if diff <= avg + std_dev {
        return (avg, 3);
    } else {
        return (avg, 4);
    }
}

fn get_tier_description(tier: u64) -> String {
    match tier {
        1 => return "less text than average".to_string(), 
        2 => return "an average, or slightly below average, amount of text".to_string(), 
        3 => return "an average, or slightly above average, amount of text".to_string(), 
        _ => return "more text than average".to_string(), 
    }
}