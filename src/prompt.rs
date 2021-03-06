use std::io;
use std::io::Write;


pub fn prompt_user(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut result = String::new();
    io::stdin()
        .read_line(&mut result)
        .expect("Can't read line!");

    result.trim().to_string()
}
