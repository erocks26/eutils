pub fn main(cli: Vec<String>) {
    println!("{:?}", cli);
    eu_sleep(std::time::Duration::new(5, 0));
}

fn eu_sleep(duration: std::time::Duration) {
    std::thread::sleep(duration);
}