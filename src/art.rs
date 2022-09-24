use wassily::prelude::*;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Art {
    pub radial_middle_stop: f32,
}

impl Art {
    pub fn new(radial_middle_stop: f32) -> Self {
        Self { radial_middle_stop }
    }
}

pub fn draw(width: u32, height: u32, scale: f32, art: &Art) -> Pixmap {
    const EDGE: u32 = 250;
    const PAD: u32 = 100;
    let mut canvas = Canvas::with_scale(width, height, scale);

    let lg = LinearGradient::new(
        pt(0.0, 0.0),
        pt(0.0, canvas.h_f32()),
        vec![
            GradientStop::new(0.0, *WHITE),
            GradientStop::new(1.0, *BLACK),
        ],
        SpreadMode::Pad,
        Transform::identity(),
    )
    .unwrap();
    let linear = paint_shader(lg);

    let rg = RadialGradient::new(
        pt(canvas.w_f32() / 2.0, canvas.h_f32() / 2.0),
        pt(canvas.w_f32() / 2.0, canvas.h_f32() / 2.0),
        700.0,
        vec![
            GradientStop::new(0.0, *MAROON),
            GradientStop::new(art.radial_middle_stop, *ORANGE),
            GradientStop::new(1.0, *BLUE),
        ],
        SpreadMode::Pad,
        Transform::identity(),
    )
    .unwrap();
    let radial = paint_shader(rg);

    ShapeBuilder::new()
        .rect_xywh(pt(0, 0), pt(canvas.width, canvas.height))
        .fill_paint(&linear)
        .no_stroke()
        .build()
        .draw(&mut canvas);

    ShapeBuilder::new()
        .rect_xywh(
            pt(
                canvas.w_f32() / 2.0 - EDGE as f32 / 2.0,
                canvas.h_f32() / 2.0 - EDGE as f32 / 2.0,
            ),
            pt(EDGE, EDGE),
        )
        .fill_paint(&radial)
        .stroke_weight(4.0)
        .build()
        .draw(&mut canvas);

    ShapeBuilder::new()
        .rect_xywh(pt(PAD, PAD), pt(EDGE, EDGE))
        .fill_paint(&radial)
        .build()
        .draw(&mut canvas);
    ShapeBuilder::new()
        .rect_xywh(pt(canvas.width() - EDGE - PAD, PAD), pt(EDGE, EDGE))
        .fill_paint(&radial)
        .build()
        .draw(&mut canvas);
    ShapeBuilder::new()
        .rect_xywh(pt(PAD, canvas.width() - EDGE - PAD), pt(EDGE, EDGE))
        .fill_paint(&radial)
        .build()
        .draw(&mut canvas);
    ShapeBuilder::new()
        .rect_xywh(
            pt(canvas.width() - EDGE - PAD, canvas.height() - EDGE - PAD),
            pt(EDGE, EDGE),
        )
        .fill_paint(&radial)
        .build()
        .draw(&mut canvas);

    canvas.pixmap
}
