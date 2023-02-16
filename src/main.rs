use image::{io::Reader as ImageReader, imageops::FilterType, Rgb};

const CELL_SIZE: u32 = 10;

fn main() {
    let src_img = ImageReader::
        open("Images\\Cat\\cat_img.jpg")
        .unwrap()
        .decode()
        .unwrap();

    let src_img = src_img.resize(
        src_img.width() - (src_img.width() % CELL_SIZE),
        src_img.height() - (src_img.height() % CELL_SIZE), 
        FilterType::Nearest
    );

    let src_img = src_img.as_rgb8().unwrap();

    let mut result_img = image::DynamicImage::new_rgb8(
        src_img.width() / CELL_SIZE, 
        src_img.height() / CELL_SIZE
    );

    let result_img = result_img.as_mut_rgb8().unwrap();

    for (x, y, result_pixel) in result_img.enumerate_pixels_mut() {
        let mut total_color: [u128; 3] = [0, 0, 0];

        for add_for_y in 0..(CELL_SIZE) {
            for add_for_x in 0..(CELL_SIZE) {
                let src_pixel = src_img.get_pixel(
                    x * CELL_SIZE + add_for_x, 
                    y * CELL_SIZE + add_for_y);
                let src_pixel_rgb = src_pixel.0;

                for i in 0..3 {
                    total_color[i] += src_pixel_rgb[i] as u128;
                }
            }
        }

        for i in 0..3 {
            total_color[i] /= CELL_SIZE.pow(2) as u128;
        }

        let total_color = [total_color[0] as u8, total_color[1] as u8, total_color[2] as u8];

        *result_pixel = Rgb(total_color);
    }

    result_img.save("Images\\Cat\\result_cat.jpg").unwrap();

    println!("Done");
}
