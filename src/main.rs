use std::{borrow::Borrow, env, fmt, fs::{read_dir, read_to_string}, iter::Enumerate, path::PathBuf};
use dialoguer::Select;

#[derive(Debug,Clone)]
struct Directory {
    file: PathBuf,
    children: Vec<Directory>
}

impl Directory {
    fn remove(self, index:usize) {
        let files = self.to_array();
        drop(files[index].clone());
        print!("{:?}",files[index].children)
    }

    fn to_array(self) -> Vec<Directory> {
        let mut files:Vec<Directory> = Vec::new();
        for entity in self.children.iter() {
            files.push(entity.clone());    

            if !entity.children.is_empty() {
                files = [files, entity.clone().to_array()].concat();
            }
        }
        files
    }
    
}


impl fmt::Display for Directory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("{}/",format!("{}",env::current_dir()
            .unwrap()
            .file_name()
            .expect("Unable to print dir")
            .to_string_lossy()));

        for entry in format_tree(self.clone()) {
            writeln!(f,"{}",entry)?
        }
        Ok(())
    }  

}

// add remove index of selection
// create iter for struct


fn main() {
    let tree = build_tree(env::current_dir().unwrap());

    tree.remove(1);
}

fn build_tree(path:PathBuf) -> Directory {
    let mut dir = Directory {
        file: path.clone(),
        children: Vec::new()
    };
   
    let ingore:Vec<String> = read_to_string(".gitignore")
        .unwrap()
        .lines()
        .map(|s|format!("{}",s.replace("/","")))
        .collect();

    for entity in read_dir(path).expect("Could not read directory") {
        match entity {
            Ok(entity)=> {
                if ingore.contains(&format!("{}", entity.path()
                        .file_name()
                        .unwrap()
                        .to_string_lossy())) {
                    continue;
                }
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
        if file.is_dir() {format!("|- \x1b[32m{}/\x1b[0m", file.file_name().unwrap().to_string_lossy())} 
        else {format!("|- {}", file.file_name().unwrap().to_string_lossy())};

    formated_file.replace("\"", "")
}


fn format_tree(dir: Directory) -> Vec<String> {
    let mut files = Vec::new();
    for entity in dir.children {
            files.push(format!("{}",format_file(&entity.file)));

            if !entity.children.is_empty(){
                let temp_tree:Vec<String> =  format_tree(entity)
                    .into_iter()
                    .map(|s|format!("|  {}", s))
                    .collect();

                files = [&files[..], &temp_tree].concat();
            }    
    }
    return files;
}


