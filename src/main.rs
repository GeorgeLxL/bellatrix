mod bellatrix;

use eframe::{epi::App, run_native, NativeOptions, epi};

use std::{borrow::Cow, iter::FromIterator};

use eframe::egui::{self, Button, Color32, CtxRef, FontDefinitions, FontFamily, Hyperlink, Label, Layout, Separator, TopBottomPanel, Ui, CentralPanel, ScrollArea, TextStyle, Vec2, RichText};
use crate::bellatrix::Bellatrix;

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);


/*
pub(crate) fn render_top_panel(&self, ctx: &CtxRef) {
    // define a TopBottomPanel widget
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.add_space(10.);
        egui::menu::bar(ui, |ui| {
            // logo
            ui.with_layout(Layout::left_to_right(), |ui| {
                ui.add(Label::new("📓").text_style(egui::TextStyle::Heading));
            });
            // controls
            ui.with_layout(Layout::right_to_left(), |ui| {
                let close_btn = ui.add(Button::new("❌").text_style(egui::TextStyle::Body));
                let refresh_btn = ui.add(Button::new("🔄").text_style(egui::TextStyle::Body));
                let theme_btn = ui.add(Button::new("🌙").text_style(egui::TextStyle::Body));
            });
        });
        ui.add_space(10.);
    });
}
*/

fn render_footer(ctx: &CtxRef) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            render_monospaced_label(ui, "Made with ♥ By Cau1dr0n 1ake");
            render_monospaced_label(ui, "Donate: 0x43aF68DcC19Bbce20F3354F31Bc1159f651643aE");
            render_hyperlink(ui, "https://domain.com", "Buy API KEY");
            ui.add_space(10.);
        })
    });
}


fn render_monospaced_label(ui: &mut Ui, text: &str) {
    ui.add(
        Label::new(
            RichText::new(text)
                .color(WHITE)
                .heading()
        ).monospace()
    );
}

fn render_hyperlink(ui: &mut Ui, url: &str, label: &str) {
    ui.add(
        Hyperlink::from_label_and_url(label, url),
    );
}

fn render_separator(ui: &mut Ui, amount: f32, spacing: f32) {
    ui.add_space(amount);
    let sep = Separator::default().spacing(spacing);
    ui.add(sep);
}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.add(
            Label::new(
                RichText::new("Account Info")
                    .color(WHITE)
                    .heading()
            )
        );

        ui.label("");

        ui.vertical_centered(|ui2| {
            ui2.label("0xa2F9...1b3E");
            ui2.label("Balance: 1.233");
        });
    });

    render_separator(ui, PADDING, 20.);
}





impl epi::App for Bellatrix {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self {
            privateKey,
            force_buy_percent,
            force_sell_percent,
            auto_swap,
            logs
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });



        egui::CentralPanel::default().show(ctx, |ui| {

            render_header(ui);
            render_footer(ctx);




            ui.horizontal(|ui| {
                ui.label("API KEY: ");
                ui.text_edit_singleline(privateKey);

                if ui.button("Set API KEY").clicked() {
                    println!("dsfsdf");
                    println!("{:?}", chrono::offset::Local::now());
                }
            });

            ui.label("");
            ui.horizontal(|ui| {
                ui.label("PrivateKey: ");
                ui.text_edit_singleline(privateKey);

                if ui.button("Load account").clicked() {
                    println!("dsfsdf");
                    println!("{:?}", chrono::offset::Local::now());
                }
            });

            ui.horizontal(|ui| {
                ui.label("Contract: ");
                ui.text_edit_singleline(privateKey);

                if ui.button("Set Contract").clicked() {
                    println!("dsfsdf");
                    println!("{:?}", chrono::offset::Local::now());
                }
            });

            ui.label("");
            ui.horizontal(|ui| {
                ui.label("Swap From: ");
                ui.text_edit_singleline(privateKey);

                if ui.button("Set Contract").clicked() {
                    println!("dsfsdf");
                    println!("{:?}", chrono::offset::Local::now());
                }
            });
            ui.horizontal(|ui| {
                ui.label("Swap To: ");
                ui.text_edit_singleline(privateKey);

                if ui.button("Set Contract").clicked() {
                    println!("dsfsdf");
                    println!("{:?}", chrono::offset::Local::now());
                }
            });


            // These are equivalent:
            ui.label("");
            ui.heading("Swap configuration");


            // BUY
            ui.label("");
            ui.horizontal(|ui| {
                ui.checkbox(auto_swap, "Enable Auto Swap");
            });

            ui.horizontal(|ui| {
                ui.label("BUY");
                ui.add(egui::Slider::new(force_buy_percent, 0.0..=100.0));
                ui.label("%");
                if ui.button("Force Buy").clicked() {
                    *force_buy_percent += 1.0;
                }

                ui.label("\t\t");

                ui.horizontal(|ui| {
                    ui.label("SELL");
                    ui.label("");
                });

                ui.add(egui::Slider::new(force_sell_percent, 0.0..=100.0));
                ui.label("%");
                if ui.button("Force Sell").clicked() {
                    *force_sell_percent += 1.0;
                }
            });

            ui.label("");

            ui.heading("Log: ");
            ui.vertical(|ui| {
                ui.label("2022/02/06 02:39:16:  dsfsdf sdfs df sdfsdfsd dsf  sdf");
                ui.label("2022/02/06 02:39:16:  dsfsdf sdfs df sdfsdfs  sdf");
                ui.label("2022/02/06 02:39:16:  dsfsdf sdfs df sdfsdfsd");
                ui.label("2022/02/06 02:39:16:  dsfsdf sdfs df sdfsdfsd dsdsfsdff  sdf: ");
                ui.label("2022/02/06 02:39:16:  dsfsdf sdfs df sdfsdfsd dssdf sdf: ");
            });


            //ui.label(l);


            egui::warn_if_debug_build(ui);
        });
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }

        self.configure_fonts(_ctx);
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn name(&self) -> &str {
        "eframe template"
    }
}





