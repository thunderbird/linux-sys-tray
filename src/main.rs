use ksni;
use std::env;

#[derive(Debug)]
struct MyTray {
    selected_option: usize,
    checked: bool,
}

impl ksni::Tray for MyTray {
    /*fn find_my_de() {
        enum LinuxDesktopEnv {
            KDE,
            GNOME,
        }
        let my_de = env::var("XDG_CURRENT_DESKTOP").unwrap();
        //println!("{MyDe}");
        //if let Some(desktop) = env::var("XDG_CURRENT_DESKTOP").and_then(|e| Some(e.split(';'));
        if my_de == LinuxDesktopEnv::KDE {
            println!("my DE is KDE!");
        }
    }*/
    fn icon_theme_path(&self) -> String {
        "/home/heather/Projects/sys-tray".into()
    }
    fn icon_name(&self) -> String {
        "Thunderbird".into()
    }
    fn title(&self) -> String {
        if self.checked { "CHECKED!" } else { "MyTray" }.into()
    }
    // NOTE: On some system trays, `id` is a required property to avoid unexpected behaviors
    fn id(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }
    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            SubMenu {
                label: "a".into(),
                submenu: vec![
                    SubMenu {
                        label: "a1".into(),
                        submenu: vec![
                            StandardItem {
                                label: "a1.1".into(),
                                ..Default::default()
                            }
                            .into(),
                            StandardItem {
                                label: "a1.2".into(),
                                ..Default::default()
                            }
                            .into(),
                        ],
                        ..Default::default()
                    }
                    .into(),
                    StandardItem {
                        label: "a2".into(),
                        ..Default::default()
                    }
                    .into(),
                ],
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            RadioGroup {
                selected: self.selected_option,
                select: Box::new(|this: &mut Self, current| {
                    this.selected_option = current;
                }),
                options: vec![
                    RadioItem {
                        label: "Option 0".into(),
                        ..Default::default()
                    },
                    RadioItem {
                        label: "Option 1".into(),
                        ..Default::default()
                    },
                    RadioItem {
                        label: "Option 2".into(),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }
            .into(),
            CheckmarkItem {
                label: "Checkable".into(),
                checked: self.checked,
                activate: Box::new(|this: &mut Self| this.checked = !this.checked),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Exit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        ]
    }
}

fn main() {
    //find_my_de();
    let service = ksni::TrayService::new(MyTray {
        selected_option: 0,
        checked: false,
    });
    let handle = service.handle();


    let mut my_de = env::var("XDG_CURRENT_DESKTOP").unwrap();
    if my_de == "ubuntu:GNOME" {
        my_de = "GNOME".to_string();
    }
    match my_de.as_str() {
        "KDE" => println!("my_de is {my_de}"),
        "GNOME" => println!("my_de is {my_de}"),
        _ => println!("Unknown DE"),
    }

/*
    let desktop = env::var("XDG_CURRENT_DESKTOP").unwrap();
    let my_de = Some();
    assert_eq!(my_de.ok_or_else(|| 0), Ok(desktop));

    let my_de: Option<&str> = None;
    assert_eq!(my_de.ok_or_else(|| 0), Err(0));
*/

/*
    let x = Some("KDE");
    assert_eq!(x.ok_or_else(|| 0), Ok("KDE"));

    //let x: Option<&str> = None;
    //assert_eq!(x.ok_or(0), Err(0));
*/
/*
    if let Some(KDE) = env::var("XDG_CURRENT_DESKTOP") {
        println!("my_de is {KDE}");
    }
*/
    /*
    if let ok_or_else(Some(desktop))? = env::var("XDG_CURRENT_DESKTOP") {
        println!("my de is {desktop}")
    }
    */
    service.spawn();

    std::thread::sleep(std::time::Duration::from_secs(5));
    // We can modify the tray
    handle.update(|tray: &mut MyTray| {
        tray.checked = true;
    });
    // Run forever
    loop {
        std::thread::park();
    }
}