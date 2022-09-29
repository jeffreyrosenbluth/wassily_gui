use wassily::prelude::*;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Art {
    pub separatation: u32,
    pub starts: u32,
    pub length: u32,
    pub step: f32,
    pub pearl_size: f32,
    pub stroke_weight: f32,
    pub noise_scale: f32,
    pub noise_factor: f32,
    pub octaves: usize,
    pub lacunarity: f64,
    pub persistence: f64,
}

impl Art {
    pub fn new(
        separatation: u32,
        starts: u32,
        length: u32,
        step: f32,
        pearl_size: f32,
        stroke_weight: f32,
        noise_scale: f32,
        noise_factor: f32,
        octaves: usize,
        lacunarity: f64,
        persistence: f64,
    ) -> Self {
        Self {
            separatation,
            starts,
            length,
            step,
            pearl_size,
            stroke_weight,
            noise_scale,
            noise_factor,
            octaves,
            lacunarity,
            persistence,
        }
    }
}

pub fn draw(width: u32, height: u32, scale: f32, art: &Art) -> Pixmap {
    let mut canvas = Canvas::with_scale(width, height, scale);
    let bg_color = *CHOCOLATE;
    canvas.fill(bg_color);
    let sep = art.separatation;
    let step = art.step;
    let max_length = art.length;
    let nf = Fbm::<Perlin>::new(0)
        .set_octaves(art.octaves)
        .set_lacunarity(art.lacunarity)
        .set_persistence(art.persistence);
    // let nf = Curl::new(nf);
    let opts = NoiseOpts::with_wh(canvas.width(), canvas.height())
        .scales(art.noise_scale)
        .factor(art.noise_factor);
    let grid: FlowGrid;
    if sep > 0 {
        let x = 1 + canvas.width() / sep;
        let y = 1 + canvas.height() / sep;
        grid = Matrix::new(x, y, vec![vec![]; (x * y) as usize]);
    } else {
        grid = Matrix::new(0 as u32, 0, vec![]);
    }
    let mut flow = FlowField {
        grid,
        noise_function: Box::new(nf),
        noise_opts: opts,
        sepration: sep as f32,
        step_size: step,
        width: canvas.width(),
        height: canvas.height(),
        max_length,
        obstacles: vec![],
    };
    let starts = halton_23(canvas.width(), canvas.height(), art.starts, 1231);
    let mut rng = SmallRng::seed_from_u64(0);
    for p in starts.iter() {
        let pts = flow.curve(p.x, p.y);
        if pts.len() > 2 {
            ShapeBuilder::new()
                .points(&pts)
                .no_fill()
                .stroke_color((*SADDLEBROWN).shade(0.75))
                .stroke_weight(art.stroke_weight)
                .line_cap(LineCap::Round)
                .build()
                .draw(&mut canvas);
            let mut colr = *FORESTGREEN;
            colr = colr.tint(rng.gen_range(0.0..0.5));
            colr.set_alpha(0.75);
            let n: f32 = rng.sample(StandardNormal);
            let size: f32 = art.pearl_size * (n + 1.0).abs();
            ShapeBuilder::new()
                .pearl(*p, size, size, 8, 5, &mut rng)
                .no_stroke()
                .fill_color(colr)
                .build()
                .draw(&mut canvas);
            let q = pts[pts.len() - 1];
            colr = *GOLDENROD;
            colr = colr.shade(rng.gen_range(0.0..0.5));
            colr.set_alpha(0.75);
            ShapeBuilder::new()
                .pearl(q, size, size, 8, 5, &mut rng)
                .no_stroke()
                .fill_color(colr)
                .build()
                .draw(&mut canvas);
        }
    }
    canvas.pixmap
}
