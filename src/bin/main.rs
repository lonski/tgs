extern crate thumbnailator;

use std::env;
use thumbnailator::thumbnailze;

fn main() {
    let mut args = env::args();
    if args.len() != 3 {
        println!("Wrong number of arguments. \nUsage: ./thumbnailator <image-file> <thumb-file>");
        std::process::exit(1);
    }
    args.next();

    let image_fn = args.next().unwrap();
    let thumb_fn = args.next().unwrap();

    thumbnailze(image_fn, thumb_fn, 200).unwrap();
}
