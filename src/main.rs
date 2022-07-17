use std::env;
use std::fs;

mod in_directory;
fn help_message(){
    println!("punch (optional)<flag> <file/location> \n
    
    punch -h 'to bring up help' \n
    punch <file_name> 'to create file' \n
    punch -dir ./<directory_name>/ 'create directory' \n
    punch ./<directory_name>/ 'to use without -dir flag' \n
    punch -d <file_name> 'or'  punch -d ./<directory_name>/ 'to delete' \n
    punch -in ./<target_directory_name>/ <file or directory_name> 'creates files inside target directory' \n
    punch -din ./<target_directory_name>/ <file or directory_name> 'deletes files inside target directory' \n
    ")
}

fn create_files(args: &Vec<String>){
    for i in 1..args.len(){
        if args[i].contains("/") && args[i].ends_with("/"){
                fs::create_dir_all(&args[i])
                .expect(format!("error creating folder: {}", args[i]).as_str());
        }
        else{
            fs::File::create(&args[i])
            .expect(format!("error creating file: {}", args[i]).as_str());

        }
    }
}
fn delete_files(args: &Vec<String>){
    for i in 2..args.len(){
        if args[i].contains("/") && args[i].ends_with("/"){
            fs::remove_dir_all(&args[i])
            .expect(format!("error deleting folder: {}", args[i]).as_str());
        }
        else{
            fs::remove_file(&args[i])
            .expect(format!("error deleting file: {}", args[i]).as_str());
        }
    }
}
fn create_directory(args: &Vec<String>){
    for i in 2..args.len(){
        fs::create_dir_all(&args[i])
        .expect(format!("error creating folder: {}", args[i]).as_str());
    }
}

fn main(){
    let args: Vec<String> = env::args().collect();

    match args[1].as_str(){
       "-d"|"-delete" => delete_files(&args),
       "-h"|"-help" => help_message(),
       "-din" => in_directory::delete_files_dir(&args),
       "-in" => in_directory::create_in_dir(&args),
       "-dir" => create_directory(&args),
       _ => create_files(&args)
    }


    
}