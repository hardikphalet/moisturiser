use std::path::Path;
use super::file_util;

pub fn retrieve_globals_from_entity(entity_path: &Path) -> Result<liquid::Object, std::io::Error> {
    let mut file_name: String = String::from("");
    if let Some(stem) = entity_path.file_stem() {
        file_name = stem.to_string_lossy().to_string();
    }

    let globals: liquid::Object = liquid::object!({
        "entity": &file_name,
        "package_name": file_util::find_package_name(entity_path).unwrap(),
        "entity_name_small": &file_name.to_lowercase(),
    });
    Ok(globals)
}