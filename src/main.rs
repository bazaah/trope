extern crate config;
extern crate serde_yaml;
#[macro_use]
extern crate clap;

use std::collections::HashMap;
use config::*;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Dynyaml")
                        .version(crate_version!())
                        .author(crate_authors!("/n"))
                        .about("Testing cross format config merging")
                        .arg(Arg::with_name("config")
                                    .short("c")
                                    .long("config")
                                    .value_name("FILE")
                                    .help("Opens a specified file")
                                    .takes_value(true)
                                    .multiple(true)
                                    .required(true))
                        .arg(Arg::with_name("debug")
                                    .short("d")
                                    .long("debug")
                                    .help("Displays contents of file from 'config'"))
                        .get_matches();

    let mut settings = config::Config::default();

    let in_iterator = matches.values_of("config");
    for f in in_iterator.unwrap() {
        settings
        .merge(config::File::with_name(f))
        .unwrap();
    }
    settings
        .merge(config::Environment::with_prefix("DYNYAML"))
        .unwrap();
    
    let yaml_doc = settings.try_into::<serde_yaml::Value>().unwrap();
    let yaml = serde_yaml::to_string(&yaml_doc).unwrap();
    let port = yaml_doc.get("port").unwrap();

    if matches.is_present("debug") {
    println!("{:#?}", yaml_doc);
    println!("{}", yaml);
    println!("{:#?}", port);
    }
}
