use std::io::{self, Write};

pub fn confirm(msg: String) -> bool {
    print!("{} [y/n]: ", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.pop();
            if input == 'y'.to_string() {
                return true;
            }
            false
        }
        Err(_) => false,
    }
}
