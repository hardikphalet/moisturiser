use std::path::Path;
use std::{env, fs};
use crate::moist::hydrate::hydrate;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ApplicationContext {
    pub template_directory: String,
    pub entity_identifier: String,
    pub template_extension: String,
    pub template_to_package: std::collections::HashMap<String, String>,
    pub author: String,
    pub version: String
}

pub fn init() {
    // TODO Add loggers here
    let args: Vec<String> = env::args().collect();
    let context: ApplicationContext = load_config();

    parse(&args, &context);
}

fn parse(args: &Vec<String>, context: &ApplicationContext) {
    if !args[1].eq("hydrate") {
        println!("Invalid input format");
        return;
    }

    let input_dir = Path::new(&args[2]);
    let dir = fs::canonicalize(input_dir).expect("Path does not exist!");
    hydrate(dir.as_path(), context);
}

fn load_config() -> ApplicationContext {
    let config_file: &str = option_env!("moist_config").expect("$moist_config variable not set");
    println!("Moist Config location: {}", config_file);
    let contents = fs::read_to_string(config_file).expect("Config file can not be read.");
    println!("Content of config files are:\n{contents}");

    let application_context: ApplicationContext = serde_json::from_str(&contents).expect("Error parsing the config file");
    println!("Template Directory: {:?}", application_context.template_directory);
    return application_context;
}
