use std::env;
use std::io;
use std::path::PathBuf;

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
    fn get_current_dir(&self) -> io::Result<PathBuf>;
    fn get_assets_dir(&self) -> PathBuf;
}

pub struct RandomStruct{
    pub directory: PathBuf,
}

impl SomeTrait for RandomStruct {
    fn get_current_dir(&self) -> io::Result<PathBuf> {
        let current_dir = env::current_dir()?;
        //println!("The current directory is {}", current_dir.display());
        Ok(current_dir)
    }
    fn get_assets_dir(&self) -> PathBuf {
        //let current_dir = self.get_current_dir();
        //let assets_dir = current_dir.expect("ermagerd");
        let assets_dir = self.get_current_dir().expect("ohno");
        println!("From utils, the assets directory is {}", assets_dir.display());
        /*
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
        */
        assets_dir
    }
}

/*
pub fn return_struct_for_dir(thing: RandomStruct) -> String{
    thing.directory
}
*/

pub fn get_icons_dir(op: &dyn SomeTrait) -> PathBuf {
    op.get_assets_dir()
}