/*
*  qrgen main
*  SPDX-License-Identifier: MIT
*  Copyright (C) 2021 Benjamin Schilling
*/

// CLI Arguments
extern crate clap;
use clap::{App, Arg};

// QR Code Generation
extern crate qrcodegen;
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;

// File System
use std::fs::File;
use std::io::prelude::*;

/// Main Function
/// 
fn main() {
    let matches = App::new("qrgen")
        .version("1.0")
        .author("Benjamin Schilling <benjamin.schilling33@gmail.com>")
        .about("Generate QR Code SVG files from command line")
        .arg(
            Arg::with_name("text")
                .short("t")
                .long("text")
                .help("The text to encode")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .help("The path to the SVG output file")
                .takes_value(true),
        )
        .get_matches();

    let text = matches.value_of("text").unwrap();
    let mut path = "qrcode.svg";
    if matches.is_present("file") {
        path = matches.value_of("file").unwrap();
    }
    let qr = QrCode::encode_text(text, QrCodeEcc::Medium).unwrap();
    let svg = qr.to_svg_string(4);
    
    let mut file = match File::create(path) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Failed creating file \"qrcode.svg\": {}", e);
            std::process::exit(0x0001)
        }
    };
    match file.write_all(svg.as_bytes()) {
        Ok(_o) => std::process::exit(0x0000),
        Err(e) => {
            eprintln!("Failed writing file: {}", e);
            std::process::exit(0x0001)
        }
    };
}
