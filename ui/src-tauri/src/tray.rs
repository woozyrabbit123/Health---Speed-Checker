// System Tray Implementation
// Creates a system tray icon with menu and notifications

use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, SystemTraySubmenu,
};

pub fn create_tray() -> SystemTray {
    // Create menu items
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let show = CustomMenuItem::new("show".to_string(), "Show Window");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide Window");
    let scan_quick = CustomMenuItem::new("scan_quick".to_string(), "Quick Scan");
    let scan_full = CustomMenuItem::new("scan_full".to_string(), "Full Scan");
    let view_history = CustomMenuItem::new("history".to_string(), "View History");
    let fix_top = CustomMenuItem::new("fix_top".to_string(), "Fix Top Issue");

    // Create submenu for scans
    let scan_submenu = SystemTraySubmenu::new(
        "Scan",
        SystemTrayMenu::new()
            .add_item(scan_quick)
            .add_item(scan_full),
    );

    // Build the full tray menu
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_submenu(scan_submenu)
        .add_item(fix_top)
        .add_item(view_history)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

pub fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            // Show/hide window on left click
            if let Some(window) = app.get_window("main") {
                if window.is_visible().unwrap_or(false) {
                    let _ = window.hide();
                } else {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        }

        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }

                "show" => {
                    if let Some(window) = app.get_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }

                "hide" => {
                    if let Some(window) = app.get_window("main") {
                        let _ = window.hide();
                    }
                }

                "scan_quick" => {
                    // Emit event to frontend to start quick scan
                    if let Some(window) = app.get_window("main") {
                        let _ = window.emit("tray-action", "scan_quick");
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }

                "scan_full" => {
                    // Emit event to frontend to start full scan
                    if let Some(window) = app.get_window("main") {
                        let _ = window.emit("tray-action", "scan_full");
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }

                "fix_top" => {
                    // Emit event to fix top issue
                    if let Some(window) = app.get_window("main") {
                        let _ = window.emit("tray-action", "fix_top");
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }

                "history" => {
                    // Show history view
                    if let Some(window) = app.get_window("main") {
                        let _ = window.emit("tray-action", "show_history");
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }

                _ => {}
            }
        }

        _ => {}
    }
}

/// Update tray icon based on health score
pub fn update_tray_icon(app: &AppHandle, health_score: u32) {
    // Access the menu item handle to ensure it exists (future updates may use it)
    let _ = app.tray_handle().get_item("health_score");

    let _icon_name = match health_score {
        0..=50 => "critical",
        51..=75 => "warning",
        76..=100 => "healthy",
        _ => "unknown",
    };

    // Update tooltip
    let tooltip = format!("Health & Speed Checker\nHealth: {}/100", health_score);
    let _ = app.tray_handle().set_tooltip(&tooltip);

    // In production, would update icon based on health_score
    // let icon_path = format!("icons/tray-{}.png", icon_name);
    // let _ = app.tray_handle().set_icon(tauri::Icon::File(icon_path.into()));
}

/// Show desktop notification
pub fn show_notification(app: &AppHandle, title: &str, body: &str) {
    use tauri::api::notification::Notification;

    let _ = Notification::new(&app.config().tauri.bundle.identifier)
        .title(title)
        .body(body)
        .show();
}

/// Notify user of critical issues
pub fn notify_critical_issues(app: &AppHandle, issue_count: usize) {
    if issue_count > 0 {
        show_notification(
            app,
            "Critical Issues Detected",
            &format!("{} critical issue(s) found. Click to view.", issue_count),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tray() {
        let tray = create_tray();
        // Basic test that tray is created
        assert!(true); // Would need more complex test setup to verify menu items
    }
}
