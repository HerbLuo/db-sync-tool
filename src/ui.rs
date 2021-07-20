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
pub fn start_tray() -> Result<impl FnOnce(), ZzErrors> {
    use std::{cell::RefCell, rc::Rc};

    let mut tray = TrayItem::new("数据库同步工具", "").map_err(|e| ZzErrors::GuiError(format!("初始化tray失败{}", e)))?;

    tray.inner_mut().add_menu_item("打开主界面", || {
        webbrowser::open("http://localhost:9886").unwrap();
    }).unwrap();
    tray.inner_mut().add_quit_item("退出");

    let tray_ref_cell = Rc::new(RefCell::new(tray));
    Ok(move || (*tray_ref_cell).borrow_mut().inner_mut().display())
}
