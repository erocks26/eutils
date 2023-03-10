pub fn throw_error<T: Into<i32>>(exit_code: T, prog_name: &str, err_message: &str) {
    println!("eu_{}: {}", prog_name, err_message);
    println!("Try '{} --help' for more information.", prog_name);
    std::process::exit(exit_code.into());
}
