use crate::types::ZzErrors;
use tray_item::TrayItem;
use crate::helper::arguments::MATCHES;

fn open_browser() {
    MATCHES.with(|matches| {
        let address = matches.value_of("ADDRESS").unwrap();
        let port = matches.value_of("PORT").unwrap();
        webbrowser::open(&format!("http://{}:{}", address, port)).unwrap();
    });
}

#[cfg(target_os = "linux")]
pub fn start_tray() -> Result<(), ZzErrors> {
    gtk::init().map_err(|e| ZzErrors::GuiError(format!("初始化gtk失败{}", e)))?;
    let mut tray = TrayItem::new("数据库同步工具", "accessories-calculator").unwrap();
    tray.add_menu_item("打开主界面", open_browser).unwrap();
    tray.add_menu_item("退出", || {
        gtk::main_quit();
    }).unwrap();
    gtk::main();
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn start_tray() -> Result<impl FnOnce(), ZzErrors> {
    use std::cell::RefCell;

    let mut tray = TrayItem::new("数据库同步工具", "").map_err(|e| ZzErrors::GuiError(format!("初始化tray失败{}", e)))?;

    tray.inner_mut().add_menu_item("打开主界面", open_browser).unwrap();
    tray.inner_mut().add_quit_item("退出");

    let tray_ref_cell = RefCell::new(tray);
    Ok(move || tray_ref_cell.borrow_mut().inner_mut().display())
}
