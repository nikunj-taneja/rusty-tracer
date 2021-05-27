fn main() {
    let image_height = 256;
    let image_width = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let r = f64::from(i)/f64::from(image_width-1);
            let g = f64::from(j)/f64::from(image_height-1);
            let b = f64::from(0.55);
            let ir = (255.999 * r) as i64;
            let ig = (255.999 * g) as i64;
            let ib = (255.999 * b) as i64;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
