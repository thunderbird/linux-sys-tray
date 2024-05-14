use std::env;
use std::path::Path;
use std::path::PathBuf;
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
    fn do_something(&self);
}

struct One;
impl SomeTrait for One {
    fn do_something(&self) {
        println!("Doing something with One");
    }
}

pub fn return_one() -> One {
    One
}

pub fn do_stuff(op: &dyn SomeTrait) {
    op.do_something();
}

/*
pub fn get_assests_dir() -> io::Result<()> {
    /*
    let current_exe_path = env::current_exe().expect("Failed to get current executable path");
    let parent_dir = Path::new(&current_exe_path).parent().expect("Failed to get parent directory");
    println!("Parent directory: {:?}", parent_dir);
    */

    let current_dir = env::current_dir()?;
    //let parent_dir = current_dir.parent().expect("Failed to get parent directory");
    let assets_dir = current_dir.join("assets");
    
    println!("The current directory is {}", current_dir.display());
    //println!("Parent directory: {:?}", parent_dir);
    println!("{:?}", assets_dir);

    //let parent_dir = current_dir.parent().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Parent directory not found"))?;
    //println!("The parent directory is {}", parent_dir.display());
   
   Ok(())
   //assets_dir
}
*/

/*
pub fn use_assets_dir() {
    let get_assets_dir_result = get_assets_dir();
    if get_assets_dir() {
        let active_dir = Some(&parent_dir.to_string());
        println!("active_dir is {:?}", active_dir);
    }
}
*/