use console_engine::screen::Screen;
use console_engine::Color;
use console_engine::rect_style::BorderStyle;
use console::style;
use std::time::Duration;
use slowprint::slow_print;

pub fn welcome_format() {
    let mut scr = Screen::new(35,6);
    scr.rect_border(0, 0, 34, 5, BorderStyle::new_double());
    scr.print_fbg(3, 2, "Welcome to I Need a Reference!", Color::Cyan, Color::Black);
    scr.print_fbg(5, 3, "The reusable file finder", Color::White, Color::Black);
    scr.draw();
    let delay = Duration::from_millis(30);
    println!("\n");
    slow_print("Fetching your reference file.......", delay);
}

pub fn open_format() {
    let mut scr = Screen::new(25,5);
    println!();
    scr.print_fbg(0, 2, "Reference file found!", Color::Black, Color::Green);
    println!();
    scr.draw();
    let delay = Duration::from_millis(30);
    println!();
    slow_print("File opening in your default text editor.......", delay);
    println!();
}

pub fn stats_format(diff: f64, tier: u64, name: &String, category: &String, avg: f64, description: &String) {
    let mut scr = Screen::new(65,4);
    scr.print_fbg(0, 2, "Opened!", Color::Black, Color::Green);
    scr.draw();

    println!("\n");
    let delay = Duration::from_millis(30);
    slow_print("\nCalculating replacement information.......", delay);
    println!("\n");
    println!("You will probably replace about {:.2} percent of the '{}' reference file.", style(diff).cyan().on_black(), name);
    println!("On average, files in the '{}' category are {:.2} percent different from their counterparts in other doc sets.", category, avg);
    println!("Your new file requires tier {} replacement, which means you'll replace {}.", tier, style(description).cyan().on_black());
}