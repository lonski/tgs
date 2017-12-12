extern crate image;

use self::image::DynamicImage;
use self::image::GenericImage;
use self::image::FilterType;
use std::path::Path;
use std::fs::File;

/// # Generates thumbnails from given files in parallel.
///
/// Arguments:
/// * files - list of images to resize
/// * prefix - thumblail filename prefix (prefix_<original-filename>)
/// * size - width of genrated thumbnails
pub fn generate_thumbnails(files: Vec<String>, prefix: String, size: u32) {
    let num_images = files.len();
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    for image in files {
        let thumb_fn = create_thumbnail_filename(&image, &prefix);
        let size = size;
        let tx = tx.clone();

        pool.execute(move || {
            println!("Generating thumbnail: <{}>", &thumb_fn);
            if let Err(e) = thumbnailze(&image, &thumb_fn, size) {
                println!(
                    "Failed to generate thumbnail from image <{}>: {}",
                    &image,
                    e
                );
            };
            tx.send(()).unwrap();
        });
    }

    for _ in 0..num_images {
        rx.recv().unwrap();
    }
}

fn thumbnailze(image_fn: &String, thumb_fn: &String, size: u32) -> Result<(), String> {
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

fn create_thumbnail_filename(image_fn: &str, prefix: &str) -> String {
    let image_path = Path::new(image_fn);
    let dir = image_path.parent().unwrap_or(Path::new(""));
    let filename = format!(
        "{}{}",
        prefix,
        image_path.file_name().unwrap().to_str().unwrap()
    );
    String::from(dir.join(filename).to_str().unwrap_or("no-name.png"))
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

#[cfg(test)]
mod tests {

    use ::*;

    #[test]
    fn should_generate_thumb_path() {
        assert_eq!(
            create_thumbnail_filename("/tmp/lul/image.png", "thumb_"),
            "/tmp/lul/thumb_image.png"
        );
        assert_eq!(
            create_thumbnail_filename("image.png", "thumb_"),
            "thumb_image.png"
        );
        assert_eq!(
            create_thumbnail_filename("/image.png", "thumb_"),
            "/thumb_image.png"
        );
    }
}
