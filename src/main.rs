extern crate config;
extern crate serde_json;
extern crate serde_yaml;
#[macro_use]
extern crate clap;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use clap::{App, Arg};

fn writer(file_name: &str, output_data: &[u8]) {
    let mut file = File::create(file_name).unwrap();
    file.write_all(output_data).unwrap();
}

fn main() {
    let matches = App::new("Trope")
        .version(crate_version!())
        .author(crate_authors!("/n"))
        .about("Utility for merging YAML and JSON files")
        .arg(
            Arg::with_name("output")
                .short("O")
                .long("output")
                .help("Sets the path to the output file")
                .takes_value(true),
        ).arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
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
                .help("Displays contents of file(s) from 'config'"),
        ).get_matches();

    let mut settings = config::Config::default();

    let in_iterator = matches.values_of("config");
    for f in in_iterator.unwrap() {
        settings
            .merge(config::File::with_name(f))
            .expect("Error in merging given files");
    }

    if matches.is_present("env_var") {
        settings
            .merge(config::Environment::with_prefix("TROPE"))
            .expect("Error in merging given environmental variable");
    }

    if let Some(o) = matches.value_of("output") {
        let file_path = Path::new(o);
        let extension_type = match file_path.extension() {
            None => "",
            Some(os_str) => match os_str.to_str() {
                Some("yaml") => "yaml",
                Some("yml") => "yaml",
                Some("json") => "json",
                _ => panic!("Can't find extension type"),
            },
        };
        if extension_type == "yaml" {
            let yaml_doc = settings
                .try_into::<serde_yaml::Value>()
                .expect("Error in serializing merged configs");
            let yaml =
                serde_yaml::to_string(&yaml_doc).expect("Error in moving merged configs to string");
            writer(o, yaml.as_bytes());
        } else if extension_type == "json" {
            let json_doc = settings
                .try_into::<serde_json::Value>()
                .expect("Error in serializing merged configs");
            let json =
                serde_json::to_string(&json_doc).expect("Error in moving merged configs to string");
            writer(o, json.as_bytes());
        }
    }

    if matches.is_present("debug") {
        //println!("{:#?}", yaml_doc);
        //println!("{}", yaml);
        //println!("{:#?}", port);
    }
}
