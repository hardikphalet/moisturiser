use std::io::Write;
use std::path::Path;
use crate::moist::utils;
use liquid::{ParserBuilder};

const TEMPLATE_DIRECTORY: &'static str = "/Users/hardikphalet/dev_personal/rust/moisturiser/moisturiser/templates";

pub fn hydrate(dir_path: &Path) {
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
    let mut count = 0;
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
            let mut file: std::fs::File = std::fs::File::create(Path::new(((String::from(String::from("rendered_file_") + count.to_string().as_str())) + ".java").as_str())).expect("Cannot create the target file");
            file.write_all(modified_content.clone().as_bytes()).expect("Cannot write to the created file");
            count = count + 1;
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
