use ksni;
mod utils;

#[derive(Debug)]
struct MyTray {
    selected_option: usize,
    checked: bool,
}

impl ksni::Tray for MyTray {

    fn icon_theme_path(&self) -> String {
        "/home/heather/Projects/sys-tray/tb-linux-sys-tray/assets".into()
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
    let service = ksni::TrayService::new(MyTray {
        selected_option: 0,
        checked: false,
    });
    let handle = service.handle();

    // I don't know where to send the assets_dir, so that it can be picked up by icon_theme_path
    // I want the above fn icon_theme_path to have something like:
    //      assets_dir.into()

    //let (outcome, assets_dir) = utils::get_assests_dir();
    //println!("Assets directory: {:?}", assets_dir);
    
    utils::find_my_de();

    // Based on the DE discovered, we could force the Thunderbird-Dark-symbolic.svg.
    // I don't know how to send whichever svg to icon_name to have something like:
    //      logo.into()
    // where logo would point to the svg file, which depends on the DE

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
