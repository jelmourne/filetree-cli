use std::{env, fs::read_dir, path::PathBuf};

#[derive(Clone)]
struct Directory {
    file: PathBuf,
    children: Vec<Directory>
}

impl Directory {
    fn print_tree(self) {
        println!("{}/",format!("{:?}",env::current_dir()
            .unwrap()
            .file_name()
            .expect("Dir"))
            .replace("\"", ""));

        for entry in represent_string_tree(self) {
            println!("{}",entry)
        }
    }
}


fn main() {
    let tree = build_tree(env::current_dir().unwrap());
    tree.print_tree()
    
}

fn build_tree(path:PathBuf) -> Directory {
    let mut dir = Directory {
        file: path.clone(),
        children: Vec::new()
    };
    for entity in read_dir(path).expect("Could not read directory") {
        match entity {
            Ok(entity)=> {
                if entity.path().is_dir() {
                    dir.children.push(build_tree(entity.path()));
                    continue;
                }
                dir.children.push(Directory {
                    file: entity.path(),
                    children: Vec::new()
                })
            }
            Err(error) => {
                println!("{}", error)
            }
        }
    }
    dir
}


fn format_file(file: &PathBuf) -> String {
    let formated_file = 
        if file.is_dir() {format!("|- {}/", file.file_name().unwrap().to_string_lossy())} 
        else {format!("|- {}", file.file_name().unwrap().to_string_lossy())};

    formated_file.replace("\"", "")
}


fn represent_string_tree(dir: Directory) -> Vec<String> {
    let mut files = Vec::new();
    for entity in dir.children {
            files.push(format!("{}",format_file(&entity.file)));

            if !entity.children.is_empty(){
                let temp_tree:Vec<String> =  represent_string_tree(entity)
                    .into_iter()
                    .map(|s|format!("|  {}", s))
                    .collect();

                files = [&files[..], &temp_tree]
                    .concat();
            }    
    }
    return files;
}


