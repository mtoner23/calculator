#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui::{FontId, TextStyle, Vec2};
use regex;

mod taxes;
use taxes::FilingStatus;

const WINDOW_WIDTH: f32 = 1500.0;
const WINDOW_HEIGHT: f32 = 600.0;

fn validate_dollar(s: &str) -> Option<f64> {
    // Use regex to validate the format
    if s.is_empty() {
        return Some(0.);
    }

    let re = regex::Regex::new(r"^-?\$?(\d{1,3}(,\d{3})*|\d+)(\.\d{1,2})?$").unwrap();
    if re.is_match(s) {
        let cleaned = &s.replace('$', "").replace(',', "");
        return Some(cleaned.parse::<f64>().unwrap());
    } else {
        return None;
    }
}

struct MyApp {
    typed_income: String,
    typed_bonus: String,
    typed_deduction: String,
    typed_pre_tax: String,
    filing_status: FilingStatus,
    use_standard: bool,
    // deduction: f64,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        MyApp {
            typed_income: "".to_owned(),
            typed_bonus: "".to_owned(),
            typed_deduction: "".to_owned(),
            typed_pre_tax: "".to_owned(),
            filing_status: FilingStatus::Single,
            use_standard: true,
            // deduction: 0.0,
        }
    }

    fn update_text_styles(ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();

        style.text_styles.insert(TextStyle::Body, FontId::new(35.0, egui::FontFamily::Proportional));
        style.text_styles.insert(TextStyle::Heading, FontId::new(64.0, egui::FontFamily::Proportional));
        style.text_styles.insert(TextStyle::Button, FontId::new(25.0, egui::FontFamily::Proportional));
        style.spacing.interact_size = Vec2::new(35.0, 35.0);

        ctx.set_style(style);
    }

    fn left_side(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("This year regular income");
                ui.label("This year Bonus");
                ui.label("Standard Deduction");
                ui.label("Itemized Deduction");
                ui.label("Pre-Tax Contributions");
                ui.label("Filing status");
            });
            ui.vertical(|ui| {
                ui.text_edit_singleline(&mut self.typed_income);
                ui.text_edit_singleline(&mut self.typed_bonus);
                ui.checkbox(&mut self.use_standard, "");
                ui.add_enabled(!self.use_standard, egui::TextEdit::singleline(&mut self.typed_deduction));
                ui.text_edit_singleline(&mut self.typed_pre_tax);
                ui.vertical(|ui| {
                    ui.radio_value(&mut self.filing_status, FilingStatus::Single, "Single");
                    ui.radio_value(&mut self.filing_status, FilingStatus::MarriedSeparate, "Married Filing Separate");
                    ui.radio_value(&mut self.filing_status, FilingStatus::MarriedJoint, "Married Filing Jointly");
                });
            });
        });
    }

    fn right_side(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            let validated_income = validate_dollar(&self.typed_income).unwrap_or_else(|| {
                ui.label("Invalid income format");
                0.0
            });
            ui.label(format!("Regular Income: {}", validated_income));

            let validated_bonus = validate_dollar(&self.typed_bonus).unwrap_or_else(|| {
                ui.label("Invalid bonus format");
                0.0
            });
            ui.label(format!("Bonus Income : {}", validated_bonus));

            let total_income = validated_income + validated_bonus;
            ui.label(format!("Total Income : {}", total_income));

            let deduction: f64 = match self.use_standard {
                true => taxes::get_standard_decution(self.filing_status),
                false => validate_dollar(&self.typed_deduction).unwrap_or_else(|| {
                    ui.label("Err: Invalid deduction amount");
                    0.0
                }),
            };

            let deductions = deduction
                + validate_dollar(&self.typed_pre_tax).unwrap_or_else(|| {
                    ui.label("Err: Invalid pre-tax amount");
                    0.0
                });

            let taxable_income = total_income - deductions;
            ui.label(format!("Taxable Income : {}", taxable_income));

            let regular_withheld = taxes::calculate_income_tax(validated_income - deductions, self.filing_status);
            let bonus_actual_withheld = validated_bonus * taxes::BONUS_WITHHELD_RATE;
            let total_needs_withheld = taxes::calculate_income_tax(taxable_income, self.filing_status);
            let withholding_differnce = total_needs_withheld - bonus_actual_withheld - regular_withheld;

            ui.label(format!("Regular Income tax withheld: {}", regular_withheld));
            ui.label(format!("Bonus Income actual withheld: {}", bonus_actual_withheld));

            ui.label(format!("Total Income needed withheld: {}", total_needs_withheld));
            ui.label(format!("Withholding Difference: {}", withholding_differnce));
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            MyApp::update_text_styles(ctx);
            ui.heading("Bonus Tax Helper");
            ui.label("Description of why this is necessary");

            ui.horizontal(|ui| {
                MyApp::left_side(self, ui);
                MyApp::right_side(self, ui);
            });
        });
    }
}

fn main() -> eframe::Result {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
        ..Default::default()
    };

    return eframe::run_native("My egui App", options, Box::new(|cc| Ok(Box::new(MyApp::new(cc)))));
}
