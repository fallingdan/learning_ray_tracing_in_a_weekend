use std::io;

use indicatif::{ProgressBar, ProgressStyle};

use crate::color::Color;
mod color;
mod ray;
mod vec3;

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;

    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    let viewport_height: f64 = 2.0;
    let viewport_width: f64 =
        viewport_height * (image_width as f64 / image_height as f64);

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    let style = ProgressStyle::with_template("{msg}").unwrap();
    let progress_bar: ProgressBar =
        ProgressBar::new(image_height.into()).with_style(style);

    let mut stdout = io::stdout();

    for j in 0..image_height {
        progress_bar.inc(1);

        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );
            color::write_color(&mut stdout, &pixel_color);
        }
    }

    progress_bar.finish_with_message("\nDone\n");
}
