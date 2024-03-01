use std::{
    fs::File,
    io::{Error, Write},
};

pub fn create_file(name: &str, content: Option<String>) -> Option<Error> {
    let file_result: Result<File, Error> = File::create(name);
    if file_result.is_err() {
        return file_result.err();
    }

    let file: Result<(), Error> = file_result
        .unwrap()
        .write_all(content.unwrap_or("Hello World".to_owned()).as_bytes());
    if file.is_err() {
        file.err()
    } else {
        None
    }
}
