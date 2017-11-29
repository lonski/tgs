# Thumbnailator

[![Build Status](https://travis-ci.org/lonski/thumbnailator.svg?branch=master)](https://travis-ci.org/lonski/thumbnailator)


Utility for generating image thumbnails.

## Building
Enter repository root and type:
```
cargo build && cargo run
```

## Usage

### Commandline interface

Parameters:
```
--images=path1,path2,..,pathN - list of images to generate thumbnails
--prefix=<string> - thumbnail filename prefix (default: --prefix=thumb_)
--size=<number> - thumbnail width in pixels (default: --size=200)
```

Example:
```
./thumbnailator --images=/tmp/img1.jpg,/tmp/img2.png
```

## TODO:
List of things to be implemented:
* Web service that accepts list of images to generate thumbnails via json api
