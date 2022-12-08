use liquid::ParserBuilder;
use std::env;
use std::fs;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();  //TODO validate argument

    let source: String = fs::read_to_string(&args[1]).expect("unable to read the file");

    let template: liquid::Template = ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(source.as_str())
        .unwrap();
    let globals: liquid::Object = liquid::object!({
        "entity" : "Lot"
    });

    let modified_content: String = template.render(&globals).unwrap();

    let mut file: fs::File = fs::File::create("Lot.java").expect("Cannot create file");
    file.write_all(modified_content.clone().as_bytes()).expect("msg");
}
