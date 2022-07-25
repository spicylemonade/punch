use std::fs;

mod args;
#[cfg(test)]
#[test]
pub fn create_in_dir() {
    let arg_struct = args::Args::default();
    let args = arg_struct.r#in.clone().unwrap(); 
    for i in 1..args.len() {
        if args[i].contains("/") {
            fs::create_dir_all(format!("{}{}", args[0], args[i]))
                .expect(format!("error creating directory: {}", args[i]).as_str());

        } else {
            fs::File::create(format!("{}/{}", args[0], args[i]))
                .expect(format!("error creating file: {}", args[i]).as_str());
        }
    }
    
    fs::remove_dir_all("./test_dir/r#in_test/").unwrap();
    fs::remove_file("./test_dir/r#in_test.txt").unwrap();
}
#[test]
pub fn delete_files_dir() {
    {
        fs::File::create("./test_dir/din_test.txt").unwrap();
        fs::create_dir_all("./test_dir/din_test/").unwrap();
    }
    let arg_struct = args::Args::default();
    let args = arg_struct.din.clone().unwrap();
    for i in 1..args.len() {
        if args[i].contains("/") {
            fs::remove_dir_all(format!("{}{}", args[0], args[i]))
                .expect(format!("error removing directory: {}", args[i]).as_str());
        } else {
            fs::remove_file(format!("{}/{}", args[0], args[i]))
                .expect(format!("error creating directory: {}", args[i]).as_str());
        }
    }
}