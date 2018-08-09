extern crate config;
extern crate serde_yaml;

fn main() {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();
    settings
        .merge(config::Environment::with_prefix("DYNYAML"))
        .unwrap();
    let yaml_doc = settings.try_into::<serde_yaml::Value>().unwrap();
    let yaml = serde_yaml::to_string(&yaml_doc).unwrap();
    let port = yaml_doc.get("port").unwrap();
    println!("{:#?}", yaml_doc);
    println!("{}", yaml);
    println!("{:#?}", port);
}
