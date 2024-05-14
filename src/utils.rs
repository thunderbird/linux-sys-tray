use std::env;
use std::path::Path;

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

pub fn get_assests_dir() -> std::io::Result<()> {
    /*
    let current_exe_path = env::current_exe().expect("Failed to get current executable path");
    let parent_dir = Path::new(&current_exe_path).parent().expect("Failed to get parent directory");
    println!("Parent directory: {:?}", parent_dir);
    */

    let current_dir = env::current_dir()?;
    let parent_dir = current_dir.parent().expect("Failed to get parent directory");
    let assets_dir = parent_dir.join("assets");
    println!("The current directory is {}", current_dir.display());
    println!("Parent directory: {:?}", parent_dir);
    println!("Assets directory: {:?}", assets_dir);

    //let parent_dir = current_dir.parent().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Parent directory not found"))?;
    //println!("The parent directory is {}", parent_dir.display());
    Ok(())
}