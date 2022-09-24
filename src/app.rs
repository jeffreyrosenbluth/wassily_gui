use crate::art::*;
use egui::{ColorImage, TextureFilter};
use wassily::prelude::Pixmap;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
// if we add new fields, give them default values when deserializing old state

pub struct TemplateApp {
    label: String,
    #[serde(skip)]
    art: Art,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            art: Art::new(0.4),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // let Self {
        //     label,
        //     value,
        //     mut art,
        // } = self;
        // let mut art = self.art.clone();
        let pixmap = draw(WIDTH, HEIGHT, 1.0, &self.art);

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Close");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(
                egui::Slider::new(&mut self.art.radial_middle_stop, 0.0..=1.0)
                    .text("Middle Gradient Stop"),
            );
            ui.add_space(20.0);
            if ui.button("Save").clicked() {
                print(pixmap.width(), pixmap.height(), 1.08, &self.art);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Wassily Gui App");
            ui.add_space(20.0);
            egui::warn_if_debug_build(ui);
            let mut opt_texture: Option<egui::TextureHandle> = None;
            let texture: &egui::TextureHandle = opt_texture.get_or_insert_with(|| {
                ui.ctx()
                    .load_texture("wave", generate(pixmap), TextureFilter::default())
            });
            let img_size = 1.0 * texture.size_vec2();
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.add_sized(
                    [WIDTH as f32, HEIGHT as f32],
                    egui::Image::new(texture, img_size),
                );
            });
        });
    }
}

fn generate(pixmap: Pixmap) -> ColorImage {
    ColorImage::from_rgba_unmultiplied(
        [pixmap.width() as usize, pixmap.height() as usize],
        pixmap.data(),
    )
}

fn print(width: u32, height: u32, scale: f32, art: &Art) {
    let pixmap = draw(width, height, scale, art);
    pixmap.save_png("./output/grad.png");
}