/*
#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![warn(clippy::all, rust_2018_idioms)]

*/

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = bellatrix::Bellatrix::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}



/*

use eframe::{epi};
use chrono;
use eframe::egui::{self, Button, Color32, CtxRef, FontDefinitions, FontFamily, Hyperlink, Label, Layout, Separator, TopBottomPanel};
use std::{borrow::Cow, iter::FromIterator};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    privateKey: String,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    force_buy_percent: f32,

    #[cfg_attr(feature = "persistence", serde(skip))]
    force_sell_percent: f32,

    #[cfg_attr(feature = "persistence", serde(skip))]
    auto_swap: bool,
}

impl TemplateApp {
    pub fn default() -> Self {
        Self {
            // Example stuff:
            privateKey: "".to_owned(),
            force_buy_percent: 0.0,
            force_sell_percent: 0.0,
            auto_swap: false,
        }
    }
}

impl epi::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self {
            privateKey,
            force_buy_percent,
            force_sell_percent,
            auto_swap
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Account Info");
            ui.label("");

            ui.vertical(|ui| {
                ui.label("Address: 0xa2F9....1b3E");

                ui.label("BNB   Balance: 1.233");
                ui.label("Token Balance: 1201212");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("API KEY: ");
                ui.text_edit_singleline(privateKey);

                if ui.button("Set API KEY").clicked() {
                    println!("dsfsdf");
                    println!("{:?}", chrono::offset::Local::now());
                }
            });

            ui.label("");
            ui.horizontal(|ui| {
                ui.label("PrivateKey: ");
                ui.text_edit_singleline(privateKey);

                if ui.button("Load account").clicked() {
                    println!("dsfsdf");
                    println!("{:?}", chrono::offset::Local::now());
                }
            });

            ui.horizontal(|ui| {
                ui.label("Contract: ");
                ui.text_edit_singleline(privateKey);

                if ui.button("Set Contract").clicked() {
                    println!("dsfsdf");
                    println!("{:?}", chrono::offset::Local::now());
                }
            });

            ui.label("");
            ui.horizontal(|ui| {
                ui.label("Swap From: ");
                ui.text_edit_singleline(privateKey);

                if ui.button("Set Contract").clicked() {
                    println!("dsfsdf");
                    println!("{:?}", chrono::offset::Local::now());
                }
            });
            ui.horizontal(|ui| {
                ui.label("Swap To: ");
                ui.text_edit_singleline(privateKey);

                if ui.button("Set Contract").clicked() {
                    println!("dsfsdf");
                    println!("{:?}", chrono::offset::Local::now());
                }
            });


            // These are equivalent:
            ui.label("");
            ui.heading("Swap configuration");


            // BUY
            ui.label("");
            ui.horizontal(|ui| {
                ui.checkbox(auto_swap, "Enable Auto Swap");
            });

            ui.horizontal(|ui| {
                ui.label("BUY");
                ui.add(egui::Slider::new(force_buy_percent, 0.0..=100.0));
                ui.label("%");
                if ui.button("Force Buy").clicked() {
                    *force_buy_percent += 1.0;
                }

                ui.label("\t\t");

                ui.horizontal(|ui| {
                    ui.label("SELL");
                    ui.label("");
                });

                ui.add(egui::Slider::new(force_sell_percent, 0.0..=100.0));
                ui.label("%");
                if ui.button("Force Sell").clicked() {
                    *force_sell_percent += 1.0;
                }
            });

            ui.label("");

            ui.heading("Log: ");
            ui.vertical(|ui| {
                ui.label("2022/02/06 02:39:16:  dsfsdf sdfs df sdfsdfsd dsf  sdf");
                ui.label("2022/02/06 02:39:16:  dsfsdf sdfs df sdfsdfs  sdf");
                ui.label("2022/02/06 02:39:16:  dsfsdf sdfs df sdfsdfsd");
                ui.label("2022/02/06 02:39:16:  dsfsdf sdfs df sdfsdfsd dsdsfsdff  sdf: ");
                ui.label("2022/02/06 02:39:16:  dsfsdf sdfs df sdfsdfsd dssdf sdf: ");
            });


            //ui.label(l);


            egui::warn_if_debug_build(ui);
        });
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }

        self.configure_fonts(_ctx);
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn name(&self) -> &str {
        "eframe template"
    }
}








-----------------------------------
#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = eframe_template::TemplateApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}




 */