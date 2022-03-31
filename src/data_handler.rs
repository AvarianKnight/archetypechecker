use std::fs::{File};
use std::io::{BufReader, Read};
use quick_xml::events::Event;
use quick_xml::Reader;
use crate::{DATA};
use std::path::{Path};

fn handle_model_push(file: String, name: String ) {
    let mut mutator = DATA.lock().unwrap();
    if !mutator.contains(&name) {
        mutator.push(name);
    } else {
        println!("{} contains duplicate {}", file, name);
    }

}

pub(crate) fn handle_data(path: &Path) {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open file {}, with error {}", path.display(), err);
            return
        }
    };
    let mut file_string = String::new();
    BufReader::new(file).read_to_string(&mut file_string).unwrap();

    let mut reader = Reader::from_str(&*file_string);

    let mut read_for_name = false;
    let mut read_name = false;

    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {

                match e.name() {
                    b"Item" => {
                        if e.attributes().map(|a| {
                            a.unwrap().unescape_and_decode_value(&reader).unwrap()
                        }).collect::<Vec<_>>().contains(&String::from("CMloArchetypeDef")) {
                            read_for_name = true
                        }
                    },
                    b"name" => {
                        if read_for_name {
                            read_name = true;
                        }
                    }
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => {
                if read_name {
                    let chars = e.unescape_and_decode(&reader).unwrap();
                    handle_model_push(path.display().to_string(), chars);
                    read_for_name = false;
                    read_name = false;
                }

            },
            Ok(Event::Eof) => break,
            Err(e) => println!("Error in file {} {}: {:?}", path.display(), reader.buffer_position(), e),
            _ => (),
        }

        buf.clear();
    }
}
