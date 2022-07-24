use std::{fs, path::Path, io::BufWriter};
use std::io::Read;
use std::io::Write;

// Reusable functions should be added to this file
pub fn move_file( from: &Path , to: &Path) {
    let mut f= fs::File::open(&from).unwrap();
    let mut file_buffer = Vec::new();
    f.read_to_end(&mut file_buffer).unwrap();

    let mut dest_file_buffer = BufWriter::new(fs::File::create(to).unwrap());
    dest_file_buffer.write_all(&file_buffer).unwrap();
    dest_file_buffer.flush().unwrap();
}