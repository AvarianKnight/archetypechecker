mod data_handler;
use std::io::{Write, Read};
use std::{fs, io};
use std::sync::{Mutex};



fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref DATA: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn handle_dir(data: &str) {
    let walk_dir = fs::read_dir(data).unwrap();
    for entry in walk_dir {
        let entry = entry.unwrap();
        data_handler::handle_data(&entry.path());
    }

}
fn main() {
    handle_dir("./data/default_maps");
    handle_dir("./data/maps");
    pause()
}
