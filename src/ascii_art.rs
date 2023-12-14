fn print_ascii_art_digit(digit: u8) {
    match digit {
        0 => {
            println!("{:>10}", " 000 ");
            println!("{:>10}", "0   0");
            println!("{:>10}", "0   0");
            println!("{:>10}", "0   0");
            println!("{:>10}", " 000 ");
        }
        1 => {
            println!("{:>10}", "  1  ");
            println!("{:>10}", " 11  ");
            println!("{:>10}", "  1  ");
            println!("{:>10}", "  1  ");
            println!("{:>10}", " 111 ");
        }
        2 => {
            println!("{:>10}", " 222 ");
            println!("{:>10}", "2   2");
            println!("{:>10}", "   2 ");
            println!("{:>10}", "  2  ");
            println!("{:>10}", "2222 ");
        }
        3 => {
            println!("{:>10}", " 333 ");
            println!("{:>10}", "3   3");
            println!("{:>10}", "  33 ");
            println!("{:>10}", "3   3");
            println!("{:>10}", " 333 ");
        }
        4 => {
            println!("{:>10}", "4   4");
            println!("{:>10}", "4   4");
            println!("{:>10}", "44444");
            println!("{:>10}", "    4");
            println!("{:>10}", "    4");
        }
        5 => {
            println!("{:>10}", "55555");
            println!("{:>10}", "5    ");
            println!("{:>10}", " 555 ");
            println!("{:>10}", "    5");
            println!("{:>10}", "555 ");
        }
        6 => {
            println!("{:>10}", " 666 ");
            println!("{:>10}", "6    ");
            println!("{:>10}", "6666 ");
            println!("{:>10}", "6   6");
            println!("{:>10}", " 666 ");
        }
        7 => {
            println!("{:>10}", "77777");
            println!("{:>10}", "    7");
            println!("{:>10}", "   7 ");
            println!("{:>10}", "  7  ");
            println!("{:>10}", " 7   ");
        }
        8 => {
            println!("{:>10}", " 888 ");
            println!("{:>10}", "8   8");
            println!("{:>10}", " 888 ");
            println!("{:>10}", "8   8");
            println!("{:>10}", " 888 ");
        }
        9 => {
            println!("{:>10}", " 999 ");
            println!("{:>10}", "9   9");
            println!("{:>10}", " 9999");
            println!("{:>10}", "    9");
            println!("{:>10}", " 999 ");
        }
        _ => println!("Sorry, ASCII art not available for this digit."),
    }
}

pub fn print_ascii_art_number(number: u32) {
    let digits: Vec<u8> = number
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect();

    for digit in digits {
        print_ascii_art_digit(digit);
        println!(); // Separate each digit for better readability
    }
}
