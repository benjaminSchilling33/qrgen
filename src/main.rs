/*
*  qrgen main
*  SPDX-License-Identifier: MIT
*  Copyright (C) 2021 Benjamin Schilling
*/

// CLI Arguments
extern crate clap;
use clap::ArgMatches;
use clap::{App, Arg};

// QR Code Generation
extern crate qrcodegen;
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;

// File System
use std::fs::File;
use std::io::prelude::*;

const PATH: &str = "qrcode.svg";

/// Main Function
///
fn main() {
    let matches = App::new("qrgen")
        .version(clap::crate_version!())
        .author("Benjamin Schilling <benjamin.schilling33@gmail.com>")
        .about("Generate QR Code SVG files from command line")
        .subcommand(
            App::new("string")
                .about("Generate a simple QR code from a string.")
                .arg(
                    Arg::with_name("text")
                        .short("t")
                        .long("text")
                        .help("The text to encode")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("wifi")
                .about("Generate a Wifi QR Code.")
                .arg(
                    Arg::with_name("ssid")
                        .short("s")
                        .long("ssid")
                        .help("Wifi SSID")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("password")
                        .short("p")
                        .long("password")
                        .help("Wifi Password")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("encryption")
                        .short("e")
                        .long("encryption")
                        .help("Wifi Encryption Algorithm")
                        .default_value("WPA2")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .help("The path to the SVG output file")
                        .takes_value(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("string", Some(sub_m)) => string_subcommand(sub_m),
        ("wifi", Some(sub_m)) => wifi_subcommand(sub_m),
        _ => {
            eprintln!("Invalid subcommand.");
        }
    }
}

fn string_subcommand(matches: &ArgMatches<'_>) {
    let text = matches.value_of("text").unwrap();
    let mut path = PATH;
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

fn wifi_subcommand(matches: &ArgMatches<'_>) {
    let ssid = matches.value_of("ssid").unwrap();
    let password = matches.value_of("password").unwrap().replace(";", "\\;");
    let enc = matches.value_of("encryption").unwrap();
    let mut path = PATH;
    if matches.is_present("file") {
        path = matches.value_of("file").unwrap();
    }
    let text = format!("WIFI:T:{};S:{};P:{};;", &enc, &ssid, &password);
    let qr = QrCode::encode_text(&text, QrCodeEcc::Medium).unwrap();
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
