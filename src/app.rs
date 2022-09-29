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
    print_scale: f32,
    // #[serde(skip)]
    art: Art,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            print_scale: 1.08,
            art: Art::new(
                2,
                100,
                100,
                5.0,
                5.0,
                2.0,
                1.0,
                1.0,
                4,
                std::f64::consts::TAU / 3.0,
                0.5,
            ),
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
            ui.add_space(10.0);
            ui.vertical_centered(|ui| ui.heading("Controls"));
            ui.add_space(10.0);
            ui.add(egui::Slider::new(&mut self.art.separatation, 0..=30).text("Separation"));
            ui.add_space(10.0);
            ui.add(egui::Slider::new(&mut self.art.starts, 1..=1500).text("Starts"));
            ui.add_space(10.0);
            ui.add(egui::Slider::new(&mut self.art.length, 1..=1000).text("Length"));
            ui.add_space(10.0);
            ui.add(egui::Slider::new(&mut self.art.step, 1.0..=100.0).text("Step"));
            ui.add_space(10.0);
            ui.add(egui::Slider::new(&mut self.art.pearl_size, 0.0..=100.0).text("Pearl Size"));
            ui.add_space(10.0);
            ui.add(
                egui::Slider::new(&mut self.art.stroke_weight, 0.0..=20.0).text("Stroke Weight"),
            );
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            ui.add(egui::Slider::new(&mut self.art.noise_scale, 0.1..=10.0).text("Noise Scale"));
            ui.add_space(10.0);
            ui.add(egui::Slider::new(&mut self.art.noise_factor, 0.0..=2.0).text("Noise Factor"));
            ui.add_space(10.0);
            ui.add(egui::Slider::new(&mut self.art.octaves, 1..=8).text("Octaves"));
            ui.add_space(10.0);
            ui.add(egui::Slider::new(&mut self.art.lacunarity, 0.0..=8.0).text("Lacunarity"));
            ui.add_space(10.0);
            ui.add(egui::Slider::new(&mut self.art.persistence, 0.0..=2.0).text("Persistence"));
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut self.print_scale, 0.36..=10.8).text("Print Scale"));
                if ui.button("Save").clicked() {
                    print(pixmap.width(), pixmap.height(), self.print_scale, &self.art);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| ui.heading("Wassily Gui Demo"));
            ui.add_space(20.0);
            egui::warn_if_debug_build(ui);
            let mut opt_texture: Option<egui::TextureHandle> = None;
            let texture: &egui::TextureHandle = opt_texture.get_or_insert_with(|| {
                ui.ctx()
                    .load_texture("wave", generate(pixmap), TextureFilter::default())
            });
            let img_size = texture.size_vec2();
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
    let mut img = pixmap.clone();
    let i = img.pixels().iter().map(|p| p.demultiply());
    ColorImage::from_rgba_unmultiplied(
        [pixmap.width() as usize, pixmap.height() as usize],
        img.data(),
    )
    // ColorImage::from_rgba_unmultiplied(
    //     [pixmap.width() as usize, pixmap.height() as usize],
    //     pixmap.data(),
    // )
}

fn print(width: u32, height: u32, scale: f32, art: &Art) {
    let pixmap = draw(width, height, scale, art);
    pixmap
        .save_png("./output/ff.png")
        .expect("Error saving image");
}
