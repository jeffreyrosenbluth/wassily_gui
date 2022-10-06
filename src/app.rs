use crate::art::*;
use egui::{ColorImage, TextureFilter};
use wassily::prelude::*;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
// if we add new fields, give them default values when deserializing old state

pub struct TemplateApp {
    print_scale: f32,
    // #[serde(skip)]
    wheel_params: WheelParams,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            print_scale: 1.08,
            wheel_params: WheelParams {
                hue1: 0.2,
                hue2: 0.4,
                hue3: 0.6,
                hue4: 0.8,
                sat1: 0.25,
                sat2: 0.5,
                sat3: 0.75,
                sat4: 1.0,
                light1: 0.25,
                light2: 0.5,
                light3: 0.75,
                sat_offset: 0.0,
                light_offset: 0.0,
            },
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

fn rando() -> Vec<f32> {
    let mut rng = thread_rng();
    vec![
        // hues
        rng.gen_range(0.0..0.25),
        rng.gen_range(0.35..0.60),
        rng.gen_range(0.6..0.77),
        rng.gen_range(0.82..1.0),
        // sats
        rng.gen_range(0.2..0.3),
        rng.gen_range(0.3..0.5),
        rng.gen_range(0.5..0.7),
        rng.gen_range(0.7..0.9),
        // lights
        rng.gen_range(0.2..0.4),
        rng.gen_range(0.4..0.6),
        rng.gen_range(0.6..0.85),
        rng.gen(),
        rng.gen(),
    ]
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let pixmap = wheel(WIDTH, HEIGHT, 1.0, &self.wheel_params);
        // let pixmap = draw(WIDTH, HEIGHT, 1.0, &self.art);
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
            ui.add(
                egui::Slider::new(&mut self.wheel_params.hue1, 0.0..=1.0)
                    .text("Hue 1")
                    .step_by(0.01),
            );
            ui.add_space(10.0);
            ui.add(
                egui::Slider::new(&mut self.wheel_params.hue2, 0.0..=1.0)
                    .text("Hue 2")
                    .step_by(0.01),
            );
            ui.add_space(10.0);
            ui.add(
                egui::Slider::new(&mut self.wheel_params.hue3, 0.0..=1.0)
                    .text("Hue 3")
                    .step_by(0.01),
            );
            ui.add_space(10.0);
            ui.add(
                egui::Slider::new(&mut self.wheel_params.hue4, 0.0..=1.0)
                    .text("Hue 4")
                    .step_by(0.01),
            );
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            ui.add(
                egui::Slider::new(&mut self.wheel_params.sat1, 0.0..=1.0)
                    .text("Saturation 1")
                    .step_by(0.01),
            );
            ui.add_space(10.0);
            ui.add(
                egui::Slider::new(&mut self.wheel_params.sat2, 0.0..=1.0)
                    .text("Saturation 2")
                    .step_by(0.01),
            );
            ui.add_space(10.0);
            ui.add(
                egui::Slider::new(&mut self.wheel_params.sat3, 0.0..=1.0)
                    .text("Saturation 3")
                    .step_by(0.01),
            );
            ui.add_space(10.0);
            ui.add(
                egui::Slider::new(&mut self.wheel_params.sat4, 0.0..=1.0)
                    .text("Saturation 4")
                    .step_by(0.01),
            );
            ui.add_space(10.0);
            ui.add(
                egui::Slider::new(&mut self.wheel_params.sat_offset, 0.0..=1.0)
                    .text("Saturation Offset")
                    .step_by(0.01),
            );
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            ui.add(
                egui::Slider::new(&mut self.wheel_params.light1, 0.0..=1.0)
                    .text("Light 1")
                    .step_by(0.01),
            );
            ui.add_space(10.0);
            ui.add(
                egui::Slider::new(&mut self.wheel_params.light2, 0.0..=1.0)
                    .text("Light 2")
                    .step_by(0.01),
            );
            ui.add_space(10.0);
            ui.add(
                egui::Slider::new(&mut self.wheel_params.light3, 0.0..=1.0)
                    .text("Light 3")
                    .step_by(0.01),
            );
            ui.add_space(10.0);
            ui.add(
                egui::Slider::new(&mut self.wheel_params.light_offset, 0.0..=1.0)
                    .text("Light Offset")
                    .step_by(0.01),
            );
            ui.separator();
            ui.add_space(10.0);
            ui.vertical_centered(|ui| {
                if ui.button("Random").clicked() {
                    let xs = rando();
                    self.wheel_params.hue1 = xs[0];
                    self.wheel_params.hue2 = xs[1];
                    self.wheel_params.hue3 = xs[2];
                    self.wheel_params.hue4 = xs[3];
                    self.wheel_params.sat1 = xs[4];
                    self.wheel_params.sat2 = xs[5];
                    self.wheel_params.sat3 = xs[6];
                    self.wheel_params.sat4 = xs[7];
                    self.wheel_params.light1 = xs[8];
                    self.wheel_params.light2 = xs[9];
                    self.wheel_params.light3 = xs[10];
                    self.wheel_params.sat_offset = xs[11];
                    self.wheel_params.light_offset = xs[12];
                }
            });
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut self.print_scale, 0.36..=10.8).text("Print Scale"));
                if ui.button("Save").clicked() {
                    print(
                        pixmap.width(),
                        pixmap.height(),
                        self.print_scale,
                        &self.wheel_params,
                    );
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
    ColorImage::from_rgba_unmultiplied(
        [pixmap.width() as usize, pixmap.height() as usize],
        pixmap.data(),
    )
}

fn print(width: u32, height: u32, scale: f32, art: &WheelParams) {
    let pixmap = wheel(width, height, scale, art);
    pixmap
        .save_png("./output/color.png")
        .expect("Error saving image");
}
