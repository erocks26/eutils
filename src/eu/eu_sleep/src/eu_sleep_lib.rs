pub fn main(cli: Vec<String>) {
    eu_sleep(std::time::Duration::new(conv_to_secs(parse_input(cli)), 0));
}

fn eu_sleep(duration: std::time::Duration) {
    std::thread::sleep(duration);
}

fn parse_input(input: Vec<String>) -> Vec<(u64, char)> { // TODO adapt this to work with floats
    let mut result: Vec<(u64, char)> = Vec::new();
    for i in input {
        let mut clean_input: String = i;
        // Check if last character is a supported suffix or a number
        // If it is supported remove that char from the string
        let mut suffix: char = 's';
        let ch = clean_input.chars().last().unwrap();
        if ch == 's' || ch == 'm' || ch == 'h' || ch == 'd' {
            suffix = ch;
            clean_input.pop();
        } else if ch.is_digit(10) != true {
            println!("{} is not a supported suffix!", ch);
            std::process::exit(1);
        }

        // Get the number
        let mut value: String = String::new();
        for c in clean_input.chars() {
            if c.is_digit(10) {
                value.push(c);
            } else {
                println!("Input contains non number values. Check that your suffix is in the right place!");
                std::process::exit(1);
            }
        }
        let value: u64 = value.parse().unwrap();
        result.push((value, suffix));
    }
    result
}

fn conv_to_secs(parsed_input: Vec<(u64, char)>) -> u64 { // TODO adapt this to work with floats
    let mut running: u64 = 0;
    for (n, c) in parsed_input {
        running += match c {
            's' => n,
            'm' => n * 60,
            'h' => n * 3600,
            'd' => n * 86400,
            _ => {
                println!("Somehow your parsed input contains an invalid character. This should literally be impossible...");
                std::process::exit(2);
            }
        };
    }
    running
}