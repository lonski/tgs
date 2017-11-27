extern crate image;

use image::DynamicImage;
use image::GenericImage;
use image::FilterType;
use std::path::Path;
use std::fs::File;

pub fn thumbnailze(image_fn: String, thumb_fn: String, size: u32) -> Result<(), String> {
    let img = image::open(&Path::new(&image_fn));
    match img {
        Ok(img) => {
            let thumnail = create_thumbnail(&img, size);
            save_image(&thumnail, &Path::new(&thumb_fn))?;
        }
        Err(e) => return Err(e.to_string()),
    };
    Ok(())
}

fn create_thumbnail(image: &DynamicImage, size: u32) -> DynamicImage {
    let (w, h) = image.dimensions();
    let ratio = h as f32 / w as f32;
    let new_width = size;
    let new_height = (new_width as f32 * ratio) as u32;

    image.resize(new_width, new_height, FilterType::Lanczos3)
}

fn save_image(image: &DynamicImage, path: &Path) -> Result<(), String> {
    println!("Saving image to <{:?}>", path);
    let f_out = File::create(path);
    match f_out {
        Ok(mut f) => {
            match image.save(&mut f, image::PNG) {
                Ok(_) => return Ok(()),
                Err(e) => return Err(e.to_string()),
            }
        }
        Err(e) => return Err(e.to_string()),
    }
}
