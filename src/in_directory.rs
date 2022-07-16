use std::fs;



pub fn create_in_dir(args: &Vec<String>){
    for i in 3..args.len(){
        if args[i].contains("/"){
            fs::create_dir_all(format!("{}{}",args[2], args[i]))
            .expect(format!("error creating folder: {}", args[i]).as_str());
        }
        else{
            fs::File::create(format!("{}/{}",args[2], args[i]))
            .expect(format!("error creating file: {}", args[i]).as_str());
        }
    }
}

pub fn delete_files_dir(args: &Vec<String>){
    for i in 3..args.len(){
        if args[i].contains("/"){
            fs::remove_dir_all(format!("{}{}",args[2], args[i]))
            .expect(format!("error creating folder: {}", args[i]).as_str());
        }
        else{
            fs::remove_file(format!("{}/{}",args[2], args[i]))
            .expect(format!("error creating folder: {}", args[i]).as_str());
        }
    }
}