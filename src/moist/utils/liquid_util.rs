// Copyright 2023 Hardik Phalet

use std::path::Path;
use convert_case::{Case, Casing};
use super::file_util;

pub fn retrieve_globals_from_entity(language: &String, entity_path: &Path) -> Result<liquid::Object, std::io::Error> {
    let mut file_name: String = String::from("");
    if let Some(stem) = entity_path.file_stem() {
        file_name = stem.to_string_lossy().to_string();
    }


    let globals: liquid::Object = liquid::object!({
        "entity": &get_file_name(language.to_string(), &file_name),
        "package_name": file_util::find_package_name(language, entity_path).unwrap(),
        "entity_name_small": &get_file_name_small(language.to_string(), &file_name),
    });
    Ok(globals)
}

pub fn get_file_name(language: String, file_name: &String) -> String {
    if language == "java" {
        return file_name.to_string();
    }

    return file_name.to_case(Case::Pascal);
}

pub fn get_file_name_small(language: String, file_name: &String) -> String {
    if language == "java" {
        return file_name.to_lowercase();
    }

    return file_name.to_case(Case::Camel)
}
