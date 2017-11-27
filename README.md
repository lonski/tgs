# Thumbnailator
Utility for generating image thumbnails.

## Building
Enter repository root and type:
```
cargo build && cargo run
```

## Usage

### Commandline interface
Provide two arguments: first image to be resized, second: thumbnail output path:
```
./thumnailator <image-to-be-resized-path> <output-thumbnail-path>
```

## TODO:
List of things to be implemented:
* Support for multiple images
* Generating thumbnails concurrently
* Web service that accepts list of images to generate thumbnails via json api
