use gtk::prelude::*;
use gtk::{
    self, glib, Application, ApplicationWindow, Button, ComboBoxText, Frame, Grid, Label,
    Orientation, Box as GtkBox, gio, Align,
};

const APP_ID: &str = "org.gnomepods";

pub fn trigger_ui() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    // Main vertical container
    let main_vbox = GtkBox::new(Orientation::Vertical, 12);

    // --- Device Dropdown ---
    let device_selector = ComboBoxText::new();

    // Simulated list of devices. Replace this with your actual device fetch.
    let devices: Vec<&str> = vec![]; // ← Replace with real device detection

    if devices.is_empty() {
        device_selector.append_text("No AirPods Detected");
        device_selector.set_active(Some(0));
        device_selector.set_sensitive(false); // Make it unselectable
    } else {
        for dev in &devices {
            device_selector.append_text(dev);
        }
        device_selector.set_active(Some(0));
}
    device_selector.set_hexpand(true);
    device_selector.set_halign(Align::Start);
    main_vbox.append(&device_selector);

    // --- AIRPODS SECTION ---
    let airpods_hbox = GtkBox::new(Orientation::Horizontal, 12);
    airpods_hbox.set_hexpand(true);
    airpods_hbox.set_vexpand(true);

    // AirPod Left
    let airpod_left_frame = Frame::new(Some("AirPod Left"));
    airpod_left_frame.set_hexpand(true);
    airpod_left_frame.set_vexpand(true);
    let left_inner = GtkBox::new(Orientation::Vertical, 6);

    let left_label = Label::new(Some("Left Icon Placeholder"));
    left_label.set_vexpand(true);
    left_label.set_valign(Align::Center);
    left_label.set_halign(Align::Center);

    let left_battery = Label::new(Some("Battery: 87%"));
    left_battery.set_valign(Align::End);
    left_battery.set_halign(Align::Center);
    left_battery.set_margin_bottom(6);

    left_inner.append(&left_label);
    left_inner.append(&left_battery);
    airpod_left_frame.set_child(Some(&left_inner));

    // AirPod Right
    let airpod_right_frame = Frame::new(Some("AirPod Right"));
    airpod_right_frame.set_hexpand(true);
    airpod_right_frame.set_vexpand(true);
    let right_inner = GtkBox::new(Orientation::Vertical, 6);

    let right_label = Label::new(Some("Right Icon Placeholder"));
    right_label.set_vexpand(true);
    right_label.set_valign(Align::Center);
    right_label.set_halign(Align::Center);

    let right_battery = Label::new(Some("Battery: 92%"));
    right_battery.set_valign(Align::End);
    right_battery.set_halign(Align::Center);
    right_battery.set_margin_bottom(6);

    right_inner.append(&right_label);
    right_inner.append(&right_battery);
    airpod_right_frame.set_child(Some(&right_inner));

    airpods_hbox.append(&airpod_left_frame);
    airpods_hbox.append(&airpod_right_frame);

    let airpods_wrapper = GtkBox::new(Orientation::Vertical, 0);
    airpods_wrapper.set_vexpand(true);
    airpods_wrapper.append(&airpods_hbox);
    main_vbox.append(&airpods_wrapper);

    // --- GRID SECTION (Noise Control Modes) ---
    let grid_wrapper = GtkBox::new(Orientation::Vertical, 0);
    grid_wrapper.set_vexpand(true);

    let grid = Grid::new();
    grid.set_column_spacing(12);
    grid.set_row_spacing(12);
    grid.set_hexpand(true);
    grid.set_vexpand(true);

    let labels = ["Off", "Active", "Transparency", "Adaptive"];

    for row in 0..2 {
        for col in 0..2 {
            let index = row * 2 + col;

            let frame = Frame::new(Some(labels[index]));
            frame.set_hexpand(true);
            frame.set_vexpand(true);

            let inner_box = GtkBox::new(Orientation::Vertical, 6);
            let button = Button::with_label(&format!("Set {}", labels[index]));
            button.set_hexpand(true);
            button.set_vexpand(true);

            inner_box.append(&button);
            frame.set_child(Some(&inner_box));

            grid.attach(&frame, col as i32, row as i32, 1, 1);
        }
    }

    grid_wrapper.append(&grid);
    main_vbox.append(&grid_wrapper);

    // --- Final Window ---
    let window = ApplicationWindow::builder()
        .application(app)
        .title("gnomePods GUI")
        .default_width(600)
        .default_height(600)
        .child(&main_vbox)
        .build();

    window.present();
}
