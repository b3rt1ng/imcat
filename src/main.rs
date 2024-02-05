use colored::Colorize;
use image::GenericImageView;
use std::env;
use term_size::dimensions;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let img = image::open(&args[1]).unwrap();
        if let Some((width, _)) = dimensions() {
            let aspect_ratio = img.dimensions().1 as f32 / img.dimensions().0 as f32;
            let resized_width = width as u32;
            let resized_height = (resized_width as f32 * aspect_ratio) as u32;
            let resized_img = img.thumbnail(resized_width, resized_height);
            let mut count = 0;
            for (_, _, pixel) in resized_img.pixels() {
                if count % resized_width == 0 {
                    println!("");
                }
                let red = pixel.0[0];
                let green = pixel.0[1];
                let blue = pixel.0[2];
                count += 1;
                print!("{}", " ".on_truecolor(red, green, blue));
            }
        } else {
            println!("Impossible de déterminer la taille du terminal.");
        }
    }
}
