# Thumbnail Generation Service (TGS)

[![Build Status](https://travis-ci.org/lonski/tgs.svg?branch=master)](https://travis-ci.org/lonski/tgs)


Utility for generating image thumbnails. Can be started in service mode as server accepting json requests.

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
./tgs --images=/tmp/img1.jpg,/tmp/img2.png
```

### Service mode

Tgs can be run as server accepting json requests. 
In service mode following route is available:

Starting service:
```
./tgs --start-service --service-port=8080
```

Service port parameter is optional. Default port is 8080.

#### Routes

 * POST /generate 
   * format: json
   * fields:
     * images (required) - input image filenames as list of strings
     * prefix (optional) - thumbnails prefix as string (default: "thumb_")
     * size (optional) - thumbnails width as int (default: 200)

#### Example request
```json
{
  "images": [
    "/home/foo/pictures/pic1.jpg",
    "/tmp/images/pic2.png"
  ],
  "prefix": "thumb_",
  "size": 200
}
```

## TODO:
List of things to be implemented:
* Responses as json
* Accept directory as input (find all images automatically)
* Improve error handling
