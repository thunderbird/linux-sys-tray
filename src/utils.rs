use std::env;
use std::io;

pub fn find_my_de() {
    let mut my_de = env::var("XDG_CURRENT_DESKTOP").unwrap();
    if my_de == "ubuntu:GNOME" {
        my_de = "GNOME".to_string();
    }
    match my_de.as_str() {
        "KDE" => println!("my_de is {my_de}"),
        "GNOME" => println!("my_de is {my_de}"),
        _ => println!("Unknown DE"),
    }
}


pub trait SomeTrait {
    fn get_current_dir(&self) -> io::Result<String>;
    fn get_assets_dir(&self);
}

pub struct RandomStruct{
    pub directory: String,
}

impl SomeTrait for RandomStruct {
    fn get_current_dir(&self) -> io::Result<String> {
        let current_dir = env::current_dir()?;
        //println!("The current directory is {}", current_dir.display());
        Ok(current_dir.display().to_string())
    }
    fn get_assets_dir(&self) {
        let current_dir = self.get_current_dir();
        match current_dir {
            Ok(directory) => {
                // Use the current_dir string here
                //println!("current directory is {}", directory);
                let assets_dir = directory + "/assets";
                println!("The assets directory is {}", assets_dir);
            },
            Err(error) => {
                // Handle the error
                eprintln!("An error occurred: {}", error);
            }
        } 
    }
}

/*
pub fn return_struct_for_dir(thing: RandomStruct) -> String{
    thing.directory
}
*/

pub fn get_icons_dir(op: &dyn SomeTrait) {
    op.get_assets_dir();
}