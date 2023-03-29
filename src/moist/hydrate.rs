// Copyright 2023 Hardik Phalet

use std::{io::Write, env};
use std::path::Path;
use convert_case::{Case, Casing};
use crate::moist::utils;
use liquid::{ParserBuilder, ValueView};
use crate::moist::application::ApplicationContext;

pub fn hydrate(dir_path: &Path, context: &ApplicationContext) {
    if context.language.eq("java") {
        env::set_current_dir(dir_path).expect("Failed to set current directory");
    }
    let entity_files: Vec<String> = utils::file_util::find_entity_files(dir_path, context)
        .expect("Cannot find entity files");
    println!("Entity files found are:");
    for entity in &entity_files {
        println!("Entity:: {}", entity);
    }

    let templates: Vec<String> = fetch_templates(context).expect("Cannot find template files");
    println!("Templates files found are:");
    for template in &templates {
        println!("Template:: {}", template);
    }

    let mut rendering_globals: Vec<liquid::Object> = vec![];
    for entity in &entity_files {
        rendering_globals.push(utils::liquid_util::retrieve_globals_from_entity(&context.language, Path::new(entity)).unwrap());
    }

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
            let template_stem =  String::from(Path::new(template).file_stem().unwrap().to_str().unwrap());

            println!("Template Stem: {}", template_stem);
            let package_folder_name = context.template_to_package.get(&template_stem).expect("No such entity exists in the map").to_string();
            let mut save_path:String = get_save_path(&context.language);
            save_path.push_str(package_folder_name.as_str());
            save_path.push_str("/");
            save_path.push_str(&*get_file_name(&context.language, globals.get("entity").as_scalar().to_kstr().as_str().to_string()));
            if context.language == "java" {
                save_path.push_str(Path::new(&template).file_stem().unwrap().to_str().unwrap());
            }
            save_path.push_str(&(".".to_owned() + &context.language));



            if !Path::new(save_path.as_str()).parent().unwrap().exists() {
                std::fs::create_dir_all(Path::new(save_path.as_str()).parent().unwrap()).unwrap();
            }
            println!("{}", save_path);
            let file: Result<std::fs::File, std::io::Error> = std::fs::OpenOptions::new().write(true)
                                                                        .create_new(true)
                                                                        .open(save_path);
            match file {
                Ok(mut file) => {
                    // println!("{}", modified_content); // TODO REMOVE BEFORE COMMIT
                    file.write_all(modified_content.as_bytes()).expect("Cannot write to the created file");
                },
                Err(_) => {println!("File created already")},
            }
            
        }
    }
}

fn get_save_path(language: &String) -> String {
    if language == "java" {
        return String::from("../");
    }

    return String::from("./");
}

fn fetch_templates(context: &ApplicationContext) -> Result<Vec<String>, std::io::Error> {
    Ok(
        utils::file_util::find_template_files(Path::new(context.template_directory.as_str()), context)
            .expect("Cannot fetch templates")
    )
}

fn get_file_name(language: &String, entity: String) -> String {
    if language == "java" {
        return entity;
    }

    return entity.to_case(Case::Snake);
}