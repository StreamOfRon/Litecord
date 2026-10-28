use std::cell::RefCell;
use tray_icon::{
    menu::{Menu, MenuItem, MenuId},
    Icon, TrayIcon, TrayIconBuilder,
};
use log::{info, warn};

thread_local! {
    // The icon must be touched only on the thread that created it (the UI thread).
    static TRAY_ICON: RefCell<Option<TrayIcon>> = RefCell::new(None);
    // Menu items are kept here so their texts can be re-set on language
    // changes. Updating texts in place (instead of rebuilding the menu) keeps
    // the MenuEvent id mapping valid — see the plan's Review Focus.
    static TRAY_ITEMS: RefCell<Option<(MenuItem, MenuItem)>> = RefCell::new(None);
}

/// Sets the tray tooltip from any thread. Runs on the UI thread only when called,
/// so nothing polls for tooltip changes.
pub fn update_tray_tooltip(tooltip: &str) {
    let tip = tooltip.to_string();
    let _ = slint::invoke_from_event_loop(move || {
        TRAY_ICON.with(|icon| {
            if let Some(icon) = icon.borrow().as_ref() {
                let _ = icon.set_tooltip(Some(tip));
            }
        });
    });
}

/// Re-labels the tray menu and resets the startup tooltip to the selected
/// language. Call from the UI thread device — `apply_i18n_translations` does
/// it for every language application, so a switch without restart keeps the
/// tray in sync (localization-consistency Goal 3).
pub fn set_tray_texts() {
    let texts = super::i18n_ui::ui_text();
    let show = texts.tray_show.to_string();
    let quit = texts.tray_quit.to_string();
    let tooltip = texts.tray_tooltip.to_string();
    let _ = slint::invoke_from_event_loop(move || {
        TRAY_ITEMS.with(|cell| {
            if let Some((show_item, quit_item)) = cell.borrow().as_ref() {
                let _ = show_item.set_text(show.as_str());
                let _ = quit_item.set_text(quit.as_str());
            }
        });
        TRAY_ICON.with(|icon| {
            if let Some(icon) = icon.borrow().as_ref() {
                let _ = icon.set_tooltip(Some(tooltip.as_str()));
            }
        });
    });
}

pub struct SystemTrayManager {
    pub show_item_id: MenuId,
    pub quit_item_id: MenuId,
}


fn create_default_icon() -> Icon {
    let icon_bytes = include_bytes!("../../assets/app_icon.png");
    if let Ok(img) = image::load_from_memory(icon_bytes) {
        let rgba_img = img.into_rgba8();
        let (width, height) = rgba_img.dimensions();
        if let Ok(icon) = Icon::from_rgba(rgba_img.into_raw(), width, height) {
            return icon;
        }
    }

    let width = 32u32;
    let height = 32u32;
    let mut rgba = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let is_border = x == 0 || x == width - 1 || y == 0 || y == height - 1;
            if is_border {
                rgba.extend_from_slice(&[0x11, 0x12, 0x14, 0xFF]);
            } else {
                rgba.extend_from_slice(&[0x58, 0x65, 0xF2, 0xFF]);
            }
        }
    }

    Icon::from_rgba(rgba, width, height).expect("Falha ao gerar ícone do tray")
}

impl SystemTrayManager {
    /// Call on the UI thread.
    pub fn setup() -> Self {
        #[cfg(target_os = "linux")]
        {
            let _ = gtk::init();
        }

        let tray_menu = Menu::new();
        let texts = super::i18n_ui::ui_text();
        let show_item = MenuItem::new(texts.tray_show, true, None);
        let quit_item = MenuItem::new(texts.tray_quit, true, None);

        let show_id = show_item.id().clone();
        let quit_id = quit_item.id().clone();
        TRAY_ITEMS.with(|cell| {
            *cell.borrow_mut() = Some((show_item.clone(), quit_item.clone()));
        });

        let _ = tray_menu.append(&show_item);
        let _ = tray_menu.append(&quit_item);

        let icon = create_default_icon();

        let tray_icon = match TrayIconBuilder::new()
            .with_icon(icon)
            .with_menu(Box::new(tray_menu))
            .with_menu_on_left_click(true)
            .with_tooltip(texts.tray_tooltip)
            .build()
        {
            Ok(icon) => {
                info!("Ícone roxo ativado na bandeja do sistema (System Tray).");
                Some(icon)
            }
            Err(e) => {
                warn!("Não foi possível inicializar o ícone do System Tray ({:?}), continuando modo janela.", e);
                None
            }
        };

        TRAY_ICON.with(|icon| *icon.borrow_mut() = tray_icon);

        Self {
            show_item_id: show_id,
            quit_item_id: quit_id,
        }
    }
}


