#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui::{FontId, TextStyle};

const WINDOW_WIDTH: f32 = 1500.0;
const WINDOW_HEIGHT: f32 = 600.0;

#[derive(Default)]
struct MyApp {
    count1: i32,
    slider: i32,
    typed_income: String,
    typed_bonus: String,
}

fn text_edit_label<S: egui::TextBuffer>(ui: &mut egui::Ui, label: String, editable: &mut S) {
    ui.horizontal(|ui| {
        let my_label = ui.label(label);
        ui.text_edit_singleline(editable).labelled_by(my_label.id);
    });
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        return Self::default();
    }

    fn left_side(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.label("Test");
            ui.label("Test 2");

            text_edit_label(ui, "This year regular income: ".to_owned(), &mut self.typed_income);

            text_edit_label(ui, "This year Bonus: ".to_owned(), &mut self.typed_bonus);

            ui.add(egui::Slider::new(&mut self.slider, 0..=100).text(""));
        });
    }

    fn right_side(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            let income_text = format!("Income {}", self.typed_income);
            ui.label(income_text);
            ui.label("Right 2");
            // ui_counter(ui, &mut self.count1);
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            update_text_styles(ctx);
            ui.heading("Hello Test Application!");

            ui.horizontal(|ui| {
                MyApp::left_side(self, ui);
                MyApp::right_side(self, ui);
            });
            // let mut regular_income: &str = "";
        });
    }
}

fn ui_counter(ui: &mut egui::Ui, counter: &mut i32) {
    ui.horizontal(|ui| {
        if ui.button("−").clicked() {
            *counter -= 1;
        }
        ui.label(counter.to_string());
        if ui.button("+").clicked() {
            *counter += 1;
        }
    });
}

fn update_text_styles(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.text_styles.insert(TextStyle::Body, FontId::new(40.0, egui::FontFamily::Proportional));
    style.text_styles.insert(TextStyle::Heading, FontId::new(64.0, egui::FontFamily::Proportional));

    ctx.set_style(style);
}

fn main() -> eframe::Result {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
        ..Default::default()
    };

    // Our application state:
    // let mut name = "Arthur".to_owned();
    // let mut age = 42;

    return eframe::run_native("My egui App", options, Box::new(|cc| Ok(Box::new(MyApp::new(cc)))));
}
