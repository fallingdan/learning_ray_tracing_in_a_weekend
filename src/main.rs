use std::io;

use indicatif::{ProgressBar, ProgressStyle};

use crate::color::Color;
mod color;
mod ray;
mod vec3;

fn main() {
    // Image
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    let style = ProgressStyle::with_template("{msg}").unwrap();
    let progress_bar: ProgressBar = ProgressBar::new(image_height.into()).with_style(style);

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
