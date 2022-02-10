use image::{self, imageops, GenericImage, GenericImageView, Rgba};

pub fn make_ichimatu() {
    let white = image::Rgb::<u8>([255, 255, 255]);
    let red = image::Rgb::<u8>([255, 90, 90]);

    let w = 64;
    let draw = |x, y| {
        let (xi, yi) = (x / w, y / w);
        match (xi % 2, yi % 2) {
            (0, 0) => white,
            (1, 0) => red,
            (0, 1) => red,
            (1, 1) => white,
            (_, _) => panic!("error"),
        }
    };
    let img = image::ImageBuffer::from_fn(512, 512, draw);
    img.save("ichimatu.png").unwrap();
}

pub fn make_thumb(fname: String) {
    let size = 128;
    let outfile = format!("crop-{}", &fname);
    let mut img = image::open(&fname).expect("Cannot read file.");
    let dim = img.dimensions();
    let w = if dim.0 > dim.1 { dim.1 } else { dim.0 };
    let mut img2 = imageops::crop(&mut img, (dim.0 - w) / 2, (dim.1 - w) / 2, w, w).to_image();
    let img3 = imageops::resize(&mut img2, size, size, imageops::Lanczos3);
    img3.save(outfile).unwrap();
}

pub fn filter_negapogi(fname: String) {
    let outfile = format!("nega-{}", &fname);
    let mut img = image::open(&fname).expect("Cannot read file.");
    let (w, h) = img.dimensions();
    for y in 0..h {
        for x in 0..w {
            let c: Rgba<u8> = img.get_pixel(x, y);
            let c = Rgba([255 - c[0], 255 - c[1], 255 - c[2], c[3]]);
            img.put_pixel(x, y, c);
        }
    }
    img.save(outfile).unwrap();
}
