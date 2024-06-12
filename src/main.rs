use std::fs::File;
use std::io::{BufRead, BufReader};

const BUFFER_SIZE: usize = 84;

fn read_file_in_byte_chunks(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;

    let mut reader = BufReader::with_capacity(BUFFER_SIZE, file);

    loop {
        let buffer = reader.fill_buf()?;

        let buffer_length = buffer.len();

        // BufRead could not read any bytes.
        // The file must have completely been read.
        if buffer_length == 0 {
            break;
        }

        //do_something_with(buffer);
        println!("{:?}", buffer);

        // All bytes consumed from the buffer
        // should not be read again.
        reader.consume(buffer_length);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    read_file_in_byte_chunks("./assets/game/MAP.BIN")
}
