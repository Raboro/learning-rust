use std::{
    fs::File,
    io::{Error, Write},
};

pub fn create_file(name: &str) -> Option<Error> {
    let file_result: Result<File, Error> = File::create(name);
    if file_result.is_err() {
        return file_result.err();
    }

    let file = file_result.unwrap().write_all(b"hello there");
    if file.is_err() {
        file.err()
    } else {
        None
    }
}
