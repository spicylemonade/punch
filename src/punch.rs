use anyhow::Result;
use std::fs;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::path::Path;

// Reusable functions should be added to this file
pub fn move_file(from: &Path, to: &Path) -> Result<()> {
    let mut f = fs::File::open(&from)?;

    let mut file_buffer = Vec::new();
    f.read_to_end(&mut file_buffer)?;

    let mut dest_file_buffer = BufWriter::new(fs::File::create(to)?);
    dest_file_buffer.write_all(&file_buffer)?;
    dest_file_buffer.flush()?;

    Ok(())
}

pub fn create_file(file: &Path) -> Result<()> {
    fs::File::create(file)?; //.expect(format!("error creating file: {:?}", file).as_str());
    Ok(())
}

pub fn create_directory(dir: &Path) -> Result<()> {
    fs::create_dir_all(dir)?; //.expect(format!("error creating folder: {:?}", dir).as_str());
    Ok(())
}

pub fn remove_file(file: &Path) -> Result<()> {
    fs::remove_file(file)?; //.expect(format!("error deleting folder: {:?}", file).as_str());
    Ok(())
}

pub fn remove_directory(dir: &Path) -> Result<()> {
    fs::remove_dir_all(dir)?; //.expect(format!("error deleting folder: {:?}", dir).as_str());
    Ok(())
}
