
use std::env;

#[derive(Debug)]
struct MyTray {
    //selected_option: usize,
    checked: bool,
}

impl ksni::Tray for MyTray {
    fn title(&self) -> String {
        if self.checked { "Thunderbird" } else { "MyTray" }.into()
    }
    // NOTE: On some system trays, `id` is a required property to avoid unexpected behaviors
    fn id(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }
    fn icon_theme_path(&self) -> String {
        let mut assets_dir = env::current_dir().expect("error");
        assets_dir.push("assets");
        assets_dir.display().to_string().into()
    }
    fn icon_name(&self) -> String {
        let my_de = env::var("XDG_CURRENT_DESKTOP").expect("Error - no detected Linux desktop");
        let mut preferred_icon = "Thunderbird.svg";
        if my_de
            .replace(":", ";")
            .split(";")
            .map(|m| m.to_ascii_lowercase())
            .any(|e| &e == "gnome")
        {
            preferred_icon = "tb-symbolic-white.svg";
        } else if my_de
            .replace(":", ";")
            .split(";")
            .map(|m| m.to_ascii_lowercase())
            .any(|e| &e == "kde")
        {
            preferred_icon = "Thunderbird_Logo_Outline-Light.svg";
        }
        preferred_icon.into()
    }
    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![StandardItem {
            label: "Exit".into(),
            icon_name: "application-exit".into(),
            activate: Box::new(|_| std::process::exit(0)),
            ..Default::default()
        }
        .into()]
    }
}

fn main() {
    let service = ksni::TrayService::new(MyTray {
        //selected_option: 0,
        checked: false,
    });
    let handle = service.handle();

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
