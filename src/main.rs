extern crate config;
extern crate serde_json;
extern crate serde_yaml;
#[macro_use]
extern crate clap;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use clap::{App, Arg};

// Function used when writing to file
// Takes a file name and set of data, returns nothing
fn writer(file_name: &str, output_data: &[u8]) {
    let mut file = File::create(file_name).unwrap();
    file.write_all(output_data).unwrap();
}

fn main() {
    // Main body of valid Clap commands
    let matches = App::new("Trope")
        .version(crate_version!())
        .author(crate_authors!("/n"))
        .about("Utility for merging YAML and JSON files")
        .arg(
            Arg::with_name("output")
                .short("O")
                .long("output")
                .help("Sets the path to the output file, or creates one if it doesn't exist")
                .takes_value(true)
                .required(true),
        ).arg(
            Arg::with_name("input")
                .short("I")
                .long("input")
                .value_name("FILE")
                .help("Specifies file(s) to merge")
                .takes_value(true)
                .multiple(true)
                .required(true),
        ).arg(
            Arg::with_name("env_var")
                .short("e")
                .long("env")
                .help("Merges environmental variables with prefix: TROPE"),
        ).arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("When set, displays merged config rather than printing to file"),
        ).get_matches();
    // Uses config_rs's default type for holding the various .yml, .json, etc fed into this program  
    let mut settings = config::Config::default();

    // Iterates through and merges files passed via the input option
    let in_iterator = matches.values_of("input");
    for f in in_iterator.unwrap() {
        settings
            .merge(config::File::with_name(f))
            .expect("Error in merging given file(s)");
    }

    // Merges any env vars provided by the user
    if matches.is_present("env_var") {
        settings
            .merge(config::Environment::with_prefix("TROPE"))
            .expect("Error in merging given environmental variable");
    }

    // Takes the file or path provided by the user and
    // determines the correct serialization to implement
    if let Some(o) = matches.value_of("output") {
        let file_path = Path::new(o);
        let extension_type = match file_path.extension() { // Isolates the file extension and converts it from os_str into a useable &str
            None => panic!("No output file extension detected"),
            Some(os_str) => match os_str.to_str() {
                Some("yaml") => "yaml",
                Some("yml") => "yaml",
                Some("json") => "json",
                _ => panic!("Improper file extension"),
            },
        };

        // Logic for determining proper serialization
        if extension_type == "yaml" {
            let yaml_doc = settings
                .try_into::<serde_yaml::Value>()
                .expect("Error in serializing merged configs");
            let yaml =
                serde_yaml::to_string(&yaml_doc).expect("Error in moving merged configs to string");
                if matches.is_present("debug") { // If debug flag true, print to cli
                    println!("{}", yaml)
                    } else { writer(o, yaml.as_bytes()); }
        } else if extension_type == "json" {
            let json_doc = settings
                .try_into::<serde_json::Value>()
                .expect("Error in serializing merged configs");
            let json =
                serde_json::to_string(&json_doc).expect("Error in moving merged configs to string");
                if matches.is_present("debug") { // If debug flag true, print to cli
                    println!("{}", json)
                    } else { writer(o, json.as_bytes()); }
        }
    }
}
