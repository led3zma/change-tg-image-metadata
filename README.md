# change-tg-image-metadata
This is a small CLI utility used to modify the creation date metadata of downloaded images from Telegram when exporting files from a chat. Simply execute the program and pass the directory that contains the target images as an argument.

## Motivation
When you export images from a Telegram chat, the creation date of every file doesn't match with the message sending date that should be. Even if the images were sent, let's say, some on April 25th and some on August 24th, the creation date of every image will be the same date of the moment you do the chat export. In my case, I want to export the images to a cloud storage (ie. Google Photos), so this detail will mess up the correct date order of every image. 

This program aims to help fix this, taking advantage of the file format of the exported images, which is `photo_num@dd-mm-yyyy_hh-mm-ss`. Doing some transformations of the filename string, it is possible to modify the file creation date metadata using it.

You may wonder why it is written in Rust if its simple enough to be written in something like Python (which I could probably do in a future). This is part of my learning process of Rust, so it is perfectly possible to write this with other tools without any problem or needing an specific requierment or reason.

## Instalation
You can download a precompiled binary of the program in the Releases page (for the moment, only gnu-linux and musl-linux binaries are available). Place it in any directory, decompress it with `tar -xzvf` and just run it: `./telegram_image_metadata`.

If you're a Rust developer, you can clone this repository and run the project with `cargo run`.

## Usage
Get help message:
```
~$ ./telegram_image_metadata --help
Usage: telegram_image_metadata <PATH>

Arguments:
  <PATH>  Path where the images are located

Options:
  -h, --help     Print help
  -V, --version  Print version
```
Run the program to process a directory with images (it will only process the images with the [correct format](#motivation) ignoring thumbnails):
```
~$ ./telegram_image_metadata /path/to/dir
```
