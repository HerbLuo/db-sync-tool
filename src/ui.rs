use tray_item::TrayItem;

pub fn start_tray() {
    gtk::init().unwrap();
    let mut tray = TrayItem::new("数据库同步工具", "accessories-calculator").unwrap();
    tray.add_menu_item("打开主界面", || {
        webbrowser::open("https://www.baidu.com").unwrap();
    }).unwrap();
    tray.add_menu_item("退出", || {
        gtk::main_quit();
    }).unwrap();
    gtk::main();
}
