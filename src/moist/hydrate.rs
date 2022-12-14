use std::{io::Write, env};
use std::path::Path;
use crate::moist::utils;
use liquid::{ParserBuilder, ValueView};

const TEMPLATE_DIRECTORY: &'static str = "/Users/hardikphalet/dev_personal/rust/moisturiser/moisturiser/templates";

pub fn hydrate(dir_path: &Path) {
    env::set_current_dir(dir_path).expect("Failed to set current directory");
    let entity_files: Vec<String> = utils::file_util::find_entity_files(dir_path)
        .expect("Cannot find entity files");
    println!("Entity files found are:");
    for entity in &entity_files {
        println!("Entity:: {}", entity);
    }

    let templates: Vec<String> = fetch_templates().expect("Cannot find template files");
    println!("Templates files found are:");
    for template in &templates {
        println!("Template:: {}", template);
    }

    // now for every entity retrieve variables (entity, package_name)
    let mut rendering_globals: Vec<liquid::Object> = vec![];
    for entity in &entity_files {
        rendering_globals.push(utils::liquid_util::retrieve_globals_from_entity(Path::new(entity)).unwrap());
    }

    // TODO optimise this
    // low level strategy
    for globals in rendering_globals {
        for template in &templates {
            let template_source = std::fs::read_to_string(template)
                .expect("unable to read the template file");
            let curr_template: liquid::Template = ParserBuilder::with_stdlib()
                .build()
                .unwrap()
                .parse(template_source.as_str())
                .unwrap();
            let modified_content: String = curr_template.render(&globals).unwrap();
            let file: Result<std::fs::File, std::io::Error> = std::fs::OpenOptions::new().write(true)
                                                                        .create_new(true)
                                                                        .open(
                                                                            Path::new((String::from("../") +
                                                                             globals.get("entity").as_scalar().to_kstr().as_str() +
                                                                              Path::new(template).file_stem().unwrap().to_str().unwrap() + ".java").as_str())
                                                                        );
            match file {
                Ok(mut file) => {
                    file.write_all(modified_content.as_bytes()).expect("Cannot write to the created file");
                },
                Err(_) => {println!("File created already")},
            }
            
        }
    }
    // for every set of entity variables render files for all templates
}

fn fetch_templates() -> Result<Vec<String>, std::io::Error> {
    // [TODO] figure a way to find templates maybe saving a global variable
    Ok(
        utils::file_util::find_template_files(Path::new(TEMPLATE_DIRECTORY))
            .expect("Cannot fetch templates")
    )
}
