use std::env;
use std::fs;

fn help_message(){
    println!("punch (optional)<flag> <file/location> \n
    -h displays help \n
    -d lets you dynamically create folders or file location ie: hello/world/test.txt \n
    ")
}

fn create_files_dir(args: &Vec<String>){
    for i in 1..args.len(){
        if args[i].contains("/"){
                fs::create_dir_all(&args[i]).expect(format!("error creating folder: {}", args[i]).as_str());
        }
        else{
            fs::File::create(&args[i]).expect(format!("error creating file: {}", args[i]).as_str());

        }
    }
}

fn create_in_dir(args: &Vec<String>){
    for i in 3..args.len(){
        if args[i].contains("/"){
            fs::create_dir_all(format!("{}{}",args[2], args[i])).expect(format!("error creating folder: {}", args[i]).as_str());
        }
        else{
            fs::File::create(format!("{}/{}",args[2], args[i])).expect(format!("error creating file: {}", args[i]).as_str());
        }
    }
}

fn delete_files(args: &Vec<String>){
    for i in 2..args.len(){
        if args[i].contains("/"){
            fs::remove_dir_all(&args[i]).expect(format!("error deleting folder: {}", args[i]).as_str());
        }
        else{
            fs::remove_file(&args[i]).expect(format!("error deleting file: {}", args[i]).as_str());
        }
    }
}


fn main(){
    let args: Vec<String> = env::args().collect();

    match args[1].as_str(){
       "-d"|"-delete" => delete_files(&args),
       "-h"|"-help" => help_message(),
       "-dir" => create_in_dir(&args),
       _ => create_files_dir(&args)
    }

    //println!("succesfully created files");

    
}
