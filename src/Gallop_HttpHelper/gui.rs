use crate::Gallop_HttpHelper::helper::str_to_c_char;
use crate::il2cpp::helper::log;
use crate::plugin_api::{GuiMenuSectionCallback, Vtable};
use egui::Key::B;
use egui::{TextBuffer, widgets};
use once_cell::sync::Lazy;
use std::ffi::{CString, c_void};
use std::ptr;
use crate::Gallop_HttpHelper::ura::{get_config, save_config, Config};

pub static mut VTABLE: Option<&'static Vtable> = None;
struct Gui {
    config_window: bool,
    config:Config,
    notifier_timeout_ms:String
}
extern "C" fn callback(ui: *mut c_void, userdata: *mut c_void) {
    unsafe {
        let vtable = VTABLE.unwrap();
        if userdata.is_null() {
            return;
        }
        let gui = &mut *(userdata as *mut Gui);
        if (vtable.gui_ui_button)(ui, str_to_c_char("URA设置")) {
            gui.config_window = true;
        }
        let eui = &mut *(ui as *mut egui::Ui);
        let ctx = eui.ctx();
        let scale = ctx
            .data(|d| d.get_temp::<f32>(egui::Id::new("gui_scale")))
            .unwrap_or(1.0);
        let mut open=gui.config_window;
        egui::Window::new("URA设置")
            .pivot(egui::Align2::CENTER_CENTER)
            .fixed_pos(ctx.viewport_rect().max / 2.0)
            .min_width(96.0 * scale)
            .max_width(320.0 * scale)
            .max_height(250.0 * scale)
            .collapsible(false)
            .resizable(false)
            .open(&mut gui.config_window)
            .show(&ctx, |ui| {
                ui.vertical(|ui| {
                    egui::Grid::new("URA_config")
                        .num_columns(2)
                        .spacing([10.0, 10.0])
                        .show(ui, |ui| {
                            ui.label("URL:");
                            ui.add(egui::TextEdit::singleline(&mut gui.config.notifier_host).desired_width(200.0));
                            ui.end_row();
                            ui.label("超时:");
                            ui.add(egui::TextEdit::singleline(&mut gui.notifier_timeout_ms).desired_width(200.0));
                            ui.label("ms");
                            ui.end_row();
                            ui.label("启用URA:");
                            ui.checkbox(&mut gui.config.enable, "启用");
                            ui.end_row();
                        });
                });

                ui.add_space(10.0);
                ui.separator(); // 分割线
                ui.add_space(5.0);

                // --- 2. 底部按钮区域 ---
                ui.horizontal(|ui| {
                    // 左侧放置一个“恢复默认”类的按钮（可选，参考你的截图）

                    // 将接下来的按钮推向右侧
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // 右下角：取消按钮
                        if ui.button("取消").clicked() {
                            // 如果取消，直接关闭窗口不保存逻辑
                            open = false;
                            gui.config=(*get_config()).clone();
                        }

                        // 右下角：保存按钮
                        if ui.button("保存").clicked() {
                            gui.config.notifier_timeout_ms=gui.notifier_timeout_ms.parse::<u64>().unwrap_or(500);
                            save_config(&gui.config);
                            open=false;
                        }
                    });
                });
            });

        gui.config_window&=open;
    }
}

pub unsafe fn init(vtable: &'static Vtable) {
    VTABLE = Some(vtable);
    let gui = Box::new(Gui {
        config_window: false,
        config:(*get_config()).clone(),
        notifier_timeout_ms: get_config().notifier_timeout_ms.to_string()
    });
    (vtable.gui_register_menu_section)(Some(callback as _), Box::into_raw(gui) as *mut c_void);
}
