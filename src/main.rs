mod config;
mod thumb;

use std::env;
use thumb::thumbnailze;
use thumb::create_thumbnail_filename;
use config::Config;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match Config::new(&args) {
        Ok(config) => {
            if let Err(e) = generate_thumbnails(config) {
                println!("Failed to generate thumbnails: {}", e);
            }
        }
        Err(e) => {
            println!("Error when parsing arguments: {}", e);
            println!("\nUsage:");
            println!("\t--images=path1,path2,..,pathN - list of images to generate thumbnails");
            println!("\t--prefix=<string> - thumbnail filename prefix (default: --prefix=thumb_)");
            println!("\t--size=<number> - thumbnail width in pixels (default: --size=200)");
        }
    }
}

fn generate_thumbnails(config: Config) -> Result<(), String> {
    for image in config.images {
        let thumb_fn = create_thumbnail_filename(&image, &config.prefix);
        println!("Generating thumbnail: <{}>", &thumb_fn);
        thumbnailze(image, thumb_fn, config.size)?;
    }
    Ok(())
}
