// Copyright 2023 Hardik Phalet

use std::{fs, io};
use std::io::{BufReader, BufRead};
use std::path::Path;

use crate::moist::application::ApplicationContext;

fn find_files(dir: &Path) -> Result<Vec<String>, std::io::Error> {
    let mut files: Vec<String> = vec![];

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                files.append(&mut find_files(&path)?);
            } else if path.is_file() {
                files.push(path.to_str().unwrap().to_string());
            }
        }
    }

    Ok(files)
}

fn is_file_entity(file: &str, context: &ApplicationContext) -> bool {
    println!("file path for entity check: {}", file);
    if file.ends_with(".DS_Store") {
        return false;
    }
    let file: fs::File = fs::File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.contains(context.entity_identifier.as_str()) {
            return true;
        }
    }
    false
}

pub fn find_entity_files(dir: &Path, context: &ApplicationContext) -> Result<Vec<String>, std::io::Error> {
    let files: Vec<String> = find_files(dir)?;
    let mut entity_files: Vec<String> = vec![];

    for file in files {
        if is_file_entity(&file, context) {
            entity_files.push(file.to_string());
        }
    }

    Ok(entity_files)
}

pub fn find_template_files(dir: &Path, context: &ApplicationContext) -> Result<Vec<String>, std::io::Error> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == context.template_extension.as_str() {
                files.push(String::from(path.to_str().unwrap()));
            }
        }
    }

    Ok(files)
}

pub fn find_package_name(language: &String, file: &Path) -> Result<String, std::io::Error> {
    if language == "java" {
        return get_java_package_name(file);
    }

    return get_golang_module_name();
}

pub fn get_java_package_name(file: &Path) -> Result<String, std::io::Error> {
    let file = fs::File::open(file)?;
    let reader = io::BufReader::new(file);

    let mut package_name: String = String::from("");
    for line in reader
        .lines()
        .skip_while(|line| !line.as_ref().unwrap().starts_with("package ")) {
        let line = line?;
        if line.starts_with("package ") {
            package_name = line[8..line.len() - 1].to_string()
        }
    }
    package_name.truncate(package_name.len() - 9); // removing the '.entities'
    Ok(package_name)
}

pub fn get_golang_module_name() -> Result<String, std::io::Error> {
    let current_dir = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
    println!("Path for Go Lang module name function: {}", current_dir);
    let file = fs::File::open(current_dir + "/go.mod")?;
    let reader = io:: BufReader::new(file);

    let mut module_name: String = String::from("");
    for line in reader
        .lines()
        .skip_while(|line| !line.as_ref().unwrap().starts_with("module ")) {
        let line = line?;
        if line.starts_with("module ") {
            module_name = line[7..line.len()].to_string()
        }
    }

    println!("{}", module_name);
    Ok(module_name)
}
