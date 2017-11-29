extern crate num_cpus;
extern crate threadpool;

mod config;
mod thumb;

use std::env;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;
use thumb::thumbnailze;
use thumb::create_thumbnail_filename;
use config::Config;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match Config::new(&args) {
        Ok(config) => generate_thumbnails(config),
        Err(e) => {
            println!("Error when parsing arguments: {}", e);
            println!("\nUsage:");
            println!("\t--images=path1,path2,..,pathN - list of images to generate thumbnails");
            println!("\t--prefix=<string> - thumbnail filename prefix (default: --prefix=thumb_)");
            println!("\t--size=<number> - thumbnail width in pixels (default: --size=200)");
        }
    }
}

fn generate_thumbnails(config: Config) {
    let num_images = config.images.len();
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    for image in config.images {
        let thumb_fn = create_thumbnail_filename(&image, &config.prefix);
        let size = config.size;
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
