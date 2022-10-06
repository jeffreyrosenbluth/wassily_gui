use wassily::prelude::*;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct WheelParams {
    pub hue1: f32,
    pub hue2: f32,
    pub hue3: f32,
    pub hue4: f32,
    pub sat1: f32,
    pub sat2: f32,
    pub sat3: f32,
    pub sat4: f32,
    pub light1: f32,
    pub light2: f32,
    pub light3: f32,
    pub sat_offset: f32,
    pub light_offset: f32,
}

impl WheelParams {
    pub fn new(
        hue1: f32,
        hue2: f32,
        hue3: f32,
        hue4: f32,
        sat1: f32,
        sat2: f32,
        sat3: f32,
        sat4: f32,
        light1: f32,
        light2: f32,
        light3: f32,
        sat_offset: f32,
        light_offset: f32,
    ) -> Self {
        Self {
            hue1,
            hue2,
            hue3,
            hue4,
            sat1,
            sat2,
            sat3,
            sat4,
            light1,
            light2,
            light3,
            sat_offset,
            light_offset,
        }
    }
}

pub fn map_circle(xs: &[f32], w: f32, x: f32) -> f32 {
    // Avoid negative numbers
    let mut x = x;
    let mut xs: Vec<f32> = xs.iter().map(|x| x + 1.0).collect();
    xs.push(xs[0] + 1.0);
    let mut mids = Vec::new();
    for y in xs.windows(2) {
        mids.push(0.5 * (y[0] + y[1]));
    }
    mids.push(mids[0] + 1.0);
    while x < mids[0] {
        x += 1.0
    }
    for i in 0..mids.len() - 1 {
        if (mids[i]..mids[i + 1]).contains(&x) {
            let v = xs[i + 1];
            return map_range(x, mids[i], mids[i + 1], v - w, v + w) % 1.0;
        }
    }
    panic!("x is not in any of the intervals");
}

pub fn map_interval(xs: &[f32], w: f32, x: f32) -> f32 {
    // Avoid negative numbers
    let xs: Vec<f32> = xs.to_vec();
    let mut mids = vec![0.0];
    for y in xs.windows(2) {
        mids.push(0.5 * (y[0] + y[1]));
    }
    mids.push(1.0);
    for i in 0..mids.len() - 1 {
        if (mids[i]..mids[i + 1]).contains(&x) {
            let v = xs[(i) % (mids.len() - 1)];
            return map_range(
                x,
                mids[i],
                mids[i + 1],
                (v - w).clamp(0.0, 1.0),
                (v + w).clamp(0.0, 1.0),
            );
        }
    }
    panic!("x is not in any of the intervals");
}

pub fn wheel(width: u32, height: u32, scale: f32, wheel_params: &WheelParams) -> Pixmap {
    let mut canvas = Canvas::with_scale(width, height, scale);
    let hues: Vec<f32> = vec![
        wheel_params.hue1,
        wheel_params.hue2,
        wheel_params.hue3,
        wheel_params.hue4,
    ];
    let sats: Vec<f32> = vec![
        wheel_params.sat1,
        wheel_params.sat2,
        wheel_params.sat3,
        wheel_params.sat4,
    ];
    let lights: Vec<f32> = vec![
        wheel_params.light1,
        wheel_params.light2,
        wheel_params.light3,
    ];
    for i in 0..width {
        let h = map_circle(&hues, 0.05, i as f32 / width as f32);
        let s = map_interval(
            &sats,
            0.1,
            (i as f32 / width as f32 + wheel_params.sat_offset) % 1.0,
        );
        let l = map_interval(
            &lights,
            0.1,
            (i as f32 / width as f32 + wheel_params.light_offset) % 1.0,
        );
        let hsluv = Hsluv::new(360.0 * h, 100.0 * s, 100.0 * l);
        let c1 = hsluv.to_color();
        ShapeBuilder::new()
            .line(pt(i, 0), pt(i, height))
            .stroke_color(c1)
            .build()
            .draw(&mut canvas);
    }
    canvas.pixmap
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn map_hue_test() {
        let hues = vec![0.1, 0.5, 0.8];
        let a = map_circle(&hues, 0.05, 0.15);
        let b = map_circle(&hues, 0.05, 0.2);
        let c = map_circle(&hues, 0.05, 0.6);
        let d = map_circle(&hues, 0.05, 0.81);
        dbg!(a, b, c, d);
    }

    #[test]
    fn map_sat_test() {
        let hues = vec![0.1, 0.5, 0.8];
        let a = map_interval(&hues, 0.05, 0.15);
        let b = map_interval(&hues, 0.05, 0.2);
        let c = map_interval(&hues, 0.05, 0.6);
        let d = map_interval(&hues, 0.05, 0.81);
        dbg!(a, b, c, d);
    }

    #[test]
    fn halton_test() {
        for i in 100..120 {
            dbg!(halton(i, 2));
        }
    }
}
