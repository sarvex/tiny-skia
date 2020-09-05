use tiny_skia::*;

fn main() {
    let mut pixmap = Pixmap::new(1000, 1000).unwrap();

    let now = std::time::Instant::now();

    let paint = Paint::default()
        .set_shader(LinearGradient::new(
            Point::from_xy(100.0, 100.0),
            Point::from_xy(900.0, 900.0),
            vec![
                GradientStop::new(0.0, Color::from_rgba8(50, 127, 150, 200)),
                GradientStop::new(1.0, Color::from_rgba8(220, 140, 75, 180)),
            ],
            SpreadMode::Pad,
            Transform::identity(),
        ).unwrap());

    let mut pb = PathBuilder::new();
    pb.move_to(60.0, 60.0);
    pb.line_to(160.0, 940.0);
    pb.cubic_to(380.0, 840.0, 660.0, 800.0, 940.0, 800.0);
    pb.cubic_to(740.0, 460.0, 440.0, 160.0, 60.0, 60.0);
    pb.close();
    let path = pb.finish().unwrap();

    pixmap.fill_path(&path, &paint);

    println!("Rendered in {:.2}ms", now.elapsed().as_micros() as f64 / 1000.0);

    pixmap.save_png("image.png").unwrap();
}
