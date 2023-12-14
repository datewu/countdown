use std::io::{self, Write};
use std::time::Duration;
use std::{env, thread};

mod ascii_art;

fn main() {
    count_down(parse_args());
}

fn parse_args() -> i32 {
    const DEFAULT: i32 = 600; // 10min
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        args[1].parse().unwrap_or(DEFAULT)
    } else {
        DEFAULT
    }
}

fn count_down(secs: i32) {
    // const CLEAR: &str = "\x1B[2K\r"; // Clear the current line
    // const CLEAR: &str = "\x1B[0J"; // ANSI escape code to clear from the cursor position to the end of the screen
    const CLEAR: &str = "\x1B[2J\x1B[1;1H"; // ANSI escape codes to clear the screen and move the cursor to the top-left corner
    for i in (1..=secs).rev() {
        print!("{}", CLEAR);
        println!("Time left:");
        println!("");
        ascii_art::print_ascii_art_number(i as u32);
        println!("{:>15}", " seconds");
        let _ = io::stdout().flush(); // Flush the output to ensure the message is displayed
        thread::sleep(Duration::from_secs(1));
    }

    print!("{}", CLEAR);
    println!("Time's up!");
}
