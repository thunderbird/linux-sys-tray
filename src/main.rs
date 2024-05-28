
use std::env;

#[derive(Debug)]
struct MyTray {
    //selected_option: usize,
    checked: bool,
}

impl dbus_hooks::Tray for MyTray {
    fn title(&self) -> String {
        if self.checked { "Thunderbird" } else { "MyTray" }.into()
    }
    // NOTE: On some system trays, `id` is a required property to avoid unexpected behaviors
    fn id(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }
    fn menu(&self) -> Vec<dbus_hooks::MenuItem<Self>> {
        use dbus_hooks::menu::*;
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
    let service = dbus_hooks::TrayService::new(MyTray {
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
