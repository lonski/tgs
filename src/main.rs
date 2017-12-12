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
                generate_thumbnails(config.images, config.prefix, config.size);
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
            print!("\n\t--start-service - server with json api will be started");
            print!("\n\t--service-port=<number> - port on which web service ");
            print!("should be started (default: --service-port=8080");
        }
    }
}
