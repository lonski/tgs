extern crate num_cpus;
extern crate threadpool;
extern crate iron;
extern crate router;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod config;
mod thumb;
mod service;

use std::env;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;
use thumb::thumbnailze;
use thumb::create_thumbnail_filename;
use config::Config;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match Config::new(&args) {
        Ok(config) => {
            if config.start_service {
                service::start(config.service_port);
            } else {
                generate_thumbnails(config);
            }
        }
        Err(e) => {
            print!("Error when parsing arguments: {}", e);
            print!("\n\nUsage:");
            print!("\n\n  CLI:");
            print!("\n\t--images=path1,path2,..,pathN - list of images to generate thumbnails");
            print!("\n\t--prefix=<string> - thumbnail filename prefix (default: --prefix=thumb_)");
            print!("\n\t--size=<number> - thumbnail width in pixels (default: --size=200)");
            print!("\n\n  Web service:");
            print!("\n\t--start-service=[true|false] - if set to true,");
            print!("server with json api will be started (default: --start-service=false)");
            print!("\n\t--service-port=<number> - port on which web service ");
            print!("should be started (default: --service-port=8080");
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
