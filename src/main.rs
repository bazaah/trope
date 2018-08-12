extern crate config;
extern crate serde_yaml;
#[macro_use]
extern crate clap;

use std::fs::File;
use std::io::prelude::*;

use clap::{App, Arg};

fn yaml_writer(f: &[u8]) -> std::io::Result<()> {
    let mut file = File::create("yml_out.yml")?;
    file.write_all(f)?;
    Ok(())
}

fn main() {
    let matches = App::new("Dynyaml")
                        .version(crate_version!())
                        .author(crate_authors!("/n"))
                        .about("Utility for merging yaml files")
                        .arg(Arg::with_name("config")
                                    .short("c")
                                    .long("config")
                                    .value_name("FILE")
                                    .help("Specifies file(s) to merge")
                                    .takes_value(true)
                                    .multiple(true)
                                    .required(true))
                        .arg(Arg::with_name("env_var")
                                    .short("e")
                                    .long("env")
                                    .help("Merges enviromental variables with prefix: DYNYAML"))            
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

    if matches.is_present("env_var") {
    settings
        .merge(config::Environment::with_prefix("TROPE"))
        .unwrap();
    }

    let yaml_doc = settings.try_into::<serde_yaml::Value>().unwrap();
    let yaml = serde_yaml::to_string(&yaml_doc).unwrap();
    //let port = yaml_doc.get("port").unwrap();

    if matches.is_present("debug") {
    //println!("{:#?}", yaml_doc);
    println!("{}", yaml);
    //println!("{:#?}", port);
    }
    yaml_writer(yaml.as_bytes());

}
