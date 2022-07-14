use std::env;
use std::fs;

fn create_file(directories: &String, files: Vec<&str>){
    fs::File::create(files[0]).expect("something went wrong, try typing: punch -d <your_file>"); //creates first file in list with location
    
    if files.len() > 1{ //if more than one file in list (comma seperated)
        for i in 1..files.len(){
                fs::File::create(format!("{}{}",directories, files[i])).expect("something went wrong, try typing, punch -h ");
        }
    }
    
}

fn help_message(){
    println!("punch (optional)<flag> <file/location> \n
    -h displays help \n
    -d lets you dynamically create folders or file location ie: hello/world/test.txt \n
    ")
}

fn dynamically_create(file_location: &String){
    let mut directories = String::new();
    let mut contains_files: bool = false;
    for i in file_location.split("/"){
        if i.contains("."){
            contains_files = true;
            break;
        }
        directories.push_str(i);
        directories.push_str("/");
    }

    fs::create_dir_all(&directories).expect("error creating folders"); //create directories mentioned
    
    if contains_files{
        create_file(&directories, file_location.split(",").collect());
    }
    //directories is the folder location without the files
    //second param may look like this [folder1/file1, file2, file3]



}
fn flag_check(flag: &String, file_location: &String){
    match flag.as_str(){
        "-h"|"-help" => help_message(),
        "-d" => dynamically_create(file_location),
        _ => println!("syntax err: punch -h"),
    }
}

fn main(){
    let args: Vec<String> = env::args().collect();

    match args.len(){
        2 => create_file(&String::from(""),vec![&args[1]]),
        3 => flag_check(&args[1], &args[2]),
        _ => println!("syntax err: punch -h"),
    }

    println!("succesfully created files");

    
}
