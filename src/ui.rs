use crate::types::ZzErrors;
use tray_item::TrayItem;

#[cfg(target_os = "linux")]
pub fn start_tray() -> Result<(), ZzErrors> {
    gtk::init().map_err(|e| ZzErrors::GuiError(format!("初始化gtk失败{}", e)))?;
    let mut tray = TrayItem::new("数据库同步工具", "accessories-calculator").unwrap();
    tray.add_menu_item("打开主界面", || {
        webbrowser::open("https://www.baidu.com").unwrap();
    }).unwrap();
    tray.add_menu_item("退出", || {
        gtk::main_quit();
    }).unwrap();
    gtk::main();
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn start_tray() {
    let mut tray = TrayItem::new("数据库同步工具", "").unwrap();
    tray.add_menu_item("打开主界面", || {
        webbrowser::open("http://localhost:9886").unwrap();
    }).unwrap();
    let inner = tray.inner_mut();
    inner.add_quit_item("退出");
    inner.display();
}
