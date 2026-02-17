use indicatif::ProgressBar;
mod vector3;

fn main() {
    // Image
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    let progressBar: ProgressBar = ProgressBar::new(image_height.into());

    for j in 0..image_height {
        progressBar.inc(1);
        for i in 0..image_width {
            let r: f64 = (i as f64) / (image_width - 1) as f64;
            let g: f64 = (j as f64) / (image_height - 1) as f64;
            let b: f64 = 0.0;

            let intR: i32 = (255.999 * r) as i32;
            let intG: i32 = (255.999 * g) as i32;
            let intB: i32 = (255.999 * b) as i32;

            print!("{} {} {}\n", intR, intG, intB);
        }
    }
}
