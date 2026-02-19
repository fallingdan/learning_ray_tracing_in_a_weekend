use indicatif::ProgressBar;
mod color;
mod vec3;

fn main() {
    // Image
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    let progress_bar: ProgressBar = ProgressBar::new(image_height.into());

    for j in 0..image_height {
        progress_bar.inc(1);
        for i in 0..image_width {
            let r: f64 = (i as f64) / (image_width - 1) as f64;
            let g: f64 = (j as f64) / (image_height - 1) as f64;
            let b: f64 = 0.0;

            let int_r: i32 = (255.999 * r) as i32;
            let int_g: i32 = (255.999 * g) as i32;
            let int_b: i32 = (255.999 * b) as i32;

            print!("{} {} {}\n", int_r, int_g, int_b);
        }
    }
}
