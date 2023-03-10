use eu_common::throw_error;

pub fn main(cli: Vec<String>) {
    eu_sleep(std::time::Duration::from_secs_f64(conv_to_secs(
        parse_input(cli),
    )));
}

fn eu_sleep(duration: std::time::Duration) {
    std::thread::sleep(duration);
}

fn parse_input(input: Vec<String>) -> Vec<(f64, char)> {
    // TODO adapt this to work with floats
    let mut result: Vec<(f64, char)> = Vec::new();
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
            throw_error(1, "sleep", &format!("'{}' is not a supported suffix!", ch));
        }

        // Get the number
        let mut value: String = String::new();
        for c in clean_input.chars() {
            if c.is_digit(10) || c == '.' {
                value.push(c);
            } else {
                throw_error(1, "sleep", "Input contains non number values. Check that your suffix is in the right place!");
            }
        }
        let value: f64 = value.parse().unwrap();
        result.push((value, suffix));
    }
    result
}

fn conv_to_secs(parsed_input: Vec<(f64, char)>) -> f64 {
    // TODO adapt this to work with floats
    let mut running: f64 = 0.0;
    for (n, c) in parsed_input {
        running += match c {
            's' => n,
            'm' => n * 60.0,
            'h' => n * 3600.0,
            'd' => n * 86400.0,
            e => {
                println!("Somehow your parsed input contains an invalid character, {}. This should literally be impossible...", e);
                println!("I would say 'try --help for more info' but honestly, that isn't gonna help lol");
                std::process::exit(2);
            }
        };
    }
    running
}
