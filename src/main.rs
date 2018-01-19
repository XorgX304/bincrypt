extern crate base64;
extern crate clap;
#[macro_use]
extern crate failure;
extern crate rust_sodium;

mod internal;

use clap::{App, Arg};
use internal::{decrypt_file, encrypt_file, generate_key, transform_key};

fn main() {
    let matches = App::new("bincrypt")
        .version("0.1.1")
        .author("frk <hazefrk+dev@gmail.com>")
        .about("Encrypt a file using XSalsa20-Poly1305!")
        .arg(
            Arg::with_name("file_in")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("FILE intput")
                .required(true),
        )
        .arg(
            Arg::with_name("file_out")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("FILE output")
                .required(true),
        )
        .arg(
            Arg::with_name("encryption_key")
                .short("k")
                .long("key")
                .value_name("KEY")
                .help("Base64 encoded encryption key")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("decrypt")
                .short("d")
                .long("decrypt")
                .help("Switch to decrypting"),
        )
        .get_matches();

    let key_opt = matches.value_of("encryption_key").and_then(transform_key);
    rust_sodium::init();

    let file_in = matches.value_of("file_in").unwrap();
    let file_out = matches.value_of("file_out").unwrap();

    if matches.is_present("decrypt") {
        if let Some(key) = key_opt {
            if let Err(err) = decrypt_file(file_in, file_out, &key) {
                println!("Error: {}", err);
            }
        } else {
            println!("Error: Need valid encryption key to decrypt!")
        }
    } else {
        let key = key_opt.unwrap_or_else(generate_key);

        if let Err(err) = encrypt_file(file_in, file_out, &key) {
            println!("Error: {}", err);
        }
    }
}
