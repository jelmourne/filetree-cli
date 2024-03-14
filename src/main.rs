use std::{env, fs::read_dir, path::PathBuf};

fn main() {
    show_tree();
}

fn represent_tree(dir: PathBuf) -> Vec<String> {
    let mut files = Vec::new();
    for entity in read_dir(dir).expect("Error reading dir") {
        match entity {
            Ok(entity)=> {
                let file = if entity.path().is_dir() {format!("|- {:?}/", entity.file_name())} else {format!("|- {:?}", entity.file_name())};
                files.push(file.replace("\"", ""));

                if entity.path().is_dir(){
                    let temp_tree:Vec<String> =  represent_tree(entity.path())
                        .into_iter()
                        .map(|s|format!("|   {}", s))
                        .collect();

                    files = [&files[..], &temp_tree]
                        .concat();
                }
                

            }
            Err(error) => {
                println!("{}",error)
            }
        }
    }
    return files;
}

fn show_tree() {
    for entry in represent_tree(env::current_dir().unwrap()) {
        println!("{}",entry)
    }
}
