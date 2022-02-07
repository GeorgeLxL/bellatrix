use eframe::egui::{TopBottomPanel, CtxRef, Layout, Button, Color32, Label, Hyperlink, FontDefinitions, FontFamily, FontData, TextStyle, Separator, Ui};
use eframe::egui;
use std::iter::FromIterator;
use std::borrow::Cow;


pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);


pub struct BotLog {
    date: String,
    text: String,
    tx_hash: String
}

pub struct Bellatrix {
    pub logs: Vec<BotLog>,

    // Example stuff:
    pub privateKey: String,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    pub force_buy_percent: f32,

    #[cfg_attr(feature = "persistence", serde(skip))]
    pub force_sell_percent: f32,

    #[cfg_attr(feature = "persistence", serde(skip))]
    pub auto_swap: bool,
}

impl Bellatrix {
    pub fn default() -> Self {
        Self {
            // Example stuff:
            logs: vec![],
            privateKey: "".to_owned(),
            force_buy_percent: 0.0,
            force_sell_percent: 0.0,
            auto_swap: false,
        }
    }

    pub fn new() -> Bellatrix {
        let iter = (0..20).map(|a| BotLog {
            date: format!("date{}", a),
            text: format!("{}", a),
            tx_hash: format!("{}", a),
        });
        Bellatrix {
            logs: Vec::from_iter(iter),
            privateKey: "".to_string(),
            force_buy_percent: 0.0,
            force_sell_percent: 0.0,
            auto_swap: false
        }
    }

    pub fn configure_fonts(&self, ctx: &CtxRef) {

        /*
        let fd = FontData::from();
        let mut font_def = FontDefinitions::default();

        font_def.font_data.insert(
            "MesloLGS".to_string(),
            fd,
        );
        */

        /*
        font_def.font_data.insert(
            "MesloLGS".to_string(),
            Cow::Borrowed(include_bytes!("../MesloLGS_NF_Regular.ttf")),
        );

        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Heading,
            (FontFamily::Proportional, 35.),
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Body,
            (FontFamily::Proportional, 20.),
        );
        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "MesloLGS".to_string());
        ctx.set_fonts(font_def);
  */
    }

    pub fn render_new_log(&self, ui: &mut eframe::egui::Ui) {
        for a in &self.logs {


            ui.horizontal(|ui| {
                let title = format!("{}: {}", a.date, "Buy 12323 TKM - 0.23 BNB");
                ui.colored_label(WHITE, title);

                ui.style_mut().visuals.hyperlink_color = CYAN;
                ui.add_space(PADDING);
                ui.with_layout(Layout::right_to_left(), |ui| {
                    ui.add(Hyperlink::new(&a.text).text("See Tx On Explorer ⤴"));
                });

            });
        }
    }


}